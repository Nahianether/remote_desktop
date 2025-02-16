use serde::{Deserialize, Serialize};

use crate::helpers::enums::SSReqType;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct SSReqRes {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flag: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ss_req_type: Option<SSReqType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frame_size: Option<(usize, usize)>,
}
