use anyhow::{bail, Result};
use serde_json::{from_str, from_value, Value};
use tokio_tungstenite::tungstenite::Message;

use crate::{
    helpers::{constraint::flags as FLAG, enums::WsMsgType},
    models::{share::SSRequest, stream_data::SSStreamData, user::WSUsers},
};

pub fn validate_client_message_type(msg: Message) -> Result<WsMsgType> {
    let msg = msg.to_string();
    let parsed: Result<Value, _> = from_str(&msg.clone());
    // WsMsgType::NewConn
    match parsed {
        Ok(value) => match value["flag"].as_str().unwrap() {
            FLAG::USERS => {
                let r: Result<WSUsers, _> = from_value(value);
                match r {
                    Ok(message) => {
                        // ss_req_validation(&message)?;
                        Ok(WsMsgType::NewConn(message))
                    }
                    Err(_) => bail!("Failed to parse text message"),
                }
            }
            FLAG::SS_REQUEST => {
                let r: Result<SSRequest, _> = from_value(value);
                match r {
                    Ok(message) => {
                        // ss_req_validation(&message)?;
                        Ok(WsMsgType::SSReq(message))
                    }
                    Err(_) => bail!("Failed to parse text message"),
                }
            }
            FLAG::SS_STREAM => {
                let r: Result<SSStreamData, _> = from_value(value);
                match r {
                    Ok(message) => {
                        // ss_stream_validation(&message)?;
                        Ok(WsMsgType::SSStreamData(message))
                    }
                    Err(_) => bail!("Failed to parse text message"),
                }
            }

            _ => bail!("Invalid flag"),
        },
        Err(_) => bail!("Failed to decode message"),
    }
}
