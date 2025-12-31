mod utils;
mod config;
mod crypto;

use std::{future::ready, io, sync::{Arc, Mutex}};

use utils::arg::parse_args;
use config::config::Config;
use utils::network::{init_sock,listener,start_input_loop, wait_for_peer};
use serde::{Deserialize,Serialize};
use x25519_dalek::PublicKey;
use crypto::crypto::{generate_keypair,derive_shared_key};
use crypto::kdf::derive_aes_key;
use base64::{engine::general_purpose, Engine as _};

use crate::{crypto::aes, utils::connection::ConnectionMethod};

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

fn main() -> io::Result<()> {

    let cli_opts = parse_args();

    let config = Config::from_cli(cli_opts)
        .expect("Invalid configuration");

    println!("Runtime config: {:?}", config);

    let socket = init_sock(config.source_port)?;
    let recv_socket = socket.try_clone()?;

    /*
    let msg = Message {
        status: MessageType::Check,
        username: config.username.clone(),
        destination_port: None,
        source_port: None,
        method: config.method,
        pubkey: None,
    };

    let serialized_msg = serde_json::to_string(&msg)?;
    let check_socket = socket.try_clone()?;
    */

    //sending check message
    let rendezvous_ip = format!("{}:{}",config.server_ip,config.server_port);
    //check_socket.send_to(serialized_msg.as_bytes(), &rendezvous_ip).unwrap();

    //prepare crypto message
    let crypto_socket = socket.try_clone()?;
    let keypair =  generate_keypair();

    let pubkey_b64 = general_purpose::STANDARD.encode(
        keypair.public.as_bytes()
    );

    let msg = Message {
        status: MessageType::Pubkey,
        username: config.username.clone(),
        destination_port:None,
        source_port: None,
        method: config.method,
        pubkey: Some(pubkey_b64),
    };

    //send pubkey
    println!("Sending cryptographique pubkey");
    let serialized_msg = serde_json::to_string(&msg)?;
    crypto_socket.send_to(serialized_msg.as_bytes(), &rendezvous_ip).unwrap();
    
    //recieve peer_pubkey
    //let mut buffer = [0u8; 4096];
    //let (size,addr) = crypto_socket.recv_from(&mut buffer)?;
    //println!("Pubkey Recieve");
    //convert json to rust struct
    //let recv_msg: Message = serde_json::from_slice(&buffer[..size])?;
    
    //sending ready message
    let msg = Message {
        status:MessageType::Ready,
        username: config.username.clone(),
        destination_port: None,
        source_port: None,
        method: config.method,
        pubkey: None,
    };

    let ready_socket = socket.try_clone()?;
    let serialized_msg = serde_json::to_string(&msg)?;
    ready_socket.send_to(serialized_msg.as_bytes(), &rendezvous_ip).unwrap();
    println!("{}",serialized_msg);

    /* 
    //extract pubkey value
    let peer_public = match recv_msg.pubkey {
        Some(pk) => pk,
        None => {
            eprintln!("No public key in message");
            return Ok(());
        }
    };
    */

    //Wait for peer
    println!("Wait for peer");
    let peer_info = wait_for_peer(&ready_socket)?;

    let peer_addr = format!("{}:{}",peer_info.0,peer_info.1);
    let peer_public = peer_info.2;

    //get aeskey
    let shared = derive_shared_key(keypair.secret, &peer_public);
    let aes_key = derive_aes_key(shared);
    println!("AES Key {:?}",aes_key);



    //protecting data for threading
    let stdout = Arc::new(Mutex::new(io::stdout()));

    listener(
        config.username.clone(),  
        recv_socket, 
        stdout.clone()
    );

    start_input_loop(socket, &peer_addr);

    Ok(())
}
