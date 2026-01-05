use std::io;
use std::net::{UdpSocket};

use crate::utils::user::color_text;

pub fn hole_punching(socket: &UdpSocket, peer_addr: &str) -> io::Result<()> {

    let msg = format!("[*] Peer address: {}", peer_addr);
    println!("{}",color_text(&msg, "yellow"));

    let ctrl = "CTRL:PUNCH";
    let mut packet = Vec::new();
    packet.push(0x02);
    packet.extend_from_slice(ctrl.as_bytes());

    println!("{}", color_text("[!] Punching Hole", "magenta"));
    socket.send_to(&packet, peer_addr)?;
    Ok(())
}