use hkdf::Hkdf;
use sha2::Sha256;
use x25519_dalek::SharedSecret;

/// Derives an AES-256 key from a Diffie-Hellman shared secret using HKDF-SHA256.
///
/// # Arguments
///
/// * `shared` - The Diffie-Hellman shared secret.
///
/// # Returns
///
/// A 32-byte key suitable for use with AES-256.
pub fn derive_aes_key(shared: SharedSecret) -> [u8;32]{
    let hk = Hkdf::<Sha256>::new(None,shared.as_bytes());

    let mut key = [0u8; 32];
    hk.expand(b"sileo-session-key", &mut key)
        .expect("HKDF expand failed");

    return key;
}