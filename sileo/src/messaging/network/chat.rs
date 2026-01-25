use std::net::{UdpSocket};
use std::io;
use std::sync::mpsc::Sender;
use std::sync::{Arc};
use std::str;


use x25519_dalek::PublicKey;
use base64::{engine::general_purpose, Engine as _};

use crate::messaging::network::peer::{PeerInfo};
use crate::messaging::crypto::aes::{decrypt, encrypt};
use crate::messaging::utils::event::BackendEvent;

/// Generate a socket (UDP) on 0.0.0.0 and a choosen port.
/// 
/// # Arguments
/// 
/// * `port` - The local port choose by the user.
/// 
/// # Returns
/// 
/// A udp socket
pub fn init_sock(port: u16) -> io::Result<UdpSocket> {
    
    let addr = format!("0.0.0.0:{}",port);
    let socket = UdpSocket::bind(addr)?;
    socket.set_nonblocking(false)?; 
    Ok(socket)
}

/// Recieve all the peer message from the network, then send it to the TUI with the channel.
///
/// The main loop contain a match loop that will check the code message: 0x01 = peer message, 0x02 = punching hole.
/// All the message a decrypted with the AES key using AES-256-GCM algorithm.  
/// 
/// # Arguments
/// 
/// * `socket` - UDP socket use for recieve message
/// * `aes_key` - A 32-byte AES-256 key, should be "Arced".
/// * `peer` - A structure that contain all the peer info needed, should be "Arced".
/// * `incoming` - A sender channel use to communicate with the TUI. BackendEvent is an enum to classify messages.
pub fn listener(socket: UdpSocket, aes_key: Arc<[u8; 32]>, peer: Arc<PeerInfo>, incoming: Sender<BackendEvent>){

        let mut buffer = [0; 1024];
        let peer = Arc::clone(&peer);
    
            loop {

                if let Ok((len, _)) = socket.recv_from(&mut buffer){
                    let data = &buffer[..len];
                    
                    if data.is_empty(){
                        continue;
                    }
                    
                    let msg_type = data[0];

                    match msg_type {
                        0x01 => {
                            let payload = &data[1..];
                            if payload.len() < 12 {
                                continue;
                            }
                            //Get nonce & cipher_text
                            let (nonce_slice, ciphertext) = payload.split_at(12);
                            let nonce: [u8; 12] = match nonce_slice.try_into() {
                                Ok(n) => n,
                                Err(_) => {
                                    eprintln!("Invalid nonce length");
                                    continue;
                                }
                            };

                            //decipher the message
                            let plain = decrypt(&aes_key, ciphertext, &nonce);
                            let message = match String::from_utf8(plain) {
                                Ok(m) => m,
                                    Err(_) => {
                                    eprintln!("Invalid UTF-8 in decrypted message");
                                    return;
                                }
                            };
                            
                            //send the message to the TUI 
                            let _ = incoming.send(BackendEvent::PeerMessage { username: peer.peer_username.clone(), message });
                        
                        }

                        0x02 => {
                            //this part is made to do the hole without loosing the first message of the peer.
                            //just checking if the message is CTRL:PUNCH
                            let payload = &data[1..];
                            let message = match std::str::from_utf8(payload) {
                                Ok(m) => m,
                                Err(_) => {
                                    eprintln!("Invalid UTF-8 in CTRL message");
                                    continue;
                                }
                            };

                            if message.trim() == "CTRL:PUNCH" {
                                continue;
                            }
                        }

                        _ => continue,
                    }
                    
            }
        }
}

/// Encrypts and sends a message to a peer over UDP.
///
/// The message is encrypted using AES-256-GCM and sent as a single UDP packet.
/// Empty or whitespace-only messages are ignored.
///
/// # Arguments
///
/// * `socket` - The UDP socket used to send the packet.
/// * `peer` - Information about the destination peer.
/// * `aes_key` - The 32-byte AES-256 key used for encryption.
/// * `msg` - The plaintext message to send.
pub fn send_message(socket: &UdpSocket, peer: &PeerInfo, aes_key: &[u8; 32], msg:String ) {
    
    let peer_addr = format!("{}:{}",peer.peer_ip,peer.sport);

        let msg = msg.trim_end().as_bytes();
        if msg.is_empty() {
            return;
        }

        let (cipher_text,nonce) = encrypt(aes_key, msg);
        let mut packet = Vec::new();
        packet.push(0x01);
        packet.extend_from_slice(&nonce);
        packet.extend_from_slice(&cipher_text);
        if let Err(e) = socket.send_to(&packet, &peer_addr) {
            eprintln!("Failed to send message: {e}");
        }

}


/// Waits for peer information received over UDP.
///
/// This function blocks until valid peer data is received, parses the
/// incoming message, and constructs a `PeerInfo` structure from it.
/// Status updates are sent to the backend through the provided channel.
///
/// # Arguments
///
/// * `socket` - The UDP socket used to receive data.
/// * `incoming` - A channel sender used to emit backend events.
///
/// # Returns
///
/// On success, returns the parsed `PeerInfo`.
/// On failure, returns an `io::Error` describing the issue.
pub fn wait_for_peer(
    socket: &UdpSocket,
    incoming: &Sender<BackendEvent>
) -> io::Result<PeerInfo> {
    
    incoming.send(
        BackendEvent::Log("Waiting for peer".to_string())
    ).ok();
    
    let mut buf = [0u8; 4096];

    loop {
        let (len, _) = socket.recv_from(&mut buf)?;
        
        incoming.send(
            BackendEvent::Log("Data recieved from relay server".to_string())
        ).ok();
        
        let data = str::from_utf8(&buf[..len]).map_err(|e| {
            io::Error::new(
                io::ErrorKind::InvalidData,
                format!("Invalid UTF-8: {}", e),
            )
        })?;

        let data = data.trim();

        let parts: Vec<&str> = data.split_whitespace().collect();

        if parts.len() != 4 {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!("Invalid peer info format: {}", data),
            ));
        }

        let ip = parts[0].to_string();

        let sport: u16 = parts[1].parse().map_err(|_| {
            io::Error::new(
                io::ErrorKind::InvalidData,
                format!("Invalid port: {}", parts[1]),
            )
        })?;

        //decode pubkey b64 -> bytes
        let pubkey_bytes = general_purpose::STANDARD
            .decode(parts[2])
            .map_err(|_| {
                io::Error::new(
                    io::ErrorKind::InvalidData, 
                    "Invalid base64 public key",
                )
            })?;

        if pubkey_bytes.len() != 32 {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Invalid public key length",
            ));
        }
        
        //reconstruct pubkey bytes -> Pubkey
        let mut pubkey_array = [0u8;32];
        pubkey_array.copy_from_slice(&pubkey_bytes);
        let pubkey_other = PublicKey::from(pubkey_array);

        let peer_username = parts[3].to_string();

        incoming.send(
            BackendEvent::PeerConnected { username: peer_username.clone(), 
            }
        ).ok();

        return Ok(PeerInfo {
            peer_ip: ip,
            sport,
            peer_pubkey: pubkey_other,
            peer_username,
        });
    }
}