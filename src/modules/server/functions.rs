use anyhow::Result;
use futures_util::SinkExt;
use std::net::SocketAddr;
use tokio_tungstenite::tungstenite::{Error, Message};

use crate::helpers::{
    enum_impl::WsMsgTypeExt, enums::WsMsgType, lock::addr::remove_socket_addr, types::WsWriter,
};

pub async fn handle_ws_error(
    mut ws_writer: WsWriter,
    addr: &SocketAddr,
    e: &Error,
) -> Result<bool> {
    println!("Error reading message from {}: {:?}", addr, e);
    if let Error::Protocol(protocol_error) = e {
        if *protocol_error
            == tokio_tungstenite::tungstenite::error::ProtocolError::ResetWithoutClosingHandshake
        {
            println!("Connection reset without closing handshake: {}", addr);
            remove_socket_addr(&addr);
            // sned_ws_message(WsMsgType::Error("Connection Closed".to_string()), &addr).await?;
            ws_writer
                .send(Message::Text(
                    WsMsgType::Error("Connection Closed".to_string())
                        .to_json()?
                        .into(),
                ))
                .await
                .ok();
        }
    }
    Ok(false)
}
