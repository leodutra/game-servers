use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct ServerStatus {
    pub name: String,
    pub server_type: String, // "Terraria", "Minecraft", "Generic"
    pub online: bool,
    pub players: String,     // "3/16"
    pub ping: u32,
    pub details: String,     // Map name or error message
    pub last_updated: String,
}
