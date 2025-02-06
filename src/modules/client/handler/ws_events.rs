use crate::{
    helpers::{
        enums::{SSReqType, WsMsgType},
        types::WsClientSender,
    },
    modules::screen_capture::screen_capture_fl::capture_screen,
};
use anyhow::Result;
use futures_util::SinkExt;
use tokio_tungstenite::tungstenite::Message;

pub async fn handle_ws_client_events(
    sender: &mut WsClientSender,
    message: WsMsgType,
    _addr: &str,
) -> Result<()> {
    match message {
        WsMsgType::SSReq(v) => {
            println!("Received a SSReq message: {:?}", v);
            match v.ss_req_type.unwrap() {
                SSReqType::Start => loop {
                    let screen_data = capture_screen();
                    sender.send(Message::binary(screen_data)).await?;
                },
                SSReqType::Stop => {}
            }
        }
        WsMsgType::NewConn(v) => {
            println!("User connected: {:?}", v);
        }
        _ => {}
    }
    Ok(())
}
