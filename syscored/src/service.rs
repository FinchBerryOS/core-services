use nix::sys::socket::{socketpair, AddressFamily, MsgFlags, SockType};
use nix::sys::wait::{waitpid, WaitPidFlag, WaitStatus};
use nix::unistd::{dup2, execvp, fork, ForkResult, Pid};
use std::collections::HashMap;
use std::os::unix::io::{AsRawFd, FromRawFd, IntoRawFd};
use std::os::unix::net::UnixStream;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

#[derive(Debug)]
pub struct Service {
    pub name: String,
    pub bin_path: PathBuf,
    pub pid: Option<Pid>,
    pub control_socket: Option<UnixStream>,
}

pub type ServiceRegistry = Arc<Mutex<HashMap<String, Service>>>;

pub fn reap_zombies(registry: &ServiceRegistry) {
    loop {
        // WNOHANG verhindert, dass syscored einfriert
        match waitpid(Pid::from_raw(-1), Some(WaitPidFlag::WNOHANG)) {
            Ok(WaitStatus::Exited(pid, status)) => {
                println!("[SYSCORED] Process {} exited with status {}", pid, status);
                handle_dead_service(registry, pid);
            }
            Ok(WaitStatus::Signaled(pid, signal, _)) => {
                println!("[SYSCORED] Process {} killed by signal {:?}", pid, signal);
                handle_dead_service(registry, pid);
            }
            Ok(WaitStatus::StillAlive) | Err(nix::errno::Errno::ECHILD) => break,
            _ => break,
        }
    }
}

fn handle_dead_service(registry: &ServiceRegistry, dead_pid: Pid) {
    let mut lock = registry.lock().unwrap();
    for service in lock.values_mut() {
        if service.pid == Some(dead_pid) {
            println!("[SYSCORED] WARNING: Service '{}' crashed/exited!", service.name);
            service.pid = None;
            service.control_socket = None; // Trigger für LXPC On-Demand Restart
            break;
        }
    }
}

pub fn spawn_service(bin: &PathBuf) -> Result<(UnixStream, Pid), Box<dyn std::error::Error>> {
    let (master, slave) = socketpair(
        AddressFamily::Unix,
        SockType::Stream,
        None,
        MsgFlags::empty(),
    )?;

    match unsafe { fork() }? {
        ForkResult::Parent { child, .. } => {
            drop(slave); // Der Parent braucht den Slave nicht
            Ok((unsafe { UnixStream::from_raw_fd(master.into_raw_fd()) }, child))
        }
        ForkResult::Child => {
            drop(master); // Das Child braucht den Master nicht
            
            dup2(slave.as_raw_fd(), 3).expect("dup2 failed");
            drop(slave); // Alten FD schließen
            
            let c_str = std::ffi::CString::new(bin.to_str().unwrap())?;
            execvp(&c_str, &[c_str.clone()]).expect("execvp failed");
            
            unreachable!("Exec fehlgeschlagen!");
        }
    }
}