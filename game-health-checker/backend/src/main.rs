use axum::{
    extract::State,
    response::{sse::{Event, Sse}},
    routing::get,
    Router,
};
use shared::ServerStatus;
use gamedig::games::minecraft;
use std::{sync::Arc, time::Duration, fs, net::TcpStream, str::FromStr};
use tokio::sync::broadcast;
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
    _protocol: Option<String>,
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

    let status = match config.server_type.as_str() {
        "Terraria" => {
            // Simple TCP Check for Terraria (TShock support removed from Gamedig)
            // Parse host IP
            let addr_str = format!("{}:{}", config.host, config.port);
            match std::net::SocketAddr::from_str(&addr_str) {
                Ok(addr) => {
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
                },
                 Err(_) => offline_status(config.name, "Terraria", "Invalid IP".to_string(), now),
            }
        },
        "Minecraft" => {
             match minecraft::query(&config.host.parse().unwrap(), Some(config.port)) {
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
        },
        _ => {
            // Generic fallback or unknown
             ServerStatus {
                name: config.name.clone(),
                server_type: config.server_type.clone(),
                online: false, // Implement generic TCP/UDP ping if needed
                players: "-/-".to_string(),
                ping: 0,
                details: "Unsupported type".to_string(),
                last_updated: now,
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

async fn sse_handler(State(state): State<Arc<AppState>>) -> Sse<impl Stream<Item = Result<Event, axum::Error>>> {
    let mut rx = state.tx.subscribe();
    let stream = async_stream::stream! {
        loop {
            if let Ok(status) = rx.recv().await
                && let Ok(json) = serde_json::to_string(&status) {
                    yield Ok(Event::default().data(json));
                }
        }
    };
    Sse::new(stream).keep_alive(axum::response::sse::KeepAlive::default())
}
