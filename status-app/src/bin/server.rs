#[cfg(not(target_arch = "wasm32"))]
mod server_impl {
    use std::env;
    use std::path::PathBuf;

    use std::time::Duration;
    use anyhow::Result;
    use axum::{
        extract::State,
        response::sse::{Event, KeepAlive, Sse},
        routing::get,
        Json,
        Router,
    };
    use futures::stream::Stream;
    use notify::{Config, RecommendedWatcher, RecursiveMode, Watcher};
    use status_monitor::{run_monitor, MonitorConfig};
    use status_app::models::PublicConfig;
    use tokio::sync::watch;
    use tokio_stream::wrappers::WatchStream;
    use tokio_stream::StreamExt;
    use tower_http::services::ServeDir;
    use tower_http::cors::CorsLayer;

    pub async fn main() -> Result<()> {
        let config = load_config();

        // Prepare public config
        let public_config = PublicConfig {
            terraria: format!("{}:{}", config.terraria_host, config.terraria_port),
            hytale: format!("{}:{}", config.hytale_host, config.hytale_port),
        };

        // Start background tasks
        spawn_monitor(config.clone());
        let rx = spawn_file_watcher(config.history_path.clone()).await;

        // Setup Axum Server
        let app = Router::new()
            .route("/api/sse", get(sse_handler))
            .route("/api/config", get(move || async move { Json(public_config) }))
            .with_state(rx)
            .fallback_service(ServeDir::new("dist"))
            .layer(CorsLayer::permissive());

        let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
        println!("Server listening on {}", listener.local_addr().unwrap());
        axum::serve(listener, app).await.unwrap();

        Ok(())
    }

    fn load_config() -> MonitorConfig {
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

        MonitorConfig {
            terraria_host,
            terraria_port,
            hytale_host,
            hytale_port,
            history_path,
        }
    }

    fn spawn_monitor(config: MonitorConfig) {
        tokio::spawn(async move {
            loop {
                println!("Starting Status Monitor daemon...");
                let config_clone = config.clone();
                match run_monitor(config_clone).await {
                    Ok(_) => eprintln!("Status Monitor exited unexpectedly. Restarting in 5 seconds..."),
                    Err(e) => eprintln!("Status Monitor crashed: {:?}. Restarting in 5 seconds...", e),
                }
                tokio::time::sleep(Duration::from_secs(5)).await;
            }
        });
    }

    async fn spawn_file_watcher(file_path: PathBuf) -> watch::Receiver<String> {
        let initial_content = tokio::fs::read_to_string(&file_path).await.unwrap_or_else(|_| "{}".to_string());
        let (tx, rx) = watch::channel(initial_content);
        let watch_path = file_path.clone();

        tokio::spawn(async move {
            let (notify_tx, mut notify_rx) = tokio::sync::mpsc::channel(1);

            let mut watcher = RecommendedWatcher::new(move |res| {
                let _ = notify_tx.blocking_send(res);
            }, Config::default()).unwrap();

            let parent = watch_path.parent().unwrap_or(&watch_path);
            if let Err(e) = watcher.watch(parent, RecursiveMode::NonRecursive) {
                eprintln!("Failed to watch directory: {:?}", e);
            }

            while let Some(res) = notify_rx.recv().await {
                match res {
                    Ok(event) => {
                        let relevant = event.paths.iter().any(|p| p.ends_with(watch_path.file_name().unwrap()));
                        if relevant && event.kind.is_modify() {
                             tokio::time::sleep(Duration::from_millis(100)).await;
                             if let Ok(content) = tokio::fs::read_to_string(&watch_path).await {
                                 // Check if content actually changed to avoid spurious updates
                                 if *tx.borrow() != content {
                                     let _ = tx.send(content);
                                 }
                             }
                        }
                    }
                    Err(e) => eprintln!("Watch error: {:?}", e),
                }
            }
        });

        rx
    }

    async fn sse_handler(
        State(rx): State<watch::Receiver<String>>,
    ) -> Sse<impl Stream<Item = Result<Event, axum::Error>>> {
        let stream = WatchStream::new(rx);

        let stream = stream.map(|data| {
            Event::default().data(data)
        })
        .map(Ok);

        Sse::new(stream).keep_alive(KeepAlive::default())
    }
}

#[cfg(not(target_arch = "wasm32"))]
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    server_impl::main().await
}

#[cfg(target_arch = "wasm32")]
fn main() {
    panic!("Server binary not supported on WASM");
}
