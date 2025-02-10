use crate::{
    helpers::{
        enums::{SSReqType, WsMsgType},
        lock::broad_cast::set_client_boradcast_enable,
        types::WsUserWriter,
    },
    modules::client::handler::ss_start::client_ss_start,
};
use anyhow::Result;

pub async fn handle_ws_client_events(
    _writer: &mut WsUserWriter,
    message: WsMsgType,
    _addr: &str,
) -> Result<()> {
    match message {
        WsMsgType::SSReq(v) => {
            println!("Received a SSReq message: {:?}", v);
            match v.ss_req_type.unwrap() {
                SSReqType::Start => client_ss_start(),
                SSReqType::Stop => {
                    set_client_boradcast_enable(false);
                }
            }
        }
        WsMsgType::NewConn(v) => {
            println!("User connected: {:?}", v);
        }
        _ => {}
    }
    Ok(())
}
