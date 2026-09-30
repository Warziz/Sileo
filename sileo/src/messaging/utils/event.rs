use std::sync::{Arc, Mutex};

use crate::messaging::client::MessagingClient;


/// Defines the different types of message accepted by the TUI
#[derive(Debug, Clone)]

pub enum BackendEvent {

    /// Log messages in the chat area.
    Log(String),
    /// Display message when peer is connected with his pseudo.
    PeerConnected { username: String},
    /// All the peer messages.
    PeerMessage { username: String, message: String },
    /// Display when an errors occure.
    Error(String),
    /// Get the instance of messagingClient
    MessagingClient(Arc<Mutex<MessagingClient>>)
}