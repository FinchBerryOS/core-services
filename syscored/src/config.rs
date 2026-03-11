use crate::service::{Service, ServiceRegistry};
use serde::Deserialize;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Deserialize, Debug)]
struct DaemonConfig {
    service: DaemonSettings,
}

#[derive(Deserialize, Debug)]
struct DaemonSettings {
    name: String,
    bin_path: String,
}

pub fn load_services(registry: &ServiceRegistry, is_user_mode: bool) {
    let daemon_dirs = if is_user_mode {
        vec!["/System/Library/LaunchAgents", "/Library/LaunchAgents"]
    } else {
        vec!["/System/Library/LaunchDaemons", "/Library/LaunchDaemons"]
    };

    println!("[SYSCORED] Scanning directories: {:?}", daemon_dirs);

    let mut lock = registry.lock().unwrap();

    for dir in daemon_dirs.iter() {
        let path = Path::new(dir);
        if !path.exists() {
            continue;
        }

        if let Ok(entries) = fs::read_dir(path) {
            for entry in entries.flatten() {
                let file_path = entry.path();
                if file_path.extension().and_then(|s| s.to_str()) == Some("toml") {
                    match fs::read_to_string(&file_path) {
                        Ok(content) => match toml::from_str::<DaemonConfig>(&content) {
                            Ok(config) => {
                                let name = config.service.name.clone();
                                lock.insert(
                                    name.clone(),
                                    Service {
                                        name: config.service.name,
                                        bin_path: PathBuf::from(config.service.bin_path),
                                        pid: None,
                                        control_socket: None,
                                    },
                                );
                                println!("[SYSCORED] Loaded: {}", name);
                            }
                            Err(e) => eprintln!("[SYSCORED] Error parsing {:?}: {}", file_path, e),
                        },
                        Err(e) => eprintln!("[SYSCORED] Cannot read {:?}: {}", file_path, e),
                    }
                }
            }
        }
    }
}