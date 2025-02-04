use crate::{
    helpers::lock::{
        addr::remove_socket_addr,
        users::{get_all_other_users, get_all_others, get_all_users_and_senders},
    },
    models::user::{UserInfo, WSUsers},
};
use anyhow::Result;
use std::net::SocketAddr;
use tokio::sync::mpsc::UnboundedSender;
use tokio_tungstenite::tungstenite::Message;

pub fn close_connection_notify(addr: &SocketAddr) -> Result<()> {
    remove_socket_addr(&addr);
    match get_all_users_and_senders() {
        Some(users_senders) => {
            for (u, sender) in users_senders {
                // let msg = u.unwrap().addr(Some(*addr)).to_ws_user();
                // UserInfo::new(Some(u.unwrap().user_id), None, Some(*addr)).to_dis_conn()
                // .to_json()
                // .to_ws_msg_text();
                let msg = WSUsers::new(Some(false), Some(vec![u.unwrap().clone()])).to_ws();
                sender.unwrap().send(msg)?;
            }
        }
        None => {}
    }
    Ok(())
}

pub fn new_connection_notify(addr: &SocketAddr) -> Result<()> {
    let others = get_all_others(&addr);
    // println!("others: {:?}", others);
    match others.is_empty() {
        true => println!("No recipient found"),
        false => match others.last() {
            Some((user, recp)) => match user {
                Some(u) => {
                    println!("New connection from: {:?}", user);
                    let msg = WSUsers::new(Some(true), Some(vec![u.clone()])).to_ws();
                    // let msg = u.to_ws_user();
                    match recp {
                        Some(r) => r.send(msg)?,
                        None => println!("No recipient found"),
                    }
                }
                None => println!("No recipient found"),
            },
            None => println!("No recipient found"),
        },
    }

    Ok(())
}

pub fn get_existing_connections(tx: UnboundedSender<Message>, addr: SocketAddr) -> Result<()> {
    let mut others: Vec<UserInfo> = vec![];
    for user in get_all_other_users(addr) {
        others.push(user);
    }
    let msg = WSUsers::new(Some(true), Some(others)).to_ws();
    tx.send(msg)?;
    Ok(())
}
