use clap::Parser;
use std::{thread, time::Duration};
use terraria_health_checker::check_server_status;
use chrono::Local;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The IP address or hostname of the Terraria server
    #[arg(short = 'H', long, default_value = "127.0.0.1")]
    host: String,

    /// The port number
    #[arg(short, long, default_value_t = 7777)]
    port: u16,

    /// Seconds between checks
    #[arg(short, long, default_value_t = 10)]
    interval: u64,
}

fn main() {
    let args = Args::parse();

    println!("Checking Terraria server at {}:{}...", args.host, args.port);
    println!("Interval: {}s", args.interval);
    println!("-----------------------------------------------------");

    if args.host == "127.0.0.1" && args.port == 7777 {
        println!("(Using defaults. To specify a server, run with: --host <IP> --port <PORT>)");
    }

    loop {
        let timestamp = Local::now().format("%H:%M:%S");
        match check_server_status(&args.host, args.port) {
            Ok(info) => {
                if info.is_online {
                    println!("[{}] 🟢 ONLINE | Ping: {}ms", timestamp, info.latency_ms);
                } else {
                    println!("[{}] 🔴 OFFLINE | Connection timed out or refused", timestamp);
                }
            }
            Err(e) => {
                println!("[{}] 🔴 OFFLINE | Error: {}", timestamp, e);
            }
        }
        thread::sleep(Duration::from_secs(args.interval));
    }
}
