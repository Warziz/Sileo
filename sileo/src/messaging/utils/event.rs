

#[derive(Debug, Clone)]

pub enum BackendEvent {

    Log(String),
    PeerConnected { username: String},
    PeerMessage { username: String, message: String },
    Error(String),
}