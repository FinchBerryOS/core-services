use crate::service::{spawn_service, ServiceRegistry};
use lxpc::{decode_header, LxpcObject};
use nix::sys::socket::{sendmsg, ControlMessage, MsgFlags};
use std::fs;
use std::io::{IoSlice, Read};
use std::os::unix::io::AsRawFd;
use std::os::unix::net::{UnixListener, UnixStream};
use std::sync::Arc;
use std::thread;
use tracing::{debug, error, info};

pub fn run_lxpc_broker(services: ServiceRegistry, is_user_mode: bool) {
    let socket_path = if is_user_mode {
        "/tmp/lxpc_user.sock" // In Produktion z.B. /run/user/1000/lxpc.sock
    } else {
        "/run/lxpc.sock"
    };

    let _ = fs::remove_file(socket_path);

    let listener = UnixListener::bind(socket_path).expect("Failed to bind LXPC socket");
    println!("[SYSCORED] LXPC Broker listening on {}", socket_path);

    for stream in listener.incoming() {
        match stream {
            Ok(client) => {
                let services_clone = Arc::clone(&services);
                thread::spawn(move || {
                    handle_client(client, services_clone);
                });
            }
            Err(e) => eprintln!("[LXPC] Connection failed: {}", e),
        }
    }
}

fn handle_client(mut client_stream: UnixStream, services: ServiceRegistry) {
    let client_fd = client_stream.as_raw_fd();

    let target_service = match read_routing_target(&mut client_stream) {
        Ok(name) => name,
        Err(e) => {
            error!("Handshake fehlgeschlagen: {}", e);
            return;
        }
    };

    let mut lock = services.lock().unwrap();
    let service = match lock.get_mut(&target_service) {
        Some(s) => s,
        None => {
            error!("Dienst {} unbekannt", target_service);
            return;
        }
    };

    // On-Demand Start, falls er nicht existiert
    if service.control_socket.is_none() {
        info!("Starte Dienst: {}", target_service);
        match spawn_service(&service.bin_path) {
            Ok((master_socket, pid)) => {
                service.control_socket = Some(master_socket);
                service.pid = Some(pid);
            }
            Err(e) => {
                error!("Konnte Dienst nicht starten: {}", e);
                return;
            }
        }
    }

    // Client via SCM_RIGHTS an den Dienst weiterreichen
    let control_fd = service.control_socket.as_ref().unwrap().as_raw_fd();
    let cmsgs = [ControlMessage::ScmRights(&[client_fd])];
    let iov = [IoSlice::new(&[0u8; 1])];

    if let Err(e) = sendmsg::<()>(control_fd, &iov, &cmsgs, MsgFlags::empty(), None) {
        error!("Konnte Client-FD nicht an Dienst delegieren: {}", e);
    } else {
        debug!("Client-FD erfolgreich an {} übergeben", target_service);
    }
}

fn read_routing_target(stream: &mut UnixStream) -> Result<String, Box<dyn std::error::Error>> {
    let mut header = [0u8; 12];
    stream.read_exact(&mut header)?;
    
    // Annahme: Dein lxpc crate validiert hier LXPC_MAGIC
    let len = decode_header(&header.try_into().unwrap())?;

    let mut payload = vec![0u8; len as usize];
    stream.read_exact(&mut payload)?;

    let obj: LxpcObject = ciborium::from_reader(&payload[..])?;
    obj.get_string("lxpc_bootstrap_target")
        .map(|s| s.to_string())
        .ok_or("Kein Ziel angegeben".into())
}