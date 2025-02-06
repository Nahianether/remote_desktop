use super::share::SSRequest;
use crate::helpers::enums::SSReqType;
use anyhow::Result;
use tokio_tungstenite::tungstenite::Message;

impl SSRequest {
    pub fn default() -> Self {
        Self {
            ss_req_type: None,
            client_id: None,
            flag: None,
        }
    }

    pub fn ss_req_type(self, ssr: Option<SSReqType>) -> Self {
        Self {
            ss_req_type: ssr,
            ..self
        }
    }
    pub fn client_id(self, client_id: Option<String>) -> Self {
        Self { client_id, ..self }
    }

    pub fn to_ws(&self) -> Result<Message> {
        Ok(Message::Text(serde_json::to_string(&self)?.into()))
    }

    pub fn new(f: &str, t: SSReqType, client_id: &str) -> Self {
        Self {
            flag: Some(f.to_string()),
            client_id: Some(client_id.to_string()),
            ss_req_type: Some(t),
        }
    }
}
