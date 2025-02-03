use std::{
    net::SocketAddr,
    sync::{Mutex, OnceLock},
};

use futures_util::stream::SplitSink;
use indexmap::IndexMap;
use tokio::{net::TcpStream, sync::mpsc::UnboundedSender};
use tokio_tungstenite::{tungstenite::Message, WebSocketStream};

use crate::models::user::UserInfo;

pub type USender = UnboundedSender<Message>;
pub type AddrAndUSender = (SocketAddr, USender);
pub type UserAndUSender = (Option<UserInfo>, Option<USender>);
pub type IndexMapType = IndexMap<SocketAddr, UserAndUSender>;

pub type SocketIndexMap = OnceLock<Mutex<IndexMapType>>;
pub type WsWriter = SplitSink<WebSocketStream<TcpStream>, Message>;
