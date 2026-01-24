use serde::{Deserialize,Serialize};

use crate::messaging::utils::connection::ConnectionMethod;

/// Represents the type of a protocol message exchanged between peers.
#[derive(Serialize, Deserialize, Debug)]
pub enum MessageType {
    /// Initial connectivity check message.
    Check,
    /// Indicates that the peer is ready to establish a connection.
    Ready,
    /// Carries the peer's public key.
    Pubkey,
}

/// Represents a protocol message exchanged during peer negotiation.
///
/// This structure is serialized and transmitted over the network
/// to coordinate connection setup and key exchange.
#[derive(Serialize, Deserialize, Debug)]
pub struct Message {
    /// The type of the message.
    pub status: MessageType,

    /// The username of the sending peer.
    pub username: String,

    /// The destination port used for peer-to-peer communication, if applicable.
    pub destination_port: Option<u16>,

    /// The source port of the sending peer, if applicable.
    pub source_port: Option<u16>,

    /// The connection method requested or negotiated.
    pub method: ConnectionMethod,

    /// The Base64-encoded public key of the peer, if provided.
    pub pubkey: Option<String>,
}
