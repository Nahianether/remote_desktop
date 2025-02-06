use super::stream_data::SSStreamData;
use crate::helpers::constraint::flags::SS_STREAM;
use anyhow::Result;
use tokio_tungstenite::tungstenite::Message;

impl SSStreamData {
    pub fn new(flag: Option<String>, bytes: Option<Vec<u8>>) -> Self {
        Self { flag, bytes }
    }

    pub fn default() -> Self {
        Self::new(Some(SS_STREAM.to_string()), None)
    }

    pub fn bytes(self, bytes: Option<Vec<u8>>) -> Self {
        Self { bytes, ..self }
    }

    pub fn to_ws(self) -> Result<Message> {
        let json = serde_json::to_string(&self)?;
        Ok(Message::Text(json.into()))
    }
}
