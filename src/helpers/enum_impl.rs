use super::enums::WsMsgType;
use anyhow::Result;
use tokio_tungstenite::tungstenite::Message;

pub trait WsMsgTypeExt {
    fn to_json(&self) -> Result<String>;
    fn to_ws(&self) -> Result<Message>;
}

impl WsMsgTypeExt for WsMsgType {
    fn to_json(&self) -> Result<String> {
        Ok(serde_json::to_string(&self)?)
    }

    fn to_ws(&self) -> Result<Message> {
        Ok(Message::Text(self.to_json()?.into()))
    }
}
