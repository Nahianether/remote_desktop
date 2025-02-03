use anyhow::Result;
use std::net::SocketAddr;
use tokio_tungstenite::tungstenite::Message;

use super::lock::addr::remove_socket_addr;

// pub async fn server_ip_address() -> SocketAddr {
//     local_server_ip_address().await
// }
// pub async fn local_server_ip_address() -> SocketAddr {
//     SocketAddr::from(([0, 0, 0, 0], free_port().await))
// }

pub fn live_server_ip_address() -> SocketAddr {
    SocketAddr::from(([0, 0, 0, 0], 8080))
}

pub fn ws_disconnected(addr: &SocketAddr, msg: Message) -> anyhow::Result<bool> {
    if msg.is_close() && msg.is_empty() {
        close_connection_notify(&addr)?;
        return Ok(true);
    }
    return Ok(false);
}

pub fn close_connection_notify(addr: &SocketAddr) -> Result<()> {
    remove_socket_addr(&addr);
    Ok(())
}
