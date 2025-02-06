use super::user::{UserInfo, WSUsers};
use crate::helpers::enums::{Mode, WsMsgType};
use std::net::SocketAddr;
use tokio_tungstenite::tungstenite::Message;

impl UserInfo {
    pub fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }

    pub fn default() -> Self {
        Self {
            user_id: None,
            user_mode: None,
            addr: None,
        }
    }

    pub fn new(user_id: Option<String>, user_mode: Option<Mode>, addr: Option<SocketAddr>) -> Self {
        Self {
            user_id,
            user_mode,
            addr,
        }
    }

    pub fn addr(self, addr: Option<SocketAddr>) -> Self {
        Self { addr, ..self }
    }

    pub fn user_id(self, user_id: Option<String>) -> Self {
        Self { user_id, ..self }
    }
}

pub trait UserExt {
    fn to_ws_user(&self) -> Message;
}

pub trait UsersExt {
    fn to_ws_users(&self) -> Message;
}

impl UserExt for UserInfo {
    fn to_ws_user(&self) -> Message {
        let users = serde_json::to_string(&self).unwrap();
        Message::Text(users.into())
    }
}

impl UsersExt for Vec<UserInfo> {
    fn to_ws_users(&self) -> Message {
        let users = serde_json::to_string(&self).unwrap();
        Message::Text(users.into())
    }
}

impl WSUsers {
    pub fn new(connected: Option<bool>, users: Option<Vec<UserInfo>>) -> Self {
        Self {
            flag: Some("users".to_string()),
            connected,
            users,
        }
    }

    pub fn connected(self, c: bool) -> Self {
        Self {
            connected: Some(c),
            ..self
        }
    }

    pub fn to_ws(&self) -> Message {
        // TODO
        let users = serde_json::to_string(&WsMsgType::NewConn(self.clone())).unwrap();
        Message::Text(users.into())
    }
}
