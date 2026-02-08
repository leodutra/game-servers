use serde::{Deserialize, Serialize};

pub use status_monitor::{ServiceStatus, History};

#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq)]
pub struct PublicConfig {
    pub terraria: String,
    pub hytale: String,
}
