mod crypto;
mod network;
mod error;

use console::style;
use anyhow::Result;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    sync::mpsc,
};
use crate::{
    crypto::Crypto,
    network::NetworkManager,
    error::ChatError,
};

#[tokio::main]
async fn main() -> Result<()> {
    let console = console::Console::new();
    let network = NetworkManager::new();
    let crypto = Crypto::new();

    console.info("Starting Secure P2P Chat...");
    
    let (public_key, private_key) = crypto.generate_key_pair()?;
    console.info(&format!("Your Public ID: {}", hex::encode(&public_key)));

    // Start network components
    let network_clone = network.clone();
    tokio::spawn(async move {
        network_clone.discover_peers().await.expect("Discovery failed");
    });

    let network_clone = network.clone();
    tokio::spawn(async move {
        network_clone.listen_for_peers().await.expect("Listener failed");
    });

    // User input handling
    let (tx, mut rx) = mpsc::channel(100);
    tokio::spawn(async move {
        let stdin = tokio::io::stdin();
        let mut buf = String::new();
        
        loop {
            buf.clear();
            stdin.read_line(&mut buf).await.expect("Read failed");
            tx.send(buf.trim().to_string()).await.expect("Send failed");
        }
    });

    // Message processing loop
    while let Some(message) = rx.recv().await {
        if message.eq_ignore_ascii_case("/exit") {
            break;
        }
        
        // TODO: Implement message encryption and sending
        println!("> {}", message);
    }

    Ok(())
}