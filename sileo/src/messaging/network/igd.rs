use std::net::SocketAddr;

extern crate igd_next as igd;


pub fn mapping_port(local_addr: SocketAddr,source_port: Option<u16>, destination_port: Option<u16> ) -> igd_next::Result<()> {

    println!("Trying to map port");
    println!("Socket Addr {:?}", local_addr);

    let source_port = source_port.unwrap();
    let destination_port = destination_port.unwrap();
    let gateway = igd::search_gateway(Default::default())?;
    let mut local_addr = local_addr;

    local_addr.set_port(source_port);
    gateway.add_port(igd::PortMappingProtocol::TCP,destination_port, local_addr, 60, "Sileo")?;
    Ok(())
          
}
