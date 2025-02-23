use std::{collections::HashMap, net::SocketAddr};
use tokio_tungstenite::tungstenite::handshake::server::{ErrorResponse, Request, Response};
use url::form_urlencoded;

use crate::{
    helpers::{
        constraint::{constraint::API_KEY, flags::*},
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
    let mut errors = vec![];
    let mut user = UserInfo::default();
    // println!("New Validation Request : \n {:?}", req.headers());
    let mut headers_params: HashMap<String, String> = HashMap::new();

    for (ref header, value) in req.headers() {
        headers_params.insert(
            header.as_str().to_string(),
            value.to_str().unwrap_or("").to_string(),
        );
    }

    if let Some(query) = req.uri().query() {
        for (key, value) in form_urlencoded::parse(query.as_bytes()) {
            headers_params.insert(key.into(), value.into());
        }
    }

    for (k, v) in headers_params.iter() {
        if k.to_lowercase() == WS_AUTH.to_lowercase() {
            if v.to_string() != API_KEY {
                errors.push("Invalid Authentication Key".to_string());
            }
        }
        if k.as_str().to_lowercase() == USER_ID.to_lowercase() {
            match !v.is_empty() {
                true => user.user_id = Some(v.to_string()),
                false => errors.push(format!("Invalid User ID : {v}")),
            }
        }

        if k.as_str().to_lowercase() == USER_MODE.to_lowercase() {
            if !v.is_empty() {
                match validate_user_mode(v) {
                    Ok(r) => user.user_mode = Some(r),
                    Err(_) => errors.push("Invalid User Mode : Server, Admin, Client".to_string()),
                };
            }
        }
    }

    if user.user_id.is_none() {
        errors.push("User ID is required".to_string());
    }

    if user.user_mode.is_none() {
        errors.push("User Mode is required".to_string());
    }

    println!("New Users Headers/Params : \n {:?}", headers_params);
    println!("Headers/Params Validation Errors : \n {:?}", errors);
    if errors.is_empty() {
        user.addr = Some(addr);
        add_socket_addr(addr, Some(user), None);
        Ok(res)
    } else {
        ws_err(Some(errors.join("\n")))
    }
}

fn ws_err(msg: Option<String>) -> Result<Response, ErrorResponse> {
    Err(Response::builder().status(401).body(msg).unwrap())
}

fn validate_user_mode(mode: &str) -> anyhow::Result<Mode> {
    match mode {
        "server" => Ok(Mode::Server),
        "admin" => Ok(Mode::Admin),
        "client" => Ok(Mode::Client),
        _ => Err(anyhow::anyhow!("Invalid User Mode")),
    }
}
