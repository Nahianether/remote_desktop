use std::{net::SocketAddr, sync::Mutex};

use indexmap::IndexMap;

use crate::{helpers::types::USender, models::user::UserInfo};

use super::oncelock::SOCKET_ADDRS;

pub fn init_socket_addrs() {
    let mut set = SOCKET_ADDRS
        .get_or_init(|| Mutex::new(IndexMap::new()))
        .lock()
        .unwrap();
    set.clear();
}

pub fn add_socket_addr(addr: SocketAddr, user: Option<UserInfo>, u_sender: Option<USender>) {
    let mut set = SOCKET_ADDRS
        .get_or_init(|| Mutex::new(IndexMap::new()))
        .lock()
        .unwrap();

    let u = match user {
        Some(u) => Some(u),
        None => match set.get(&addr).map(|(v, _)| v.clone()) {
            Some(v) => v,
            None => None,
        },
    };

    let s = match u_sender.clone() {
        Some(s) => Some(s),
        None => match set.get(&addr).map(|(_, s)| s.clone()) {
            Some(v) => v,
            None => None,
        },
    };

    if u_sender.is_some() {
        println!("Added new Socket User: {:?} with addr : {:?}", u, addr);
    }

    set.insert(addr, (u, s));
}

pub fn update_client_frame_size(client_id: &str, frame_size: Option<(usize, usize)>) {
    let mut set = SOCKET_ADDRS
        .get_or_init(|| Mutex::new(IndexMap::new()))
        .lock()
        .unwrap();
    for (_, (u, _)) in set.iter_mut() {
        if u.clone().unwrap().user_id.is_some() && u.clone().unwrap().user_id.unwrap() == client_id
        {
            u.as_mut().unwrap().frame_size = frame_size;
        }
    }
}

pub fn remove_socket_addr(addr: &SocketAddr) {
    let mut set = SOCKET_ADDRS
        .get_or_init(|| Mutex::new(IndexMap::new()))
        .lock()
        .unwrap();
    println!("Removing Socket User with addr : {:?}", addr);
    set.shift_remove(addr);
}
