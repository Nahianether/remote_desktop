use anyhow::Result;

use super::enums::WsMsgType;

pub trait WsMsgTypeExt {
    fn to_json(&self) -> Result<String>;
}
impl WsMsgTypeExt for WsMsgType {
    fn to_json(&self) -> Result<String> {
        Ok(serde_json::to_string(&self)?)
    }
}
