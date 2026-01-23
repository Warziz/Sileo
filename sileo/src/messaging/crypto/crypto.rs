use x25519_dalek::{EphemeralSecret, PublicKey, SharedSecret};
use base64::{engine::general_purpose, Engine as _};

use crate::{messaging::crypto::kdf::derive_aes_key, messaging::network::peer::PeerInfo};

/// Holds an ephemeral Diffie-Hellmain key pair
pub struct KeyPair {
    /// Ephemeral secret
    pub secret: EphemeralSecret,
    /// Corresponding public key
    pub public: PublicKey,
}

/// Generates a new ephemeral Diffie-Hellman key pair.
///
/// # Returns
///
/// A `KeyPair` containing a randomly generated private key
/// and its associated public key.
fn generate_keypair() -> KeyPair {
    let secret = EphemeralSecret::random();
    let public = PublicKey::from(&secret);

    KeyPair {secret,public}
}

/// Derives a shared secret using Diffie-Hellman key agreement.
///
/// # Arguments
///
/// * `my_secret` - The local ephemeral private key.
/// * `peer_public` - The peer's public key.
///
/// # Returns
///
/// The derived shared secret.
pub fn derive_shared_key(my_secret: EphemeralSecret, peer_public: &PublicKey) -> SharedSecret {
    my_secret.diffie_hellman(peer_public)
}

/// Generates a key pair and encodes the public key in Base64.
///
/// # Returns
///
/// A tuple containing:
/// * The Base64-encoded public key
/// * The generated `KeyPair`
pub fn prepare_pubkey () -> (String, KeyPair) {
    let keypair = generate_keypair();

    let pubkey_b64 = general_purpose::STANDARD.encode(
        keypair.public.as_bytes()
    );

    return (pubkey_b64, keypair);
}

/// Derives an AES-256 key from a Diffie-Hellman shared secret.
///
/// # Arguments
///
/// * `keypair` - The local ephemeral key pair.
/// * `peer` - Information about the peer, including its public key.
///
/// # Returns
///
/// A 32-byte AES-256 key.
pub fn get_aes_key (keypair: KeyPair, peer: &PeerInfo) -> [u8;32] {

    let shared = derive_shared_key(keypair.secret, &peer.peer_pubkey);
    let aes_key = derive_aes_key(shared);

    return aes_key;
}