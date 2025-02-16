use anyhow::Result;
use futures_util::SinkExt;
use std::net::SocketAddr;
use tokio_tungstenite::tungstenite::{Error, Message};

use crate::helpers::{
    enum_impl::WsMsgTypeExt, enums::WsMsgType, lock::addr::remove_socket_addr, types::WsWriter,
};

pub async fn handle_ws_error(
    ws_writer: &mut WsWriter,
    addr: &SocketAddr,
    e: &Error,
) -> Result<bool> {
    println!("Error reading message from {}: {:?}", addr, e);

    let (close, err_msg) = match *e {
        Error::ConnectionClosed => (true, "Connection closed"),
        Error::Io(_) => (true, "IO error"),
        Error::Protocol(_) => (true, "Protocol error"),
        Error::WriteBufferFull(_) => (true, "Write buffer full"),
        Error::Utf8 => (false, "UTF-8 error"),
        Error::Url(_) => (false, "URL error"),
        Error::Http(_) => (false, "HTTP error"),
        Error::Capacity(_) => (false, "Capacity error"),
        Error::AlreadyClosed => (false, "Already closed"),
        Error::AttackAttempt => (false, "Attack attempt"),
        Error::HttpFormat(_) => (false, "HTTP format error"),
        Error::Tls(_) => (false, "TLS error"),
    };
    println!("Error: {}", err_msg);
    if close {
        remove_socket_addr(&addr);
        ws_writer
            .send(Message::Text(
                WsMsgType::Error(format!("Connection Closed with : {}", err_msg))
                    .to_json()?
                    .into(),
            ))
            .await
            .ok();
    }

    Ok(false)
}

pub async fn send_ws_err_message(ws_writer: &mut WsWriter, e: String) -> Result<bool> {
    ws_writer
        .send(Message::Text(
            WsMsgType::Error(e.to_string()).to_json()?.into(),
        ))
        .await
        .ok();

    Ok(false)
}
