use crate::{
    helpers::{
        enum_impl::WsMsgTypeExt,
        enums::{SSReqType, WsMsgType},
        lock::users::{get_user_and_sender, get_user_by_email},
    },
    models::share::SSRequest,
};
use anyhow::Result;
use std::net::SocketAddr;

pub async fn handle_ss_req(msg: &SSRequest, addr: &SocketAddr) -> Result<()> {
    match get_user_by_email(msg.user_id.clone().unwrap().as_str()) {
        Some((_, s)) => {
            let msg = SSRequest::default()
                .ss_req_type(Some(SSReqType::Start))
                .to_ws()?;
            s.unwrap().send(msg)?;
        }
        None => match get_user_and_sender(*addr) {
            Some((_, s)) => s.unwrap().send(
                WsMsgType::Error(format!(
                    "The client `{}` is not connected to the server.",
                    msg.user_id.clone().unwrap()
                ))
                .to_ws()?,
            )?,
            None => {}
        },
    }
    Ok(())
}
