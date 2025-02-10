use crate::{
    helpers::{
        enums::{SSReqType, WsMsgType},
        lock::client_buffer::get_client_buffer_sender,
        types::WsUserWriter,
    },
    modules::screen_capture::screen_capture_fl::capture_screen,
};
use anyhow::Result;

pub async fn handle_ws_client_events(
    writer: &mut WsUserWriter,
    message: WsMsgType,
    _addr: &str,
) -> Result<()> {
    match message {
        WsMsgType::SSReq(v) => {
            println!("Received a SSReq message: {:?}", v);
            match v.ss_req_type.unwrap() {
                SSReqType::Start => loop {
                    let screen_data = capture_screen();
                    let sender = get_client_buffer_sender();
                    match sender.send(screen_data) {
                        Ok(_) => {}
                        Err(e) => println!("Error to send byts to broadcast : {:?}", e),
                    }
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
