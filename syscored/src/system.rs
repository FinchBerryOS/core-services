use std::fs;
use std::process::Command;

pub fn setup_infrastructure() {
    println!("[SYSCORED] Setting up infrastructure...");
    
    // Netzwerk Loopback hochfahren
    let _ = Command::new("ip").args(&["link", "set", "lo", "up"]).status();

    // Hostname auslesen oder Fallback setzen
    if let Ok(hostname) = fs::read_to_string("/private/etc/hostname") {
        let _ = Command::new("hostname").arg(hostname.trim()).status();
    } else {
        let _ = Command::new("hostname").arg("finchberry").status();
    }

    // Sticky-Bit für sichere Temporärdateien
    let _ = Command::new("chmod").args(&["1777", "/tmp"]).status();
}

pub fn confirm_boot_success() {
    let flag_path = "/private/system/boot_successful";
    let _ = fs::write(flag_path, "1");
    println!("[SYSCORED] Boot success flag written.");
}