// --- wallet.rs ---

use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

pub struct WalletRotator {
    wallets: Vec<String>,
    index: AtomicUsize,
}

impl WalletRotator {
    pub fn new(wallets: Vec<String>) -> Arc<Self> {
        Arc::new(WalletRotator {
            wallets,
            index: AtomicUsize::new(0),
        })
    }

    pub fn next_wallet(&self) -> &str {
        let i = self.index.fetch_add(1, Ordering::SeqCst) % self.wallets.len();
        &self.wallets[i]
    }
}
