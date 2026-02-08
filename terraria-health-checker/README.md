# Terraria Health Checker

A simple, robust Rust CLI tool for monitoring Terraria game servers via TCP.

## Features

- **Protocol**: Performs a TCP handshake to verify the server is accepting connections.
- **Metrics**: Measures TCP connection latency (Ping).
- **Efficiency**: Minimal resource usage, perfect for sidecar monitoring containers or quick CLI checks.
- **Monitoring**: Continuous checking loop with configurable pacing.

## Installation

Ensure you have Rust installed (via [rustup](https://rustup.rs/)).

```bash
cd terraria-health-checker
cargo build --release
```

The binary will be available at `target/release/terraria-health-checker`.

## Usage

```bash
# Basic usage (defaults to 127.0.0.1:7777, 10s interval)
./terraria-health-checker

# Monitor a specific server
./terraria-health-checker --host terraria.myserver.com --port 7777

# Fast polling mode
./terraria-health-checker --host 192.168.1.10 --interval 2
```

### CLI Arguments

| Argument     | Short | Default     | Description                         |
| :----------- | :---- | :---------- | :---------------------------------- |
| `--host`     | `-H`  | `127.0.0.1` | Target server IP or hostname.       |
| `--port`     | `-p`  | `7777`      | Target TCP port (Terraria default). |
| `--interval` | `-i`  | `10`        | Seconds between health checks.      |
