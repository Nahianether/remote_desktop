use crate::helpers::enums::WsMsgType;
use anyhow::Result;
use std::net::SocketAddr;

use super::{handle_ss_req::handle_ss_req, handle_ss_stream::handle_ss_stream};

pub async fn handle_ws_events(msg_type: WsMsgType, addr: &SocketAddr) -> Result<()> {
    match msg_type {
        WsMsgType::NewConn(_) => {}
        WsMsgType::DisConn(_) => {}
        WsMsgType::Error(_) => {}
        WsMsgType::SSReq(v) => handle_ss_req(&v, addr).await?,
        WsMsgType::SSStreamData(v) => handle_ss_stream(&v, addr).await?,
    }
    Ok(())
}
