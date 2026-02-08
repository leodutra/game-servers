use std::time::Duration;
use tokio::net::UdpSocket;
use tokio::time::timeout;
use anyhow::{Context, Result};
use rand::Rng;

pub struct ServerInfo {
    pub is_online: bool,
    pub latency_ms: u128,
}

pub async fn check_hytale_status(host: &str, port: u16) -> Result<ServerInfo> {
    let addr_str = format!("{}:{}", host, port);
    let addrs = tokio::net::lookup_host(&addr_str).await
        .context("Failed to resolve host address")?;

    let mut last_error = anyhow::anyhow!("No address resolved");

    for addr in addrs {
        // Bind to appropriate interface based on IP version
        let bind_addr = if addr.is_ipv4() { "0.0.0.0:0" } else { "[::]:0" };

        let socket = match UdpSocket::bind(bind_addr).await {
            Ok(s) => s,
            Err(e) => {
                last_error = e.into();
                continue;
            }
        };

        // Create QUIC Initial Packet
        let mut packet = vec![0u8; 1200]; // Allocate 1200 bytes (Min MTU for QUIC)
        let mut offset = 0;

        // 1. Header Byte (Long Header, Initial, PN Len=1)
        packet[offset] = 0xC0;
        offset += 1;

        // 2. Version (4 bytes) - 0x00000001 (QUIC v1)
        packet[offset] = 0x00;
        packet[offset+1] = 0x00;
        packet[offset+2] = 0x00;
        packet[offset+3] = 0x01;
        offset += 4;

        // 3. Dest Connection ID Length (1 byte)
        packet[offset] = 8;
        offset += 1;

        // 4. Dest Connection ID (8 bytes - random)
        rand::rng().fill(&mut packet[offset..offset+8]);
        offset += 8;

        // 5. Source Connection ID Length (1 byte)
        packet[offset] = 8;
        offset += 1;

        // 6. Source Connection ID (8 bytes - random)
        rand::rng().fill(&mut packet[offset..offset+8]);
        offset += 8;

        // 7. Token Length (VarInt) - 0
        packet[offset] = 0;
        offset += 1;

        // 8. Length (VarInt)
        // Payload = Packet Num (1) + Frame Type (1) + Off (1) + Len (1) + Crypto Data (46) = 50 bytes
        // But for simplicity, we mock the length field as provided: 0x4000 | 100
        let payload_length: u16 = 100;
        let len_field = 0x4000 | payload_length;
        packet[offset] = (len_field >> 8) as u8;
        packet[offset+1] = (len_field & 0xFF) as u8;
        offset += 2;

        // 9. Packet Number (1 byte)
        packet[offset] = 0;
        offset += 1;

        // 10. Frame Type: CRYPTO (0x06)
        packet[offset] = 0x06;
        offset += 1;

        // 11. Offset (VarInt) - 0
        packet[offset] = 0;
        offset += 1;

        // 12. Length (VarInt) - 50 bytes of crypto data
        let crypto_data_len: u8 = 50;
        packet[offset] = crypto_data_len;
        offset += 1;

        // 13. CRYPTO Data (Simplified TLS ClientHello)
        let client_hello_prefix = [
            0x01, // Handshake Type: ClientHello
            0x00, 0x00, 0x2e, // Length: 46 bytes
            0x03, 0x03, // Version: TLS 1.2
        ];

        // Copy prefix
        for b in client_hello_prefix.iter() {
            packet[offset] = *b;
            offset += 1;
        }

        // Random 32 bytes
        rand::rng().fill(&mut packet[offset..offset+32]);
        offset += 32;

        // Suffix
        let client_hello_suffix = [
            0x00, // Session ID Length
            0x00, 0x02, // Cipher Suites Length
            0x13, 0x01, // TLS_AES_128_GCM_SHA256
            0x01, // Compression Methods Length
            0x00, // Compression Method: none
        ];

        for b in client_hello_suffix.iter() {
            packet[offset] = *b;
            offset += 1;
        }

        // 14. Padding (zeroes)
        // The vector was initialized with zeroes, so just ensure we send exactly 1200 bytes

        let start = std::time::Instant::now();

        if let Err(e) = socket.send_to(&packet, addr).await {
            last_error = e.into();
            continue;
        }

        // Wait for response
        let mut buf = [0u8; 1500];
        match timeout(Duration::from_secs(5), socket.recv_from(&mut buf)).await {
            Ok(Ok((_len, _src))) => {
                let latency = start.elapsed().as_millis();
                return Ok(ServerInfo {
                    is_online: true,
                    latency_ms: latency,
                });
            },
            Ok(Err(e)) => {
                last_error = e.into();
                continue;
            },
            Err(_) => {
                last_error = anyhow::anyhow!("Timeout connecting to {}", addr);
                continue;
            }
        }
    }

    Err(last_error)
}

