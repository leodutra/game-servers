use axum::{
    extract::State,
    response::{sse::{Event, Sse}},
    routing::get,
    Router,
};
use shared::ServerStatus;
use gamedig::games::minecraft;
use std::{sync::Arc, time::Duration, fs, net::{TcpStream, ToSocketAddrs, SocketAddr}};
use tokio::{sync::broadcast, net::UdpSocket};
use futures::stream::Stream;
use tower_http::services::ServeDir;
use serde::Deserialize;

#[derive(Clone)]
struct AppState {
    tx: broadcast::Sender<ServerStatus>,
}

#[derive(Deserialize, Debug)]
struct Config {
    servers: Vec<ServerConfigItem>,
}

#[derive(Deserialize, Debug, Clone)]
struct ServerConfigItem {
    name: String,
    host: String,
    port: u16,
    #[serde(rename = "type")]
    server_type: String,
    #[serde(default)]
    protocol: Option<String>,
}

#[tokio::main]
async fn main() {
    // 1. Read Configuration
    let config_path = if fs::metadata("servers.toml").is_ok() {
        "servers.toml"
    } else if fs::metadata("../servers.toml").is_ok() {
        "../servers.toml"
    } else {
        panic!("Could not find servers.toml");
    };

    let config_str = fs::read_to_string(config_path).expect("Failed to read servers.toml");
    let config: Config = toml::from_str(&config_str).expect("Failed to parse servers.toml");
    println!("Loaded configuration for {} servers.", config.servers.len());

    // 2. Setup Broadcast Channel
    let (tx, _rx) = broadcast::channel(100);
    let app_state = Arc::new(AppState { tx: tx.clone() });

    // 3. Start the Monitor Loop
    let servers = Arc::new(config.servers);
    let tx_monitor = tx.clone();

    tokio::spawn(async move {
        loop {
            for server in servers.iter() {
                let s_config = server.clone();
                let tx_inner = tx_monitor.clone();

                tokio::spawn(async move {
                    check_server(s_config, tx_inner).await;
                });
            }
            tokio::time::sleep(Duration::from_secs(10)).await;
        }
    });

    // Determine dist path
    let dist_path = if fs::metadata("frontend/dist").is_ok() {
        "frontend/dist"
    } else {
        "../frontend/dist"
    };

    // 4. Serve Frontend & API
    let app = Router::new()
        .route("/api/events", get(sse_handler))
        .nest_service("/", ServeDir::new(dist_path))
        .with_state(app_state);

    println!("Sentinel running on http://0.0.0.0:3000");
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn check_server(config: ServerConfigItem, tx: broadcast::Sender<ServerStatus>) {
    let now = chrono::Local::now().format("%H:%M:%S").to_string();

    // Resolve Hostname to IP (Blocking DNS)
    let addr_str = format!("{}:{}", config.host, config.port);
    let socket_addr = addr_str.to_socket_addrs().ok().and_then(|mut iter| iter.next());

    let status = match config.server_type.as_str() {
        "Terraria" => {
            if let Some(addr) = socket_addr {
                 match TcpStream::connect_timeout(&addr, Duration::from_secs(2)) {
                    Ok(_) => ServerStatus {
                        name: config.name.clone(),
                        server_type: "Terraria".to_string(),
                        online: true,
                        players: "?/?".to_string(),
                        ping: 0,
                        details: "Online (TCP)".to_string(),
                        last_updated: now,
                    },
                    Err(e) => offline_status(config.name, "Terraria", e.to_string(), now),
                }
            } else {
                 offline_status(config.name, "Terraria", "DNS Resolution Failed".to_string(), now)
            }
        },
        "Minecraft" => {
             if let Some(addr) = socket_addr {
                 match minecraft::query(&addr.ip(), Some(config.port)) {
                    Ok(response) => ServerStatus {
                        name: config.name.clone(),
                        server_type: "Minecraft".to_string(),
                        online: true,
                        players: format!("{}/{}", response.players_online, response.players_maximum),
                        ping: 0,
                        details: "Online".to_string(),
                        last_updated: now,
                    },
                     Err(e) => offline_status(config.name, "Minecraft", e.to_string(), now),
                 }
             } else {
                 offline_status(config.name, "Minecraft", "DNS Resolution Failed".to_string(), now)
             }
        },
        _ => {
            // Generic fallback or unknown
             let proto = config.protocol.as_deref().unwrap_or("tcp").to_lowercase();
             if let Some(addr) = socket_addr {
                 if proto == "udp" {
                     match udp_probe(addr, Duration::from_secs(2)).await {
                        Ok(()) => ServerStatus {
                            name: config.name.clone(),
                            server_type: config.server_type.clone(),
                            online: true,
                            players: "-/-".to_string(),
                            ping: 0,
                            details: "Online (UDP)".to_string(),
                            last_updated: now,
                        },
                        Err(e) => offline_status(config.name, &config.server_type, e, now),
                    }
                 } else {
                     // Assume TCP
                     match TcpStream::connect_timeout(&addr, Duration::from_secs(2)) {
                        Ok(_) => ServerStatus {
                            name: config.name.clone(),
                            server_type: config.server_type.clone(),
                            online: true,
                            players: "-/-".to_string(),
                            ping: 0,
                            details: "Online (TCP)".to_string(),
                            last_updated: now,
                        },
                        Err(e) => offline_status(config.name, &config.server_type, e.to_string(), now),
                    }
                 }
             } else {
                  offline_status(config.name, &config.server_type, "DNS Resolution Failed".to_string(), now)
             }
        }
    };

    let _ = tx.send(status);
}

fn offline_status(name: String, s_type: &str, error: String, time: String) -> ServerStatus {
    ServerStatus {
        name,
        server_type: s_type.to_string(),
        online: false,
        players: "-/-".to_string(),
        ping: 0,
        details: format!("Offline: {}", error),
        last_updated: time,
    }
}

async fn udp_probe(addr: SocketAddr, timeout: Duration) -> Result<(), String> {
    let bind_addr = if addr.is_ipv6() { "[::]:0" } else { "0.0.0.0:0" };
    let socket = UdpSocket::bind(bind_addr).await.map_err(|e| e.to_string())?;
    socket.connect(addr).await.map_err(|e| e.to_string())?;

    // Send a single-byte probe. Some servers respond with a banner/ping reply.
    // NOTE: Many game servers (Hytale, Minecraft Bedrock, etc) ignore empty or garbled UDP packets.
    // Sending a specific payload might be required for Hytale, but a generic "ping" (0xFE or similar) usually works better
    // than a zero byte for games.
    // For now, let's try sending a common query byte (0xFE) which is used in some protocols as a ping.
    socket.send(&[0xFE]).await.map_err(|e| e.to_string())?;

    let mut buf = [0u8; 1024]; // Increase buffer size to capture banner
    match tokio::time::timeout(timeout, socket.recv(&mut buf)).await {
        Ok(Ok(_)) => Ok(()),
        Ok(Err(e)) => Err(e.to_string()),
        Err(_) => Err("No UDP response".to_string()),
    }
}

async fn sse_handler(State(state): State<Arc<AppState>>) -> Sse<impl Stream<Item = Result<Event, axum::Error>>> {
    let mut rx = state.tx.subscribe();
    let stream = async_stream::stream! {
        loop {
            match rx.recv().await {
                Ok(status) => {
                    if let Ok(json) = serde_json::to_string(&status) {
                        yield Ok(Event::default().data(json));
                    }
                },
                Err(broadcast::error::RecvError::Lagged(missed)) => {
                    // Log or handle lag if critical, but we continue to get next updates
                    eprintln!("Client lagged, missed {} messages", missed);
                    continue;
                },
                Err(broadcast::error::RecvError::Closed) => {
                    break;
                },
            }
        }
    };
    Sse::new(stream).keep_alive(axum::response::sse::KeepAlive::new().text("keep-alive"))
}
