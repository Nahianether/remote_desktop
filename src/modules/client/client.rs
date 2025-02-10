use crate::{
    helpers::{enums::Mode, lock::client_buffer::init_client_buffer},
    modules::client::{
        handler::ws_events::handle_ws_client_events,
        validation::msg_validation::validate_client_message_type,
    },
};
use anyhow::Result;
use futures_util::{SinkExt, StreamExt};
use tokio_tungstenite::{
    connect_async,
    tungstenite::{client::IntoClientRequest, http::HeaderValue, Message},
};

pub async fn run_client(client_id: &str, addr: &str) -> Result<()> {
    let mut buffer_receiver = init_client_buffer();
    let mut request = addr.into_client_request().unwrap();

    let headers = request.headers_mut();
    headers.insert("authorization", HeaderValue::from_static("api_key"));
    headers.insert("userId", HeaderValue::from_str(client_id).unwrap());
    headers.insert(
        "userMode",
        HeaderValue::from_str(Mode::Client.to_string().as_str()).unwrap(),
    );

    let (stream, _) = connect_async(request).await.unwrap();

    let (mut write, mut read) = stream.split();

    // let message = Message::Text("Hello WebSocket".to_string().into());
    // write.send(message).await.unwrap();

    loop {
        tokio::select! {
          Some(msg)= read.next() =>{
        // let msg = read.next().await.unwrap();
            match msg {
                // println!("Received a message: {:?}", msg);
              Ok(msg) => {
              match msg.clone() {
                Message::Text(_) => match validate_client_message_type(msg.clone()) {
                    Ok(message) => {
                        handle_ws_client_events(&mut write, message, &addr).await?;
                      }
                      Err(e) => {
                        println!("{:?}", e)
                      }
                    },
                    _ => {
                    println!("Received a non-text message: {:?}", msg);
                  }
                }
              }
              Err(e) => {
                println!("Error reading message: {:?}", e);
                break Ok(());
              }
            }
          },
          Ok(buffer) = buffer_receiver.recv() => {
            let b= buffer;
            match write.send(Message::binary(b)).await{
                Ok(_) => println!("bynary message sent"),
                Err(e) => println!("Error sending message to ws: {:?}", e)
            }
          }
        }
    }
}
