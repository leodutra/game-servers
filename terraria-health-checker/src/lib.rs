use std::net::{TcpStream, ToSocketAddrs};
use std::time::{Duration, Instant};
use std::io;

pub struct ServerInfo {
    pub is_online: bool,
    pub latency_ms: u128,
}

/// Checks if a Terraria server is reachable via TCP.
///
/// # Arguments
/// * `host` - The IP address or hostname of the server.
/// * `port` - The port number (usually 7777).
///
/// # Returns
/// * `Ok(ServerInfo)` if the check completed (successfully connected or refused).
/// * `Err(e)` if there was a DNS resolution error.
pub fn check_server_status(host: &str, port: u16) -> io::Result<ServerInfo> {
    // specific string formatting for the socket address
    let address = format!("{}:{}", host, port);

    // Set a timeout so the CLI doesn't hang if the server is down
    let timeout = Duration::from_secs(3);

    // Resolve the address first
    let socket_addrs = address.to_socket_addrs()?;

    // Attempt to connect to any resolved IP
    for addr in socket_addrs {
        let start = Instant::now();
        if TcpStream::connect_timeout(&addr, timeout).is_ok() {
            let latency = start.elapsed().as_millis();
            return Ok(ServerInfo {
                is_online: true,
                latency_ms: latency,
            });
        }
    }

    Ok(ServerInfo {
        is_online: false,
        latency_ms: 0,
    })
}
