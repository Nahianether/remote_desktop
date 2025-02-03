use std::net::SocketAddr;

use serde::{Deserialize, Serialize};

use crate::helpers::enums::Mode;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct UserInfo {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_mode: Option<Mode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub addr: Option<SocketAddr>,
}
