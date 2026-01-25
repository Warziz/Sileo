use std::net::{UdpSocket};
use std::thread;
use std::sync::mpsc::{Sender, Receiver};
use std::sync::Arc;

use crate::messaging::config::Config;
use crate::messaging::network::peer::PeerInfo;
use crate::messaging::utils::event::BackendEvent;
use crate::messaging::utils::message::{Message,MessageType};
use crate::messaging::network::chat::{init_sock, listener, send_message, wait_for_peer};
use crate::messaging::network::hole_punching::{hole_punching};
use crate::messaging::crypto::crypto::{prepare_pubkey, get_aes_key};


/// A client responsible for encrypted peer-to-peer messaging.
///
/// `MessagingClient` acts as the interface between the backend logic
/// and the TUI. It manages network communication, key exchange,
/// encryption, and message dispatching.
pub struct MessagingClient {
    /// UDP socket used for sending and receiving packets.
    socket: Arc<UdpSocket>,
    /// Information about the connected peer.
    peer: Arc<PeerInfo>,
    /// AES-256 key used to encrypt and decrypt messages.
    aes_key: Arc<[u8;32]>,
    /// Channel used to send backend events to the TUI.
    incoming: Sender<BackendEvent>,
    /// Channel used to receive outgoing messages from the TUI.
    outgoing: Option<Receiver<String>>,
}

impl MessagingClient {

    /// Creates and initializes a new `MessagingClient`.
    ///
    /// This function establishes the initial connection with the rendezvous
    /// server, exchanges public keys, derives a shared AES key, and performs
    /// NAT hole punching with the peer.
    ///
    /// # Arguments
    ///
    /// * `config` - Client configuration parameters (server address, ports,
    ///   username, connection method, etc.).
    /// * `incoming` - Channel used to send backend events to the TUI.
    /// * `outgoing` - Channel used to receive messages from the TUI to be sent
    ///   over the network.
    ///
    /// # Returns
    ///
    /// On success, returns an initialized `MessagingClient`.
    /// On failure, returns an error describing the issue.

    pub fn new(
        config: Config, 
        incoming: Sender<BackendEvent>,
        outgoing: Receiver<String>,
    ) -> Result<Self, Box<dyn std::error::Error>> {

        //initiate socket and server addresse
        let socket = init_sock(config.source_port)?;
        let rendezvous_ip = format!("{}:{}", config.server_ip, config.server_port);

        incoming.send(BackendEvent::Log(format!("Trying to connect to {}",rendezvous_ip))).ok();

        //generate pubkey for prepare encryption     
        let (pubkey_b64, keypair) = prepare_pubkey();

        //prepare message type: pubkey
        let msg = Message {
            status: MessageType::Pubkey,
            username: config.username.clone(),
            destination_port: None,
            source_port: None,
            method: config.method,
            pubkey: Some(pubkey_b64),
        };

        socket.send_to(serde_json::to_string(&msg)?.as_bytes(), &rendezvous_ip)?;

        //prepare message type: ready for communication
        let msg = Message {
            status: MessageType::Ready,
            username: config.username.clone(),
            destination_port: Some(config.destination_port),
            source_port: Some(config.source_port),
            method: config.method,
            pubkey: None,
        };

        socket.send_to(serde_json::to_string(&msg)?.as_bytes(), &rendezvous_ip)?;

        //generate peer information in struct then generate AES key
        let peer = wait_for_peer(&socket, &incoming)?;
        let aes_key = get_aes_key(keypair, &peer);

        //punching hole throught NAT
        hole_punching(&socket, &peer, &incoming)?;

        Ok(Self {
            socket: Arc::new(socket),
            peer: Arc::new(peer),
            aes_key: Arc::new(aes_key),
            incoming,
            outgoing: Some(outgoing)
        })
    }


    /// Starts the messaging client.
    ///
    /// This function spawns two background threads:
    /// - A listener thread that receives and decrypts messages from the peer.
    /// - A sender thread that encrypts and sends messages provided by the TUI.
    ///
    /// This method consumes the outgoing channel and should only be called once.

    pub fn start(&mut self) {

        //cloning values
        let socket_recv = self.socket.try_clone().expect("clone socket failed");
        let socket_send = self.socket.try_clone().expect("clone socket failed");

        let peer = self.peer.clone();
        let aes_key = self.aes_key.clone();

        let incoming = self.incoming.clone();
        let outgoing = self.outgoing.take().expect("outgoing already taken");

        //recieve thread
        {
            let peer = Arc::clone(&peer);
            let aes_key = Arc::clone(&aes_key);
            let incoming = incoming;

            thread::spawn(move || {
                listener(socket_recv, aes_key, peer, incoming);
            });
        }

        //sender thread
        {
            let peer = Arc::clone(&peer);
            let aes_key = Arc::clone(&aes_key);

            thread::spawn(move || {
                while let Ok(msg) = outgoing.recv() {
                    send_message(&socket_send, &peer, &aes_key, msg);
                }
            });
        }
    }
}
