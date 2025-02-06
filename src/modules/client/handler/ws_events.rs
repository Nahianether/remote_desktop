use crate::{
    helpers::{
        enums::{SSReqType, WsMsgType},
        types::WsClientSender,
    },
    models::stream_data::SSStreamData,
    modules::screen_capture::screen_capture_fl::capture_screen,
};
use anyhow::Result;
use futures_util::SinkExt;

pub async fn handle_ws_events(
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
                    let msg = SSStreamData::default().bytes(Some(screen_data));
                    sender.send(msg.to_ws()?).await?;
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
