use anyhow::{bail, Result};
use serde_json::{from_str, from_value, Value};
use tokio_tungstenite::tungstenite::Message;

use crate::{
    helpers::{constraint::flags as FLAG, enums::WsMsgType},
    models::{share::SSReqRes, stream_data::SSStreamData, user::WSUsers},
};

pub fn validate_client_message_type(msg: Message) -> Result<WsMsgType> {
    let msg = msg.to_string();
    let parsed: Result<Value, _> = from_str(&msg.clone());
    // WsMsgType::NewConn
    match parsed {
        Ok(value) => match value["flag"].as_str() {
            Some(flag) => match flag {
                FLAG::USERS => {
                    let r: Result<WSUsers, _> = from_value(value);
                    match r {
                        Ok(message) => Ok(WsMsgType::NewConn(message)),
                        Err(e) => bail!("Failed to parse text message: {:?}", e),
                    }
                }
                FLAG::SS_REQUEST => {
                    let r: Result<SSReqRes, _> = from_value(value);
                    match r {
                        Ok(message) => Ok(WsMsgType::SSReq(message)),
                        Err(e) => bail!("Failed to parse text message: {:?}", e),
                    }
                }
                FLAG::SS_STREAM => {
                    let r: Result<SSStreamData, _> = from_value(value);
                    match r {
                        Ok(message) => Ok(WsMsgType::SSStreamData(message)),
                        Err(e) => bail!("Failed to parse text message: {:?}", e),
                    }
                }
                _ => bail!("Flag not found: {:?}", flag),
            },
            None => bail!("The flag is invalid : {:?}", value["flag"].as_str()),
        },
        Err(e) => bail!("Failed to decode message: {:?}", e),
    }
}
