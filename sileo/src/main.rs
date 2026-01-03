mod utils;
mod config;
mod crypto;

use std::{io, sync::{Arc, Mutex}};

use utils::arg::parse_args;
use utils::network::{init_sock,listener,start_input_loop, wait_for_peer, hole_punching};
use config::config::Config;
use crypto::crypto::{generate_keypair,derive_shared_key};
use crypto::kdf::derive_aes_key;

use serde::{Deserialize,Serialize};
use base64::{engine::general_purpose, Engine as _};
use crate::{utils::{connection::ConnectionMethod, user::color_text}};

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

    let msg = format!("[*] Runtime config: {:?}", config);
    println!("{}", color_text(&msg, "yellow"));

    let socket = init_sock(config.source_port)?;

    //sending check message
    let rendezvous_ip = format!("{}:{}",config.server_ip,config.server_port);

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
    println!("{}",color_text("[*] Sending cryptographique pubkey to relay server","yellow"));
    
    let serialized_msg = serde_json::to_string(&msg)?;
    crypto_socket.send_to(serialized_msg.as_bytes(), &rendezvous_ip).unwrap();
    
    //sending ready message
    let msg = Message {
        status:MessageType::Ready,
        username: config.username.clone(),
        destination_port: Some(config.destination_port),
        source_port: Some(config.source_port),
        method: config.method,
        pubkey: None,
    };

    let ready_socket = socket.try_clone()?;
    let serialized_msg = serde_json::to_string(&msg)?;
    ready_socket.send_to(serialized_msg.as_bytes(), &rendezvous_ip).unwrap();
    println!("{}",serialized_msg);

    //Wait for peer
    let peer_info = wait_for_peer(&ready_socket)?;

    let peer_addr = format!("{}:{}",peer_info.0,peer_info.1);
    let peer_public = peer_info.2;

    //get aeskey
    let shared = derive_shared_key(keypair.secret, &peer_public);
    let aes_key = derive_aes_key(shared);

    //hole punching
    hole_punching(&socket, &peer_addr)?;
    let recv_socket = socket.try_clone()?;

    //protecting data for threading
    let stdout = Arc::new(Mutex::new(io::stdout()));

    println!("{}",color_text("[+] Sequence complete: press ENTER or send a message", "green"));
    
    start_input_loop(socket, &peer_addr, &aes_key);
    
    listener(
        config.username.clone(),  
        recv_socket, 
        stdout.clone(),
        aes_key
    );

    

    Ok(())
}
