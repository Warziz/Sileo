use serde::{Deserialize,Serialize};

use crate::utils::connection::ConnectionMethod;

#[derive(Serialize, Deserialize, Debug)]
pub enum MessageType {
    Check,
    Ready,
    Pubkey,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Message {
    pub status: MessageType,
    pub username: String,
    pub destination_port: Option<u16>,
    pub source_port: Option<u16>,
    pub method: ConnectionMethod,
    pub pubkey: Option<String>, 
}