use anyhow::{bail, Result};
use serde_json::{from_str, from_value, Value};
use tokio_tungstenite::tungstenite::Message;

use crate::{
    helpers::{constraint::flags as FLAG, enums::WsMsgType},
    models::share::SSRequest,
};

use super::ss_req_validation::ss_req_validation;

pub fn validate_message_type(msg: Message) -> Result<WsMsgType> {
    let msg = msg.to_string();
    let parsed: Result<Value, _> = from_str(&msg.clone());
    match parsed {
        Ok(value) => match value["flag"].as_str().unwrap() {
            FLAG::SS_REQUEST => {
                let r: Result<SSRequest, _> = from_value(value);
                match r {
                    Ok(message) => {
                        ss_req_validation(&message)?;
                        Ok(WsMsgType::SSReq(message))
                    }
                    Err(_) => bail!("Failed to parse text message"),
                }
            }

            _ => bail!("Invalid flag"),
        },
        Err(_) => bail!("Failed to decode message"),
    }
}
