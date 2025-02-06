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
    modules::server::{
        connection::{get_existing_connections, new_connection_notify},
        functions::{handle_ws_error, send_ws_err_message},
        handler::{handle_ss_stream::handle_ss_stream, handle_ws_events::handle_ws_events},
        validations::{
            connection_validation::conn_validation, msg_validation::validate_message_type,
        },
    },
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

    get_existing_connections(tx.clone(), addr.clone())?;
    new_connection_notify(&addr)?;

    loop {
        tokio::select! {
          Some(msg)= ws_read.next() =>{
            match msg {
              Ok(msg) => {
                match ws_disconnected(&addr ,msg.clone())?{
                  true=>{ break Ok(()); }
                  false=>{
                    println!("Received a message {} from {}", msg,addr);
                    match msg {
                        Message::Text(_)=>{
                          match validate_message_type(msg.clone()){
                            Ok(message)=> {
                              handle_ws_events(message, &addr).await?;
                            }
                            Err(e)=> {
                              send_ws_err_message(&mut ws_writer, e.to_string()).await?;
                            }
                          }
                        }
                        Message::Binary(_)=>{
                          handle_ss_stream(msg, &addr).await?;
                        }
                        _=>{}
                    }
                  }
                }
              }
              Err(e) => {
                handle_ws_error(&mut ws_writer, &addr, &e).await?;
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
