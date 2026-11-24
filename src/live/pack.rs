//! The world's resource pack for players of the live server. A server ignores the world's
//! `resources.zip`, so the server tells each player to fetch it from here instead.

use std::io::{Read, Write};
use std::net::{Ipv4Addr, TcpListener, TcpStream};
use std::path::Path;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread::JoinHandle;
use std::time::Duration;

pub struct PackServer {
    pub url: String,
    pub sha1: String,
    addr: std::net::SocketAddr,
    stop: Arc<AtomicBool>,
    thread: Option<JoinHandle<()>>,
}

/// Serves `zip`, if the world has one. Players on the network can fetch it too when `lan`.
pub fn serve(zip: &Path, lan: bool) -> Result<Option<PackServer>, String> {
    let bytes = match std::fs::read(zip) {
        Ok(b) => b,
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => return Ok(None),
        Err(e) => return Err(format!("Cannot read {}: {e}", zip.display())),
    };
    let (bind, host) = match lan.then(super::lan::lan_ip).flatten() {
        Some(ip) => (Ipv4Addr::UNSPECIFIED, ip),
        None => {
            if lan {
                eprintln!("[live] No network address found, so only this computer gets the world's resource pack.");
            }
            (Ipv4Addr::LOCALHOST, Ipv4Addr::LOCALHOST)
        }
    };
    let listener = TcpListener::bind((bind, 0)).map_err(|e| e.to_string())?;
    let addr = listener.local_addr().map_err(|e| e.to_string())?;
    let sha1 = sha1_smol::Sha1::from(bytes.as_slice()).digest().to_string();
    let stop = Arc::new(AtomicBool::new(false));
    let thread = {
        let stop = stop.clone();
        std::thread::spawn(move || {
            for stream in listener.incoming() {
                if stop.load(Ordering::Acquire) {
                    break;
                }
                // One at a time: each player fetches it once, and nobody can pile up threads.
                if let Ok(s) = stream {
                    answer(s, &bytes);
                }
            }
        })
    };
    Ok(Some(PackServer {
        // The hash in the path makes the game fetch a changed pack instead of its cached one.
        url: format!("http://{host}:{}/{sha1}/resources.zip", addr.port()),
        sha1,
        addr,
        stop,
        thread: Some(thread),
    }))
}

/// Answers any GET with the pack; the game asks for nothing else.
fn answer(mut s: TcpStream, pack: &[u8]) {
    let _ = s.set_read_timeout(Some(Duration::from_secs(10)));
    let _ = s.set_write_timeout(Some(Duration::from_secs(30)));
    let mut head = Vec::new();
    let mut buf = [0u8; 1024];
    while !head.windows(4).any(|w| w == b"\r\n\r\n") && head.len() < 16 * 1024 {
        match s.read(&mut buf) {
            Ok(0) | Err(_) => return,
            Ok(n) => head.extend_from_slice(&buf[..n]),
        }
    }
    let reply = if head.starts_with(b"GET ") {
        format!(
            "HTTP/1.1 200 OK\r\nContent-Type: application/zip\r\nContent-Length: {}\r\nConnection: close\r\n\r\n",
            pack.len()
        )
    } else {
        "HTTP/1.1 405 Method Not Allowed\r\nContent-Length: 0\r\nConnection: close\r\n\r\n"
            .to_string()
    };
    if s.write_all(reply.as_bytes()).is_ok() && head.starts_with(b"GET ") {
        let _ = s.write_all(pack);
    }
}

impl Drop for PackServer {
    fn drop(&mut self) {
        self.stop.store(true, Ordering::Release);
        // Wakes the accept loop so it sees the stop.
        let _ = TcpStream::connect((Ipv4Addr::LOCALHOST, self.addr.port()));
        if let Some(t) = self.thread.take() {
            let _ = t.join();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_pack_is_served_with_its_hash_and_a_world_without_one_serves_nothing() {
        let dir = tempfile::tempdir().unwrap();
        let zip = dir.path().join("resources.zip");
        assert!(serve(&zip, false).unwrap().is_none());

        std::fs::write(&zip, b"PK pack bytes").unwrap();
        let server = serve(&zip, false).unwrap().unwrap();
        assert_eq!(
            server.sha1,
            sha1_smol::Sha1::from(b"PK pack bytes").digest().to_string()
        );
        let url = server.url.strip_prefix("http://").unwrap();
        let (host, path) = url.split_once('/').unwrap();
        let mut s = TcpStream::connect(host).unwrap();
        write!(s, "GET /{path} HTTP/1.1\r\nHost: {host}\r\n\r\n").unwrap();
        let mut got = Vec::new();
        s.read_to_end(&mut got).unwrap();
        let got = String::from_utf8(got).unwrap();
        assert!(got.starts_with("HTTP/1.1 200 OK"), "{got}");
        assert!(got.ends_with("\r\n\r\nPK pack bytes"), "{got}");
        drop(server);
    }
}
