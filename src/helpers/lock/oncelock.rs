use std::{
    collections::HashSet,
    sync::{Mutex, OnceLock},
};

use crate::{helpers::types::SocketIndexMap, models::user::UserInfo};

pub static SOCKET_ADDRS: SocketIndexMap = OnceLock::new();
pub static CURRENT_USER: OnceLock<Mutex<HashSet<UserInfo>>> = OnceLock::new();
