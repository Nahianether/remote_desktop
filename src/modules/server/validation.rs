use std::net::SocketAddr;
use tokio_tungstenite::tungstenite::{
    handshake::server::{ErrorResponse, Request, Response},
    http::HeaderValue,
};

use crate::{
    helpers::{
        constraint::{API_KEY, USER_ID, USER_MODE, WS_AUTH},
        enums::Mode,
        lock::addr::add_socket_addr,
    },
    models::user::UserInfo,
};

pub fn conn_validation(
    req: &Request,
    res: Response,
    addr: SocketAddr,
) -> Result<Response, ErrorResponse> {
    let mut valid = false;
    let mut user = UserInfo::default();
    println!("New Validation Request : \n {:?}", req.headers());
    for (ref header, value) in req.headers() {
        if header.as_str() == WS_AUTH.to_lowercase() {
            println!("* {}: {:?}", header, value);
            valid = match validate_authentication_key(value) {
                Ok(v) => match v {
                    true => true,
                    false => false,
                },
                Err(_) => false,
            };
        }

        if header.as_str() == USER_ID.to_lowercase() {
            println!("* {}: {:?}", header, value);
            user.user_id = match validate_user_id(value) {
                Ok(r) => Some(r),
                Err(_) => None,
            };
        }

        if header.as_str() == USER_MODE.to_lowercase() {
            println!("* {}: {:?}", header, value);
            user.user_mode = match validate_user_mode(value) {
                Ok(r) => Some(r),
                Err(_) => None,
            };
        }
    }

    // println!("loop end");
    if valid {
        user.addr = Some(addr);
        add_socket_addr(addr, Some(user), None);
        Ok(res)
    } else {
        ws_err(Some("Invalid Authentication Key".to_string()))
    }
}

fn ws_err(msg: Option<String>) -> Result<Response, ErrorResponse> {
    Err(Response::builder().status(401).body(msg).unwrap())
}

fn validate_authentication_key(value: &HeaderValue) -> anyhow::Result<bool> {
    let k = value.to_str()?;
    Ok(k == API_KEY.to_lowercase())
}

fn validate_user_id(value: &HeaderValue) -> anyhow::Result<String> {
    Ok(value.to_str()?.to_string())
}

fn validate_user_mode(value: &HeaderValue) -> anyhow::Result<Mode> {
    let mode = value.to_str()?;
    match mode {
        "server" => Ok(Mode::Server),
        "admin" => Ok(Mode::Admin),
        "client" => Ok(Mode::Client),
        _ => Err(anyhow::anyhow!("Invalid User Mode")),
    }
}
