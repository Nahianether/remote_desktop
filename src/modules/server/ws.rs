use anyhow::Result;
use futures_util::{SinkExt, StreamExt};
use std::net::SocketAddr;
use tokio::{net::TcpStream, sync::mpsc::unbounded_channel};
use tokio_tungstenite::tungstenite::{
    handshake::server::{Request, Response},
    Message,
};

use crate::{
    helpers::{lock::addr::add_socket_addr, ws_functions::ws_disconnected},
    modules::server::{functions::handle_slideshow_ws_error, validation::conn_validation},
};

use super::config::ws_config;

pub async fn run_ws() {
    match ws_config().await {
        Ok(listener) => {
            while let Ok((stream, addr)) = listener.accept().await {
                tokio::task::spawn(async move { handle_connection(stream, addr).await });
            }
        }
        Err(e) => eprintln!("{:?}", e),
    }
}

pub async fn handle_connection(raw_stream: TcpStream, addr: SocketAddr) -> Result<()> {
    println!("Incoming TCP connection from: {}", addr);
    let ws_stream =
        tokio_tungstenite::accept_hdr_async(raw_stream, |req: &Request, res: Response| {
            conn_validation(req, res, addr)
        })
        .await
        .expect("Error during the websocket handshake occurred");
    println!("WebSocket connection established: {}", addr);

    let (tx, mut rx) = unbounded_channel::<Message>();
    add_socket_addr(addr.clone(), None, Some(tx.clone()));

    let (mut ws_writer, mut ws_read) = ws_stream.split();

    loop {
        tokio::select! {
          Some(msg)= ws_read.next() =>{
            match msg {
              Ok(msg) => {
                match ws_disconnected(&addr ,msg.clone())?{
                  true=>{ break Ok(()); }
                  false=>{
                    println!("Received a message from {}", addr);
                    // let message= validate_message_type(msg.clone())?;
                    // sned_ws_message(message, &addr).await?;
                  }
                }
              }
              Err(e) => {
                handle_slideshow_ws_error(ws_writer, &addr, &e).await?;
                break Ok(())
              }
            }
          }
          Some(m)= rx.recv()=> {
            ws_writer.send(m).await.expect("Failed to send msg");
          }
        }
    }

    // Ok(())
}
