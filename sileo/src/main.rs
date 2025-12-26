mod utils;
mod config;

use std::{io, sync::{Arc, Mutex}};

use utils::arg::parse_args;
use config::config::Config;
use utils::network::{init_sock,listener,start_input_loop};

fn main() -> io::Result<()> {

    let cli_opts = parse_args();

    let config = Config::from_cli(cli_opts)
        .expect("Invalid configuration");

    println!("Runtime config: {:?}", config);

    let socket = init_sock(config.source_port)?;
    let recv_socket = socket.try_clone()?;

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
