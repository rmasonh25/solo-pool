use crate::wallet::WalletRotator;
use std::sync::Arc;

pub async fn handle_client(socket: TcpStream, rotator: Arc<WalletRotator>) {
    // use rotator.next_wallet() to assign a wallet to this client
}
