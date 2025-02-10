use std::{
    collections::HashSet,
    sync::{Mutex, OnceLock},
};

use crate::{
    helpers::types::{BroadcastIndexMap, ClientBuffer, SocketIndexMap},
    models::user::UserInfo,
};

pub static SOCKET_ADDRS: SocketIndexMap = OnceLock::new();
pub static CURRENT_USER: OnceLock<Mutex<HashSet<UserInfo>>> = OnceLock::new();
pub static BROADCAST_SENDER: BroadcastIndexMap = OnceLock::new();
pub static CLIENT_BUFFER: ClientBuffer = OnceLock::new();
