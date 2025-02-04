use super::share::SSRequest;
use crate::helpers::enums::SSReqType;
use anyhow::Result;
use tokio_tungstenite::tungstenite::Message;

impl SSRequest {
    pub fn default() -> Self {
        Self {
            ss_req_type: None,
            user_id: None,
            flag: None,
        }
    }

    pub fn ss_req_type(self, ssr: Option<SSReqType>) -> Self {
        Self {
            ss_req_type: ssr,
            ..self
        }
    }

    pub fn to_ws(&self) -> Result<Message> {
        Ok(Message::Text(serde_json::to_string(&self)?.into()))
    }
}
