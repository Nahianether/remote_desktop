use super::share::SSReqRes;
use crate::helpers::enums::SSReqType;
use anyhow::Result;
use tokio_tungstenite::tungstenite::Message;

impl SSReqRes {
    pub fn default() -> Self {
        Self {
            ss_req_type: None,
            client_id: None,
            flag: None,
            frame_size: None,
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

    pub fn new(
        f: &str,
        ss_req_type: Option<SSReqType>,
        client_id: Option<String>,
        frame_size: Option<(usize, usize)>,
    ) -> Self {
        Self {
            flag: Some(f.to_string()),
            client_id,
            ss_req_type,
            frame_size,
        }
    }
}
