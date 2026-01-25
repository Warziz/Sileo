use x25519_dalek::PublicKey;


/// Holds all the useful information about the peer
#[derive(Debug)]
pub struct PeerInfo {
    pub peer_ip: String,        // His IP
    pub sport: u16,             // The port we can contact him 
    pub peer_pubkey: PublicKey, // Public Key
    pub peer_username: String,  // His username
}