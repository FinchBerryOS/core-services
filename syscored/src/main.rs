use nix::sys::signal::{SigSet, Signal};
use nix::sys::signalfd::SignalFd;
use std::collections::HashMap;
use std::env;
use std::os::unix::io::FromRawFd;
use std::os::unix::net::UnixStream;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

// Modul-Deklarationen (verbinden die anderen Dateien mit dem Projekt)
mod broker;
mod config;
mod service;
mod system;

use service::ServiceRegistry;

fn main() {
    let args: Vec<String> = env::args().collect();
    let is_user_mode = args.contains(&"--user".to_string());

    if is_user_mode {
        println!("[SYSCORED-USER] Starting in User Space Mode...");
    } else {
        println!("[SYSCORED-SYSTEM] FinchBerryOS Core Daemon booting...");
    }

    // 1. Sicheres Signal-Handling für PID 1
    let mut mask = SigSet::empty();
    mask.add(Signal::SIGCHLD);
    mask.add(Signal::SIGTERM);
    mask.thread_block().expect("Failed to block signals");
    let mut sig_fd = SignalFd::new(&mask).expect("Failed to create signalfd");

    // 2. System-Aufgaben (Nur als Root)
    if !is_user_mode {
        system::setup_infrastructure();
    }

    // 3. Adoptiere den System-LXPC-Bus auf FD 3 (Nur im User-Mode)
    // Das Unterstrich-Präfix verhindert eine Compiler-Warnung, da wir die Variable
    // hier nur festhalten, damit der Socket nicht geschlossen wird.
    let _system_bus = if is_user_mode {
        let fd3 = unsafe { UnixStream::from_raw_fd(3) };
        println!("[SYSCORED-USER] Successfully adopted System-LXPC bus on FD 3");
        Some(fd3)
    } else {
        None
    };

    // 4. Konfigurationen laden (.toml)
    let services: ServiceRegistry = Arc::new(Mutex::new(HashMap::new()));
    config::load_services(&services, is_user_mode);

    // 5. LXPC Broker in eigenem Thread starten
    let services_lxpc = Arc::clone(&services);
    thread::spawn(move || {
        broker::run_lxpc_broker(services_lxpc, is_user_mode);
    });

    // 6. Boot abschließen (Nur als Root)
    if !is_user_mode {
        system::confirm_boot_success();
    }

    let mode_str = if is_user_mode { "-USER" } else { "-SYSTEM" };
    println!("[SYSCORED{}] Entering Watchdog/Reaper Loop.", mode_str);

    // 7. Watchdog Loop
    loop {
        match sig_fd.read_signal() {
            Ok(Some(sig)) => {
                if sig.ssi_signo == Signal::SIGCHLD as u32 {
                    service::reap_zombies(&services);
                } else if sig.ssi_signo == Signal::SIGTERM as u32 {
                    println!("[SYSCORED{}] SIGTERM received. Shutting down...", mode_str);
                    break;
                }
            }
            Ok(None) => continue,
            Err(e) => {
                eprintln!("[SYSCORED{}] SignalFd error: {}", mode_str, e);
                thread::sleep(Duration::from_secs(1));
            }
        }
    }
}