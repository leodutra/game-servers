# Hytale Health Checker

A lightweight, asynchronous Rust CLI tool designed to monitor the status and latency of Hytale game servers (Kestrel/QUIC).

## Features

- **Protocol**: Uses a compliant **QUIC Initial Packet** handshake to verify server reachability.
- **Metrics**: Measures Round-Trip Time (Ping).
- **Architecture**: Built with `tokio` for efficient asynchronous I/O and `rand` for secure connection ID generation.
- **Monitoring**: Runs a continuous health check loop with configurable intervals.

## Installation

Ensure you have Rust installed (via [rustup](https://rustup.rs/)).

```bash
cd hytale-health-checker
cargo build --release
```

The binary will be available at `target/release/hytale-health-checker`.

## Usage

Run the tool using `cargo run` or the compiled binary.

```bash
# Basic usage (defaults to localhost:5520, 10s interval)
./hytale-health-checker

# Monitor a remote server
./hytale-health-checker --host play.hytaleserver.com --port 5520

# Customize check interval
./hytale-health-checker --host 192.168.1.50 --interval 2
```

### CLI Arguments

| Argument     | Short | Default     | Description                            |
| :----------- | :---- | :---------- | :------------------------------------- |
| `--host`     | `-H`  | `127.0.0.1` | Target server IP or hostname.          |
| `--port`     | `-p`  | `5520`      | Target UDP port (Hytale QUIC default). |
| `--interval` | `-i`  | `10`        | Seconds between health checks.         |

## Technical Details

Since modern Hytale servers operate on the Kestrel engine which uses QUIC over UDP, standard TCP pings or RakNet queries (Bedrock) are often insufficient or unsupported. This tool constructs a valid, randomized QUIC Initial Packet (`0xC0` header) with a simplified TLS `ClientHello` payload to provoke a response from the server stack, accurately measuring application-level reachability.
