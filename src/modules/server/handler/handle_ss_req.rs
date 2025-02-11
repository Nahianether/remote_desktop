use crate::{
    helpers::{
        enum_impl::WsMsgTypeExt,
        enums::WsMsgType,
        lock::users::{get_user_and_sender, get_user_by_email},
    },
    models::share::SSReqRes,
};
use anyhow::Result;
use std::net::SocketAddr;

pub async fn handle_ss_req(ss_req: &SSReqRes, addr: &SocketAddr) -> Result<()> {
    match get_user_and_sender(*addr) {
        Some((_, admin_sender)) => {
            match get_user_by_email(ss_req.client_id.clone().unwrap().as_str()) {
                Some((_, s)) => {
                    let msg = ss_req.clone().client_id(None).to_ws()?;
                    s.unwrap().send(msg)?;
                }
                None => admin_sender.unwrap().send(
                    WsMsgType::Error(format!(
                        "The client `{}` is not connected to the server.",
                        ss_req.client_id.clone().unwrap()
                    ))
                    .to_ws()?,
                )?,
            }
        }
        None => {}
    }

    Ok(())
}
