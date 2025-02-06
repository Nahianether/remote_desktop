use crate::{
    helpers::{
        enum_impl::WsMsgTypeExt,
        enums::WsMsgType,
        lock::{
            broad_cast::add_remove_ss_req_broadcast,
            users::{get_user_and_sender, get_user_by_email},
        },
    },
    models::share::SSRequest,
};
use anyhow::Result;
use std::net::SocketAddr;

pub async fn handle_ss_req(ss_req: &SSRequest, addr: &SocketAddr) -> Result<()> {
    match get_user_and_sender(*addr) {
        Some((admin_user, admin_sender)) => {
            match get_user_by_email(ss_req.client_id.clone().unwrap().as_str()) {
                Some((_, s)) => {
                    let msg = ss_req.clone().client_id(None).to_ws()?;
                    add_remove_ss_req_broadcast(
                        admin_user.unwrap().user_id.unwrap(),
                        ss_req.client_id.clone().unwrap(),
                        ss_req.ss_req_type.clone().unwrap(),
                    );
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
