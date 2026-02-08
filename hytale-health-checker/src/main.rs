use clap::Parser;
use hytale_health_checker::check_hytale_status;
use std::time::Duration;
use chrono::Local;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Server address
    #[arg(short = 'H', long, default_value = "127.0.0.1")]
    host: String,

    /// Server port
    #[arg(short, long, default_value_t = 25565)]
    port: u16,

    /// Seconds between checks
    #[arg(short, long, default_value_t = 10)]
    interval: u64,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    println!("Starting Hytale Health Checker...");
    println!("Target: {}:{}", args.host, args.port);
    println!("Interval: {}s", args.interval);
    println!("-----------------------------------------------------");

    // Use interval to ensure stable cadence and immediate first tick
    let mut interval = tokio::time::interval(Duration::from_secs(args.interval));

    loop {
        interval.tick().await; // First tick triggers immediately

        let timestamp = Local::now().format("%H:%M:%S");

        match check_hytale_status(&args.host, args.port).await {
            Ok(info) => {
                println!(
                    "[{}] 🟢 ONLINE | Ping: {}ms",
                    timestamp,
                    info.latency_ms
                );
            }
            Err(e) => {
                println!("[{}] 🔴 OFFLINE | Error: {}", timestamp, e);
            }
        }
    }
}
