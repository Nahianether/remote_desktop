use crate::helpers::lock::{
    broad_cast::get_broadcast_admins,
    users::{get_all_u_senders_by_emails, get_user},
};
use anyhow::Result;
use std::net::SocketAddr;
use tokio_tungstenite::tungstenite::Message;

pub async fn handle_ss_stream(msg: Message, addr: &SocketAddr) -> Result<()> {
    if let Some(client) = get_user(addr) {
        match get_broadcast_admins(&client.user_id.unwrap()) {
            Some(admins) => {
                for sender in get_all_u_senders_by_emails(admins) {
                    match sender.send(msg.clone()) {
                        Ok(_) => {}
                        Err(e) => println!("Error sending bytes: {:?}", e),
                    }
                }
            }
            None => {}
        }
    }
    Ok(())
}
