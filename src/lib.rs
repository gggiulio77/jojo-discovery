pub mod discovery;

use std::net::SocketAddr;

use anyhow::bail;

pub async fn initialize(port: u16) -> anyhow::Result<()> {
    let bind_address: SocketAddr = match std::env::var("BROADCAST_BIND_ADDRESS") {
        Ok(value) => value.parse::<SocketAddr>()?,
        Err(_) => {
            bail!("[init]: BROADCAST_BIND_ADDRESS env not found")
        }
    };

    // TODO: remove discovery feature to another repo, tauri can spawn both discovery and websocket separately
    discovery::init_broadcast(bind_address, port).await?;

    Ok(())
}
