use x25519_dalek::{EphemeralSecret, PublicKey, SharedSecret};
use base64::{engine::general_purpose, Engine as _};

pub struct KeyPair {
    pub secret: EphemeralSecret,
    pub public: PublicKey,
}


fn generate_keypair() -> KeyPair {
    let secret = EphemeralSecret::random();
    let public = PublicKey::from(&secret);

    KeyPair {secret,public}
}


pub fn derive_shared_key(my_secret: EphemeralSecret, peer_public: &PublicKey) -> SharedSecret {
    my_secret.diffie_hellman(peer_public)
}

pub fn prepare_pubkey () -> (String, EphemeralSecret) {
    let keypair = generate_keypair();

    let pubkey_b64 = general_purpose::STANDARD.encode(
        keypair.public.as_bytes()
    );

    return (pubkey_b64,keypair.secret);
}