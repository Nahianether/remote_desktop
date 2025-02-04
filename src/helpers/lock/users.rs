use std::{net::SocketAddr, sync::Mutex};

use indexmap::IndexMap;

use crate::{
    helpers::types::{USender, UserAndUSender},
    models::user::UserInfo,
};

use super::oncelock::SOCKET_ADDRS;

pub fn get_all_u_sender() -> Vec<USender> {
    let set = SOCKET_ADDRS
        .get_or_init(|| Mutex::new(IndexMap::new()))
        .lock()
        .unwrap();

    set.clone()
        .into_iter()
        .filter(|s| s.1 .1.is_some())
        .map(|(_, s)| s.1.unwrap())
        .collect()
}

pub fn get_user_and_sender(addr: SocketAddr) -> Option<UserAndUSender> {
    let set = SOCKET_ADDRS
        .get_or_init(|| Mutex::new(IndexMap::new()))
        .lock()
        .unwrap();

    let users = set
        .clone()
        .into_iter()
        .filter(|(u, v)| u == &addr && v.0.is_some() && v.1.is_some())
        .map(|(_, s)| s)
        .collect::<Vec<UserAndUSender>>();
    match users.is_empty() {
        true => None,
        false => users.first().cloned(),
    }
}

pub fn get_all_users_and_senders() -> Option<Vec<UserAndUSender>> {
    let set = SOCKET_ADDRS
        .get_or_init(|| Mutex::new(IndexMap::new()))
        .lock()
        .unwrap();

    let users = set
        .clone()
        .into_iter()
        // .filter(|(u, v)| u == &addr && v.0.is_some() && v.1.is_some())
        .map(|(_, s)| s)
        .collect::<Vec<UserAndUSender>>();
    match users.is_empty() {
        true => None,
        false => Some(users.clone()),
    }
}

pub fn get_user(addr: &SocketAddr) -> Option<UserInfo> {
    let set = SOCKET_ADDRS
        .get_or_init(|| Mutex::new(IndexMap::new()))
        .lock()
        .unwrap();

    let users = set
        .clone()
        .into_iter()
        .filter(|(u, v)| u == addr && v.0.is_some())
        .map(|(_, s)| s.0.unwrap())
        .collect::<Vec<UserInfo>>();
    match users.is_empty() {
        true => None,
        false => users.first().cloned(),
    }
}

pub fn get_user_by_email(e: &str) -> Option<UserAndUSender> {
    let set = SOCKET_ADDRS
        .get_or_init(|| Mutex::new(IndexMap::new()))
        .lock()
        .unwrap();
    match set
        .clone()
        .into_iter()
        .find(|(_, (u, _))| u.is_some() && u.clone().unwrap().user_id == Some(e.to_string()))
    {
        Some((_, s)) => Some(s.clone()),
        None => None,
    }
}

pub fn get_all_others(addr: &SocketAddr) -> Vec<UserAndUSender> {
    let set = SOCKET_ADDRS
        .get_or_init(|| Mutex::new(IndexMap::new()))
        .lock()
        .unwrap();

    set.clone()
        .into_iter()
        .filter(|(u, v)| u != addr && v.1.is_some())
        .map(|(_, s)| s)
        .collect()
}

pub fn get_all_other_users(addr: SocketAddr) -> Vec<UserInfo> {
    let set = SOCKET_ADDRS
        .get_or_init(|| Mutex::new(IndexMap::new()))
        .lock()
        .unwrap();

    set.clone()
        .into_iter()
        .filter(|(u, v)| u != &addr && v.0.is_some())
        .map(|(_, s)| s.0.unwrap())
        .collect()
}
