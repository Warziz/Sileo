use std::io;
use std::net::{UdpSocket};
use std::sync::mpsc::Sender;

use crate::messaging::network::peer::PeerInfo;
use crate::messaging::utils::user::color_text;

pub fn hole_punching(socket: &UdpSocket, peer: &PeerInfo, incoming: &Sender<String>) -> io::Result<()> {


    let peer_addr = format!("{}:{}",peer.peer_ip,peer.sport);
    let msg = format!("[INFO] Peer address: {}", peer_addr);
    println!("{}",color_text(&msg, "yellow"));

    let ctrl = "CTRL:PUNCH";
    let mut packet = Vec::new();
    packet.push(0x02);
    packet.extend_from_slice(ctrl.as_bytes());

    incoming.send(color_text("[INFO] Punching Hole", "magenta")).ok();
    socket.send_to(&packet, peer_addr)?;
    Ok(())
}