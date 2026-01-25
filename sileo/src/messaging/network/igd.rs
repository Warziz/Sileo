use std::{net::SocketAddr, time::Duration};

use igd_next::{self, SearchOptions};


pub async fn mapping_port(local_addr: SocketAddr) -> igd_next::Result<()> {

    println!("Trying to map port");
    println!("Socket Addr {:?}", local_addr);

    let ops = SearchOptions { timeout: Some(Duration::from_secs(60)), ..Default::default()};
    match igd_next::search_gateway(ops) {
        Err(ref err) => println!("Error: {err}"),
        Ok(gateway) => match gateway.get_external_ip() {
            Err(ref err) => {
                println!("There was an error! {err}");
            }
            Ok(ext_addr) => {
                println!("Local gateway: {gateway}, External ip address: {ext_addr}");
            }
        },
    }

/* 
    match gateway
        .add_any_port(
            igd_next::PortMappingProtocol::TCP,
            local_addr,
            180,
            "Testing mapping",
        )
    {
        Ok(port) => {
            println!("Port mapped successfully: {}", port);
        }
        Err(err) => {
            println!("There was an error: {}", err);
        }
    }
*/
    Ok(())
}
