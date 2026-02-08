#[derive(Clone, PartialEq)]
pub struct GameCardConfig {
    pub name: &'static str,
    pub background_image: &'static str,
    pub border_class: &'static str,
    pub logo: &'static str,
}

pub const TERRARIA_CONFIG: GameCardConfig = GameCardConfig {
    name: "Terraria Server",
    background_image: "images/terraria-card-bg.jpg",
    border_class: "border-brown",
    logo: "images/terraria-logo.jpg",
};

pub const HYTALE_CONFIG: GameCardConfig = GameCardConfig {
    name: "Hytale Server",
    background_image: "images/hytale-card-bg.jpg",
    border_class: "border-hytale",
    logo: "images/hytale-logo.png",
};
