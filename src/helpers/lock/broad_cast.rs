use super::oncelock::{BROADCAST_SENDER, CLIENT_BROADCAST_ENABLE};
use crate::helpers::enums::SSReqType;
use indexmap::IndexMap;
use std::sync::Mutex;

pub fn add_remove_ss_req_broadcast(admin: String, client: String, req_type: SSReqType) {
    let mut set = BROADCAST_SENDER
        .get_or_init(|| Mutex::new(IndexMap::new()))
        .lock()
        .unwrap();

    match set.get(&client).map(|v| v.clone()) {
        Some(mut admins) => {
            if req_type == SSReqType::Stop {
                admins.retain(|x| x != &admin);
                if admins.is_empty() {
                    set.shift_remove(&client);
                    return;
                }
                set.insert(client, admins);
                return;
            }
            admins.push(admin);
            set.insert(client, admins);
        }
        None => {
            if req_type == SSReqType::Stop {
                return;
            }

            set.insert(client, vec![admin]);
        }
    }
}

pub fn get_ss_broadcast_admins(client: &String) -> Option<Vec<String>> {
    let set = BROADCAST_SENDER
        .get_or_init(|| Mutex::new(IndexMap::new()))
        .lock()
        .unwrap();

    match set.get(client) {
        Some(admins) => Some(admins.clone()),
        None => None,
    }
}

pub fn get_broadcast_admins(client: &String) -> Option<Vec<String>> {
    let set = BROADCAST_SENDER
        .get_or_init(|| Mutex::new(IndexMap::new()))
        .lock()
        .unwrap();

    match set.get(client) {
        Some(admins) => Some(admins.clone()),
        None => None,
    }
}

pub fn ss_broadcast_is_active(client: &str) -> bool {
    let set = BROADCAST_SENDER
        .get_or_init(|| Mutex::new(IndexMap::new()))
        .lock()
        .unwrap();

    set.contains_key(client)
}

// ? for client
pub fn set_client_boradcast_enable(enable: bool) {
    let v = CLIENT_BROADCAST_ENABLE.get_or_init(|| Mutex::new(enable));
    *v.lock().unwrap() = enable;
}

pub fn get_client_boradcast_enable() -> bool {
    *CLIENT_BROADCAST_ENABLE
        .get_or_init(|| Mutex::new(false))
        .lock()
        .unwrap()
}
