
## Hytale Health Checker Specification

### Overview

`hytale-health-checker` is a Rust-based CLI utility designed to monitor Hytale game servers.
This tool utilizes a **QUIC Initial Packet** (UDP) handshake to verify server reachability and latency.
The current implementation focuses on "Ping" (Online/Offline status and RTT) rather than protocol-deep metadata extraction, matching the behavior of the server's QUIC stack (RFC 9000).

### Features

* **QUIC Handshake Ping**: Sends a compliant QUIC Initial Packet to trigger a server response.
* **Architecture**: Rust-based asynchronous network handling.
* **Visual Status**: Color-coded console output (Green/Red) with latency metrics.
* **Eager Execution**: Checks immediately upon start, then waits for the interval.

---

### Technical Architecture

#### Language & Runtime

* **Language**: Rust (Edition 2021)
* **Dependencies**:
* `clap` (v4.5): CLI argument parsing.
* `tokio`: Asynchronous UDP networking.
* `rand`: For generating connection IDs and nonces.

#### Logic Workflow

1.  **QUIC Initial Packet Construction**:
    *   Allocate a 1200-byte buffer (Min QUIC MTU).
    *   **Header**: Long Header (0xC0), Version 1 (RFC 9000).
    *   **Connection IDs**: Random 8-byte Source and Destination CIDs.
    *   **Payload**: A simplified `CRYPTO` frame containing a minimal TLS 1.2/1.3 `ClientHello`.
    *   **Padding**: Zero-pad the packet to exactly 1200 bytes to ensure processing by the server.

2.  **Transmission**:
    *   Send the packet to the target host/port via UDP.
    *   Record timestamp `T1`.

3.  **Response Handling**:
    *   Wait for *any* UDP response from the server.
    *   On receipt at `T2`, calculate RTT (`T2 - T1`).
    *   Mark server as **ONLINE**.

4.  **Error Handling**:
    *   Timeout (Default: 5s).
    *   Socket errors.

#### Core Logic (`src/lib.rs`)

* **`check_hytale_status`**: Asynchronous function returning `Result<ServerStatus>`.
* **Timeout**: Configurable, defaults to 5 seconds.
* **Data Structure**:

```rust
struct ServerStatus {
    is_online: bool,
    latency_ms: u128,
}
```

---

### CLI Application (`src/main.rs`)

1.  **Arguments**:
    *   `-H, --host`: Server address (Default: `127.0.0.1`).
    *   `-p, --port`: Server port (Default: `5520`, Hytale/QUIC default).
    *   `-i, --interval`: Seconds between checks (Default: `5`).

2.  **Display Output**:

```text
[17:10:05] 🟢 ONLINE | Ping: 45ms
[17:10:10] 🔴 OFFLINE | Error: Timeout
```

### Protocol Details (QUIC Initial Packet)

*   **Header Byte**: `0xC0` (Long Header, Initial, PN Len=1).
*   **Version**: `0x00000001` (QUIC v1).
*   **Dest CID Length**: 8 bytes.
*   **Dest CID**: Random 8 bytes.
*   **Src CID Length**: 8 bytes.
*   **Src CID**: Random 8 bytes.
*   **Token Length**: 0.
*   **Packet Length**: 2-byte VarInt.
*   **Packet Number**: 0 (1-byte).
*   **Frame**: CRYPTO (0x06).
*   **Crypto Data**: Synthesized TLS ClientHello.

