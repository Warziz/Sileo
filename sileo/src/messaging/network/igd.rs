use std::{net::SocketAddr, sync::mpsc::Sender, time::Duration};

use local_ip_address::local_ip;

use crate::messaging::utils::event::BackendEvent;

extern crate igd_next as igd;


pub fn mapping_port(source_port: u16, destination_port: u16, incoming: &Sender<BackendEvent> ) -> igd_next::Result<()> {

    incoming.send(BackendEvent::Log("Trying to map port".to_string())).ok();
    
    let local_ip = local_ip().unwrap();
    
    let local_addr = SocketAddr::new(local_ip,source_port);
    let local_addr_search = SocketAddr::new(local_ip,0);
    let log = format!("Socket Addr {:?}", local_addr);
    incoming.send(BackendEvent::Log(log)).ok();

    let opts = igd::SearchOptions{
        bind_addr: local_addr_search,
        timeout: Some(Duration::from_secs(5)),
        ..Default::default()
    };

    let gateway = igd::search_gateway(opts)?;

    gateway.add_port(igd::PortMappingProtocol::UDP,destination_port, local_addr, 0, "Sileo")?;
    let msg = format!("Mapping port {} -> {}",destination_port,source_port);
    incoming.send(BackendEvent::Log(msg)).ok();

    Ok(())
          
}

pub fn remove_mapping(destination_port:u16, incoming: &Sender<BackendEvent>) -> igd_next::Result<()>{

    incoming.send(BackendEvent::Log("Trying to delete port mapping".to_string())).ok();

    let local_ip = local_ip().unwrap();
    let local_addr_search = SocketAddr::new(local_ip,0);

    let opts = igd::SearchOptions {
    bind_addr: local_addr_search, // ton IP locale exacte
    timeout: Some(Duration::from_secs(5)),
    ..Default::default()
    };
    
    let gateway =  igd::search_gateway(opts)?;
    gateway.remove_port(igd::PortMappingProtocol::UDP, destination_port)?;
    Ok(()) 

}