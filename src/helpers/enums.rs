use serde::{Deserialize, Serialize};

use crate::models::{share::SSRequest, stream_data::SSStreamData, user::WSUsers};
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub enum Mode {
    Server,
    Admin,
    Client,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
#[serde(rename_all = "camelCase")]
pub enum WsMsgType {
    NewConn(WSUsers),
    DisConn(WSUsers),
    Error(String),
    SSReq(SSRequest),
    SSStreamData(SSStreamData),
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
#[serde(rename_all = "camelCase")]
pub enum SSReqType {
    Start,
    Stop,
}
