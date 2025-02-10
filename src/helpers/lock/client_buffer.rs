use std::sync::Mutex;

use indexmap::IndexMap;
use tokio::sync::broadcast::{self, Receiver};

use crate::helpers::constraint::flags::BUFFER;

use super::oncelock::CLIENT_BUFFER;

pub fn init_client_buffer() -> Receiver<Vec<u8>> {
    let mut set = CLIENT_BUFFER
        .get_or_init(|| Mutex::new(IndexMap::new()))
        .lock()
        .unwrap();

    let (tx, rx) = broadcast::channel::<Vec<u8>>(3);
    set.insert(BUFFER.to_string(), tx);
    rx
}

pub fn add_bytes_in_client_buffer(bytes: Vec<u8>) {
    let set = CLIENT_BUFFER
        .get_or_init(|| Mutex::new(IndexMap::new()))
        .lock()
        .unwrap();

    let tx = set.get(BUFFER).unwrap().clone();
    match tx.send(bytes) {
        Ok(_) => {}
        Err(e) => {
            eprintln!("Error to send bytes to broadcast{:?}", e);
        }
    }
}

pub fn get_client_buffer_sender() -> broadcast::Sender<Vec<u8>> {
    let set = CLIENT_BUFFER
        .get_or_init(|| Mutex::new(IndexMap::new()))
        .lock()
        .unwrap();

    set.get(BUFFER).unwrap().clone()
}
