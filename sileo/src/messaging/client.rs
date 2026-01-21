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

pub struct MessagingClient {
    socket: Arc<UdpSocket>,
    peer: Arc<PeerInfo>,
    aes_key: Arc<[u8;32]>,
    incoming: Sender<BackendEvent>,
    outgoing: Option<Receiver<String>>,
}

impl MessagingClient {
    pub fn new(
        config: Config, 
        incoming: Sender<BackendEvent>,
        outgoing: Receiver<String>,
    ) -> Result<Self, Box<dyn std::error::Error>> {

        let socket = init_sock(config.source_port)?;
        let rendezvous_ip = format!("{}:{}", config.server_ip, config.server_port);

        incoming.send(BackendEvent::Log(format!("Trying to connect to {}",rendezvous_ip))).ok();

        let (pubkey_b64, keypair) = prepare_pubkey();

        let msg = Message {
            status: MessageType::Pubkey,
            username: config.username.clone(),
            destination_port: None,
            source_port: None,
            method: config.method,
            pubkey: Some(pubkey_b64),
        };

        socket.send_to(serde_json::to_string(&msg)?.as_bytes(), &rendezvous_ip)?;

        // Ready
        let msg = Message {
            status: MessageType::Ready,
            username: config.username.clone(),
            destination_port: Some(config.destination_port),
            source_port: Some(config.source_port),
            method: config.method,
            pubkey: None,
        };

        socket.send_to(serde_json::to_string(&msg)?.as_bytes(), &rendezvous_ip)?;

        let peer = wait_for_peer(&socket, &incoming)?;
        let aes_key = get_aes_key(keypair, &peer);

        hole_punching(&socket, &peer, &incoming)?;

        Ok(Self {
            socket: Arc::new(socket),
            peer: Arc::new(peer),
            aes_key: Arc::new(aes_key),
            incoming,
            outgoing: Some(outgoing)
        })
    }


    pub fn start(&mut self) {

        let socket_recv = self.socket.try_clone().expect("clone socket failed");
        let socket_send = self.socket.try_clone().expect("clone socket failed");

        let peer = self.peer.clone();
        let aes_key = self.aes_key.clone();

        let incoming = self.incoming.clone();
        let outgoing = self.outgoing.take().expect("outgoing already taken");

        // Thread réception
        {
            let peer = Arc::clone(&peer);
            let aes_key = Arc::clone(&aes_key);
            let incoming = incoming;

            thread::spawn(move || {
                listener(socket_recv, aes_key, peer, incoming);
            });
        }

        // Thread envoi
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
