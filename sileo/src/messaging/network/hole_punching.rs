use std::io;
use std::net::{UdpSocket};
use std::sync::mpsc::Sender;

use crate::messaging::network::peer::PeerInfo;
use crate::messaging::utils::event::BackendEvent;

pub fn hole_punching(socket: &UdpSocket, peer: &PeerInfo, incoming: &Sender<BackendEvent>) -> io::Result<()> {


    let peer_addr = format!("{}:{}",peer.peer_ip,peer.sport);
    let msg = format!("Peer address: {}", peer_addr);
    incoming.send(BackendEvent::Log(msg));

    let ctrl = "CTRL:PUNCH";
    let mut packet = Vec::new();
    packet.push(0x02);
    packet.extend_from_slice(ctrl.as_bytes());

    incoming.send(BackendEvent::Log("Punching Hole".to_string())).ok();
    socket.send_to(&packet, peer_addr)?;
    Ok(())
}