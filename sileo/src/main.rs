mod utils;
mod config;

use std::{io, sync::{Arc, Mutex}};

use utils::arg::parse_args;
use config::config::Config;
use utils::network::{init_sock,listener,start_input_loop};
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
    pub msg_type: MessageType,
    pub username: String,
    pub destination_port: Option<u16>,
    pub method: ConnectionMethod, 
}

fn main() -> io::Result<()> {

    let cli_opts = parse_args();

    let config = Config::from_cli(cli_opts)
        .expect("Invalid configuration");

    println!("Runtime config: {:?}", config);

    let socket = init_sock(config.source_port)?;
    let recv_socket = socket.try_clone()?;

    let msg = Message {
        msg_type: MessageType::Check,
        username: config.username.clone(),
        destination_port: None,
        method: config.method,
    };

    let serialized_msg = serde_json::to_string(&msg)?;
    let check_socket = socket.try_clone()?;

    //sending check message
    println!("{}", serialized_msg);
    let rendezvous_ip = format!("{}:{}",config.server_ip,config.server_port);
    check_socket.send_to(serialized_msg.as_bytes(), rendezvous_ip).unwrap();

    //sending crypto message
    /*TO DO later*/

    //sending ready message

    let msg = Message {
        msg_type:MessageType::Ready,
        username: config.username.clone(),
        destination_port: None,
        method: config.method,
    };

    let serialized_msg = serde_json::to_string(&msg)?;
    println!("{}",serialized_msg);


    //protecting data for threading
    let stdout = Arc::new(Mutex::new(io::stdout()));

    listener(
        config.username.clone(),  
        recv_socket, 
        stdout.clone()
    );

    let peer_addr = "127.0.0.1:50002";
    start_input_loop(socket, peer_addr);

    Ok(())
}
