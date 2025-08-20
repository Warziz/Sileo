import os
import secrets
import hashlib


from cryptography.hazmat.primitives.ciphers.aead import AESGCM


class DiffieHellman:
    def __init__(self, p, generator=2, key_length=2048):
        self.min_key_length = 540
        self.default_key_length = key_length
        self.default_generator = generator

        valid_generators = [2, 3, 5, 7]
        if self.default_generator not in valid_generators:
            raise ValueError("Invalid generator!")
        if self.default_key_length < self.min_key_length:
            raise ValueError("Invalid key length!")

        self.p = p
        self.private_key = self.gen_private_key(self.p)

    def gen_private_key(self, p):
        return secrets.randbelow(p - 2) + 2

    def get_public_key(self):
        return pow(self.default_generator, self.private_key, self.p)

    def gen_shared_secret(self, other_key):
        return pow(other_key, self.private_key, self.p)

    def derive_shared_key(self, other_key):
        self.shared_secret = self.gen_shared_secret(other_key)
        shared_secret_bytes = self.shared_secret.to_bytes(
            (self.shared_secret.bit_length() + 7) // 8, byteorder="big"
        )
        self.key = hashlib.sha256(shared_secret_bytes).digest()

    def get_key(self):
        return self.key


class Cipher:
    @staticmethod
    def encrypt_message(aes_key: bytes, message: str) -> bytes:
        aesgcm = AESGCM(aes_key)
        nonce = os.urandom(12)
        ciphertext = aesgcm.encrypt(nonce, message.encode(), None)
        return nonce + ciphertext

    @staticmethod
    def decrypt_message(aes_key: bytes, data: bytes) -> str:
        aesgcm = AESGCM(aes_key)
        nonce = data[:12]
        ciphertext = data[12:]
        plaintext = aesgcm.decrypt(nonce, ciphertext, None)
        return plaintext.decode()
