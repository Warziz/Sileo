use aes_gcm::{
    Aes256Gcm,
    Key,
    Nonce,
    aead::{Aead, KeyInit, OsRng, rand_core::RngCore},
};

/// Encrypts a plaintext using AES-256-GCM.
///
/// # Arguments
///
/// * `key_bytes` - A reference to a 32-byte AES-256 key.
/// * `plaintext` - The data to encrypt, provided as a byte slice.
///
/// # Returns
///
/// A tuple containing:
/// * The ciphertext as a `Vec<u8>`
/// * The randomly generated 12-byte nonce used for encryption

pub fn encrypt(key_bytes: &[u8;32], plaintext: &[u8]) -> (Vec<u8>, [u8; 12]){

    let key = Key::<Aes256Gcm>::from_slice(key_bytes);
    let cipher = Aes256Gcm::new(key);

    let mut nonce_bytes = [0u8; 12];
    OsRng.fill_bytes(&mut nonce_bytes);
    let nonce = Nonce::from_slice(&nonce_bytes);

    let ciphertext = cipher
        .encrypt(nonce, plaintext)
        .expect("encryption failed");

    (ciphertext, nonce_bytes)
}


/// Decrypts a ciphertext using AES-256-GCM.
///
/// # Arguments
///
/// * `key_bytes` - A reference to the 32-byte AES-256 key.
/// * `ciphertext` - The encrypted data to decrypt.
/// * `nonce_bytes` - The 12-byte nonce that was used during encryption.
///
/// # Returns
///
/// The decrypted plaintext as a `Vec<u8>`.

pub fn decrypt(
    key_bytes: &[u8; 32],
    ciphertext: &[u8],
    nonce_bytes: &[u8; 12],
) -> Vec<u8> {
    let key = Key::<Aes256Gcm>::from_slice(key_bytes);
    let cipher = Aes256Gcm::new(key);

    let nonce = Nonce::from_slice(nonce_bytes);

    cipher
        .decrypt(nonce, ciphertext)
        .expect("decryption failed")
}
