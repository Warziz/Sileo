use aes_gcm::{
    Aes256Gcm,
    Key,
    Nonce,
    aead::{Aead, OsRn, rand_core::RngCore},
};

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
