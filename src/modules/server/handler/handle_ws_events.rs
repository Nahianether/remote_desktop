use crate::helpers::enums::WsMsgType;
use anyhow::Result;
use std::net::SocketAddr;

use super::{client_fram_size::handle_client_fram_size, handle_ss_req::handle_ss_req};

pub async fn handle_ws_events(msg_type: WsMsgType, addr: &SocketAddr) -> Result<()> {
    match msg_type {
        WsMsgType::NewConn(_) => {}
        WsMsgType::DisConn(_) => {}
        WsMsgType::Error(_) => {}
        WsMsgType::SSReq(v) => handle_ss_req(&v, addr).await?,
        WsMsgType::SSFramSize(v) => handle_client_fram_size(v,addr),
        WsMsgType::SSStreamData(_) => {}
    }
    Ok(())
}
