mod utils;
mod config;
mod crypto;
mod network;

use std::{io, sync::{Arc, Mutex}};

use utils::arg::parse_args;
use utils::message::{Message, MessageType};
use network::chat::{init_sock,listener,start_input_loop, wait_for_peer};
use network::hole_punching::hole_punching;
use config::config::Config;
use crypto::crypto::{get_aes_key, prepare_pubkey};


use crate::{utils::{connection::ConnectionMethod, user::color_text}};


fn main() -> Result<(), Box<dyn std::error::Error>> {

    let cli_opts = parse_args();

    let config = Config::from_cli(cli_opts)
        .expect("Invalid configuration");

    let msg = format!("[*] Runtime config: {:?}", config);
    println!("{}", color_text(&msg, "yellow"));

    let socket = init_sock(config.source_port)?;
    let rendezvous_ip = format!("{}:{}",config.server_ip,config.server_port);

    //prepare crypto message
    let crypto_socket = socket.try_clone()?;
    let (pubkey_b64, keypair) = prepare_pubkey();


    let msg = Message {
        status: MessageType::Pubkey,
        username: config.username.clone(),
        destination_port:None,
        source_port: None,
        method: config.method,
        pubkey: Some(pubkey_b64),
    };

    //send pubkey
    println!("{}",color_text("[*] Sending cryptographic pubkey to relay server","yellow"));
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

    //Wait for peer
    let peer = wait_for_peer(&ready_socket)?;
    
    //get aeskey
    let aes_key = get_aes_key(keypair,&peer);

    match config.method {
        ConnectionMethod::Hole => {
            //hole punching
            hole_punching(&socket, &peer)?;
        }
        ConnectionMethod::Upnp => {
            println!("pas la");
        }
        ConnectionMethod::Both => {
            println!("Pas encore la");
        }
    
        _ => println!("Invalid method"),
    
    }


    let recv_socket = socket.try_clone()?;
    //protecting data for threading
    let stdout = Arc::new(Mutex::new(io::stdout()));
    println!("{}",color_text("[+] Sequence complete: press ENTER or send a message", "green"));
    
    listener(
        config.username.clone(),  
        recv_socket, 
        stdout.clone(),
        aes_key
    );

    start_input_loop(socket, &peer, &aes_key);
        

    Ok(())
}
