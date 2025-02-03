use anyhow::{Ok, Result};
use tokio::net::TcpListener;

use crate::helpers::ws_functions::live_server_ip_address;

pub async fn ws_config() -> Result<TcpListener> {
    let addr = live_server_ip_address();
    let try_socket = TcpListener::bind(&addr).await;
    let listener = try_socket?;
    println!("WS Listening on: {}", addr);
    Ok(listener)
}
