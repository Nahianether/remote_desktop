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
        Ok(value) => match value["flag"].as_str() {
            Some(flag) => match flag {
                FLAG::SS_REQUEST => {
                    let r: Result<SSReqRes, _> = from_value(value);
                    match r {
                        Ok(message) => {
                            ss_req_validation(&message)?;
                            Ok(WsMsgType::SSReq(message))
                        }
                        Err(e) => bail!("Failed to parse text message: {:?}", e),
                    }
                }
                FLAG::SS_RESPONSE => {
                    let r: Result<SSReqRes, _> = from_value(value);
                    match r {
                        Ok(message) => {
                            ss_res_validation(&message)?;
                            Ok(WsMsgType::SSFramSize(message))
                        }
                        Err(e) => bail!("Failed to parse text message: {:?}", e),
                    }
                }
                FLAG::SS_STREAM => {
                    let r: Result<SSStreamData, _> = from_value(value);
                    match r {
                        Ok(message) => {
                            ss_stream_validation(&message)?;
                            Ok(WsMsgType::SSStreamData(message))
                        }
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
