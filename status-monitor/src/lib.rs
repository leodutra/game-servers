#[cfg(not(target_arch = "wasm32"))]
use std::path::{Path, PathBuf};
#[cfg(not(target_arch = "wasm32"))]
use std::time::Duration;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
#[cfg(not(target_arch = "wasm32"))]
use anyhow::Result;

#[cfg(not(target_arch = "wasm32"))]
use tokio::fs::{self, File};
#[cfg(not(target_arch = "wasm32"))]
use tokio::io::AsyncWriteExt;

pub const MAX_HISTORY_ENTRIES: usize = 1440;
pub const CHECK_INTERVAL_SECS: u64 = 60;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ServiceStatus {
    pub service_name: String,
    pub is_online: Option<bool>,
    pub latency_ms: u128,
    pub timestamp: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
pub struct History {
    pub terraria: Vec<ServiceStatus>,
    pub hytale: Vec<ServiceStatus>,
}

#[derive(Clone, Debug)]
#[cfg(not(target_arch = "wasm32"))]
pub struct MonitorConfig {
    pub terraria_host: String,
    pub terraria_port: u16,
    pub hytale_host: String,
    pub hytale_port: u16,
    pub history_path: PathBuf,
}

#[cfg(not(target_arch = "wasm32"))]
pub async fn run_monitor(config: MonitorConfig) -> Result<()> {
    println!("Starting Status Monitor...");
    println!("Monitoring Terraria at {}:{}", config.terraria_host, config.terraria_port);
    println!("Monitoring Hytale at {}:{}", config.hytale_host, config.hytale_port);
    println!("History file: {:?}", config.history_path);

    loop {
        let now = Utc::now();
        println!("Running checks at {}", now);

        // Check Terraria (blocking)
        let t_host = config.terraria_host.clone();
        let t_port = config.terraria_port;
        let terraria_result = tokio::task::spawn_blocking(move || {
            terraria_health_checker::check_server_status(&t_host, t_port)
        }).await;

        let terraria_entry = match terraria_result {
            Ok(Ok(info)) => ServiceStatus {
                service_name: "Terraria".to_string(),
                is_online: Some(info.is_online),
                latency_ms: info.latency_ms,
                timestamp: now,
            },
            Ok(Err(e)) => {
                eprintln!("Terraria monitor error: {:?}", e);
                 ServiceStatus {
                    service_name: "Terraria".to_string(),
                    is_online: None, // DNS or other error -> No Data
                    latency_ms: 0,
                    timestamp: now,
                }
            },
            Err(e) => {
                eprintln!("Terraria task error: {:?}", e);
                ServiceStatus {
                    service_name: "Terraria".to_string(),
                    is_online: None,
                    latency_ms: 0,
                    timestamp: now,
                }
            },
        };

        // Check Hytale (async)
        let hytale_result = hytale_health_checker::check_hytale_status(&config.hytale_host, config.hytale_port).await;
        let hytale_entry = match hytale_result {
            Ok(info) => ServiceStatus {
                service_name: "Hytale".to_string(),
                is_online: Some(info.is_online),
                latency_ms: info.latency_ms,
                timestamp: now,
            },
            Err(e) => {
                let err_str = e.to_string();
                let is_timeout = err_str.contains("Timeout");
                if is_timeout {
                     ServiceStatus {
                        service_name: "Hytale".to_string(),
                        is_online: Some(false),
                        latency_ms: 0,
                        timestamp: now,
                    }
                } else {
                    eprintln!("Hytale monitor error: {:?}", e);
                    ServiceStatus {
                        service_name: "Hytale".to_string(),
                        is_online: None, // No Data
                        latency_ms: 0,
                        timestamp: now,
                    }
                }
            },
        };

        // Update History
        update_history(&config.history_path, terraria_entry, hytale_entry).await?;

        tokio::time::sleep(Duration::from_secs(CHECK_INTERVAL_SECS)).await;
    }
}

#[cfg(not(target_arch = "wasm32"))]
async fn update_history(path: &Path, terraria: ServiceStatus, hytale: ServiceStatus) -> Result<()> {
    let mut history = load_history(path).await.unwrap_or_default();

    history.terraria.push(terraria);
    if history.terraria.len() > MAX_HISTORY_ENTRIES {
        history.terraria.remove(0);
    }

    history.hytale.push(hytale);
    if history.hytale.len() > MAX_HISTORY_ENTRIES {
        history.hytale.remove(0);
    }

    save_history(path, &history).await?;
    Ok(())
}

#[cfg(not(target_arch = "wasm32"))]
async fn load_history(path: &Path) -> Result<History> {
    if path.exists() {
        let content = fs::read_to_string(path).await?;
        let mut history = History::default();

        for (i, line) in content.lines().enumerate() {
            if i == 0 { continue; } // Skip header
            if line.trim().is_empty() { continue; }

            let parts: Vec<&str> = line.split(',').collect();
            if parts.len() < 5 { continue; }

            let timestamp = DateTime::parse_from_rfc3339(parts[0])?.with_timezone(&Utc);

            let t_online: Option<bool> = match parts[1].trim() {
                "true" => Some(true),
                "false" => Some(false),
                _ => None,
            };
            let t_latency: u128 = parts[2].parse().unwrap_or(0);

            let h_online: Option<bool> = match parts[3].trim() {
                "true" => Some(true),
                "false" => Some(false),
                _ => None,
            };
            let h_latency: u128 = parts[4].parse().unwrap_or(0);

            history.terraria.push(ServiceStatus {
                service_name: "Terraria".to_string(),
                is_online: t_online,
                latency_ms: t_latency,
                timestamp,
            });

            history.hytale.push(ServiceStatus {
                service_name: "Hytale".to_string(),
                is_online: h_online,
                latency_ms: h_latency,
                timestamp,
            });
        }
        return Ok(history);
    }
    Ok(History::default())
}

#[cfg(not(target_arch = "wasm32"))]
async fn save_history(path: &Path, history: &History) -> Result<()> {
    // Ensure directory exists
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).await?;
    }
    let mut file = File::create(path).await?;
    file.write_all(b"Timestamp,TerrariaOnline,TerrariaLatency,HytaleOnline,HytaleLatency\n").await?;

    let len = std::cmp::min(history.terraria.len(), history.hytale.len());
    for i in 0..len {
        let t = &history.terraria[i];
        let h = &history.hytale[i];

        let t_online_str = match t.is_online {
            Some(true) => "true",
            Some(false) => "false",
            None => "",
        };
        let h_online_str = match h.is_online {
            Some(true) => "true",
            Some(false) => "false",
            None => "",
        };

        let line = format!("{},{},{},{},{}\n",
            t.timestamp.to_rfc3339(),
            t_online_str,
            t.latency_ms,
            h_online_str,
            h.latency_ms
        );
        file.write_all(line.as_bytes()).await?;
    }
    Ok(())
}
