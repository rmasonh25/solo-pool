// --- config.rs ---

use serde::Deserialize;
use std::fs;

#[derive(Debug, Deserialize)]
pub struct PoolConfig {
    pub port: u16,
    pub difficulty: f64,
    pub wallets: Vec<String>,
}

pub fn load_config() -> PoolConfig {
    let content = fs::read_to_string("pool.toml").expect("Failed to read config file");
    toml::from_str(&content).expect("Invalid TOML format")
}
