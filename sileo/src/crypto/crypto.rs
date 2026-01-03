use x25519_dalek::{EphemeralSecret, PublicKey, SharedSecret};

pub struct KeyPair {
    pub secret: EphemeralSecret,
    pub public: PublicKey,
}


pub fn generate_keypair() -> KeyPair {
    let secret = EphemeralSecret::random();
    let public = PublicKey::from(&secret);

    KeyPair {secret,public}
}


pub fn derive_shared_key(my_secret: EphemeralSecret, peer_public: &PublicKey) -> SharedSecret {
    my_secret.diffie_hellman(peer_public)
}