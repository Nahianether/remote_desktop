use std::{net::SocketAddr, thread};

use crate::{
    helpers::lock::{
        addr::update_client_frame_size,
        broad_cast::get_broadcast_admins,
        users::{get_all_u_senders_by_emails, get_user},
    },
    models::share::SSReqRes,
};

pub fn handle_client_fram_size(res: SSReqRes, addr: &SocketAddr) {
    let addr = addr.clone();
    thread::spawn(move || match get_user(&addr) {
        Some(client) => {
            update_client_frame_size(client.clone().user_id.unwrap().as_str(), res.frame_size);
            match get_broadcast_admins(&client.user_id.unwrap()) {
                Some(admins) => {
                    for s in get_all_u_senders_by_emails(admins) {
                        match s.send(res.to_ws().unwrap()) {
                            Ok(_) => {}
                            Err(e) => eprintln!("{:?}", e),
                        }
                    }
                }
                None => {}
            }
        }
        None => {}
    });
}
