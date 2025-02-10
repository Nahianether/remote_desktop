use anyhow::{bail, Result};
use serde_json::{from_str, from_value, Value};
use tokio_tungstenite::tungstenite::Message;

use crate::{
    helpers::{constraint::flags as FLAG, enums::WsMsgType},
    models::{share::SSReqRes, stream_data::SSStreamData},
};

use super::{
    ss_req_res_validation::{ss_req_validation, ss_res_validation},
    ss_stream_validation::ss_stream_validation,
};

pub fn validate_message_type(msg: Message) -> Result<WsMsgType> {
    let msg = msg.to_string();
    let parsed: Result<Value, _> = from_str(&msg.clone());
    match parsed {
        Ok(value) => match value["flag"].as_str().unwrap() {
            FLAG::SS_REQUEST => {
                let r: Result<SSReqRes, _> = from_value(value);
                match r {
                    Ok(message) => {
                        ss_req_validation(&message)?;
                        Ok(WsMsgType::SSReq(message))
                    }
                    Err(_) => bail!("Failed to parse text message"),
                }
            }
            FLAG::SS_RESPONSE => {
                let r: Result<SSReqRes, _> = from_value(value);
                match r {
                    Ok(message) => {
                        ss_res_validation(&message)?;
                        Ok(WsMsgType::SSFramSize(message))
                    }
                    Err(_) => bail!("Failed to parse text message"),
                }
            }
            FLAG::SS_STREAM => {
                let r: Result<SSStreamData, _> = from_value(value);
                match r {
                    Ok(message) => {
                        ss_stream_validation(&message)?;
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
