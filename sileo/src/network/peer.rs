use x25519_dalek::PublicKey;

#[derive(Debug)]
pub struct PeerInfo {
    pub peer_ip: String,
    pub sport: u16,
    pub peer_pubkey: PublicKey,
    pub peer_username: String,
}