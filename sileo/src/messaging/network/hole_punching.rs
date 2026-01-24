use std::io;
use std::net::{UdpSocket};
use std::sync::mpsc::Sender;

use crate::messaging::network::peer::PeerInfo;
use crate::messaging::utils::event::BackendEvent;

/// Performs UDP hole punching with a peer.
///
/// This function sends a control packet to the peer in order to
/// establish a direct UDP communication path through NAT.
/// Progress information is emitted through the backend event channel.
///
/// # Arguments
///
/// * `socket` - The UDP socket used to send the control packet.
/// * `peer` - Information about the target peer.
/// * `incoming` - A channel sender used to emit backend events.
///
/// # Returns
///
/// Returns `Ok(())` if the packet was successfully sent,
/// or an `io::Error` if the send operation fails.
pub fn hole_punching(socket: &UdpSocket, peer: &PeerInfo, incoming: &Sender<BackendEvent>) -> io::Result<()> {


    let peer_addr = format!("{}:{}",peer.peer_ip,peer.sport);
    let msg = format!("Peer address: {}", peer_addr);
    incoming.send(BackendEvent::Log(msg)).ok();

    let ctrl = "CTRL:PUNCH";
    let mut packet = Vec::new();
    packet.push(0x02);
    packet.extend_from_slice(ctrl.as_bytes());

    incoming.send(BackendEvent::Log("Punching Hole".to_string())).ok();
    socket.send_to(&packet, peer_addr)?;
    Ok(())
}