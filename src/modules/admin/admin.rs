use anyhow::Result;
use futures_util::{SinkExt, StreamExt};
use minifb::Window;
use tokio_tungstenite::{
    connect_async,
    tungstenite::{client::IntoClientRequest, http::HeaderValue, Message},
};

use crate::{
    helpers::enums::{Mode, SSReqType},
    models::share::SSRequest,
    modules::admin::{
        handler::{
            handle_binary_events::handle_admin_binary_events, ws_events::handle_ws_admin_events,
        },
        validation::msg_validation::validate_admin_message_type,
    },
};

pub async fn run_admin(admin_id: &str, addr: &str) -> Result<()> {
    let mut request = addr.into_client_request().unwrap();

    let headers = request.headers_mut();
    headers.insert("authorization", HeaderValue::from_static("api_key"));
    headers.insert("userId", HeaderValue::from_str(admin_id).unwrap());
    headers.insert(
        "userMode",
        HeaderValue::from_str(Mode::Client.to_string().as_str()).unwrap(),
    );

    let (stream, _) = connect_async(request).await.unwrap();

    let (mut write, mut read) = stream.split();

    let msg = SSRequest::new("ss_request", SSReqType::Start, "1@client.user").to_ws()?;

    write.send(msg).await.unwrap();
    let mut window = get_window();

    loop {
        // tokio::select! {
        //   Some(msg)= read.next() =>{
        let msg = read.next().await.unwrap();
        match msg {
            Ok(msg) => {
                println!("Received a message: {:?}", msg);
                match msg.clone() {
                    Message::Text(_) => match validate_admin_message_type(msg.clone()) {
                        Ok(message) => {
                            handle_ws_admin_events(&mut write, message, &addr).await?;
                        }
                        Err(e) => {
                            println!("{:?}", e)
                        }
                    },
                    Message::Binary(b) => {
                        handle_admin_binary_events(&mut window, &mut write, b, &addr).await?;
                    }
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
        //   }
        // }
    }
}

fn get_window() -> Window {
    let mut window = minifb::Window::new(
        "Remote Desktop Viewer",
        1920,
        1080,
        minifb::WindowOptions::default(),
    )
    .expect("Failed to create window");

    window.limit_update_rate(Some(std::time::Duration::from_micros(16600))); // ~60 FPS limit (error occured or buffer drop if decrease from 60)
    window
}
