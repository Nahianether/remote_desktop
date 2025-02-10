use std::{net::SocketAddr, thread};

use crate::{
    helpers::lock::{
        broad_cast::get_broadcast_admins,
        users::{get_all_u_senders_by_emails, get_user},
    },
    models::share::SSReqRes,
};

pub fn handle_client_fram_size(res: SSReqRes, addr: &SocketAddr) {
    let addr = addr.clone();
    thread::spawn(move || match get_user(&addr) {
        Some(client) => match get_broadcast_admins(&client.user_id.unwrap()) {
            Some(admins) => {
                for s in get_all_u_senders_by_emails(admins) {
                    match s.send(res.to_ws().unwrap()) {
                        Ok(_) => {}
                        Err(e) => eprintln!("{:?}", e),
                    }
                }
            }
            None => {}
        },
        None => {}
    });
}
