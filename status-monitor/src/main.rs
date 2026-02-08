use std::env;
use std::path::PathBuf;
use status_monitor::{run_monitor, MonitorConfig};
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let terraria_host = env::var("TERRARIA_HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let terraria_port = env::var("TERRARIA_PORT")
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(7777);

    let hytale_host = env::var("HYTALE_HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let hytale_port = env::var("HYTALE_PORT")
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(12345);

    let history_path = env::var("HISTORY_FILE_PATH")
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from("status-history.csv"));

    let config = MonitorConfig {
        terraria_host,
        terraria_port,
        hytale_host,
        hytale_port,
        history_path,
    };

    run_monitor(config).await
}
