mod config;
mod handlers;
mod job;
mod wallet;

use crate::config::load_config;
use crate::wallet::WalletRotator;

use std::sync::Arc;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    // Load configuration from pool.toml
    let config = load_config();
    println!("[+] Starting Solo-Pool on port {} with {} wallets", config.port, config.wallets.len());

    // Initialize wallet rotator
    let wallet_rotator = WalletRotator::new(config.wallets.clone());

    // Start listening for miners
    let listener = TcpListener::bind(("0.0.0.0", config.port))
        .await
        .expect("[!] Failed to bind to port");

    // Spawn a job simulation loop (optional)
    tokio::spawn(job::start_job_broadcast());

    // Accept loop
    loop {
        match listener.accept().await {
            Ok((socket, addr)) => {
                println!("[*] New miner connected: {}", addr);
                let rotator = Arc::clone(&wallet_rotator);
                tokio::spawn(handlers::handle_client(socket, rotator));
            }
            Err(e) => {
                eprintln!("[!] Connection failed: {}", e);
            }
        }
    }
}

