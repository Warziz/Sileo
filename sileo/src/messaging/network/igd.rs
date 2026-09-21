use std::{net::SocketAddr, sync::mpsc::Sender};

use local_ip_address::local_ip;

use crate::messaging::utils::event::BackendEvent;

extern crate igd_next as igd;


pub fn mapping_port(source_port: Option<u16>, destination_port: Option<u16>, incoming: &Sender<BackendEvent> ) -> igd_next::Result<()> {

    println!("Trying to map port");
    
    let local_ip = local_ip().unwrap();
    let source_port = source_port.unwrap();
    
    let local_addr = SocketAddr::new(local_ip,source_port);
    println!("Socket Addr {:?}", local_addr);

    
    let destination_port = destination_port.unwrap();
    let gateway = igd::search_gateway(Default::default())?;
    let mut local_addr = local_addr;

    local_addr.set_port(source_port);
    gateway.add_port(igd::PortMappingProtocol::TCP,destination_port, local_addr, 60, "Sileo")?;
    let msg = format!("Mapping port {} -> {}",destination_port,source_port);
    incoming.send(BackendEvent::Log(msg)).ok();

    Ok(())
          
}
