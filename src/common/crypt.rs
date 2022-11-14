use crypto::aes;
use crypto::aes::KeySize;
use crypto::buffer::{RefReadBuffer, RefWriteBuffer};
use crypto::symmetriccipher::{Decryptor, Encryptor};
use log::debug;
use rand_core::RngCore;
use x25519_dalek::{PublicKey, StaticSecret};

fn convert_vec_u8(v: Vec<u8>) -> [u8; 32] {
    /*let mut vec = [0u8; 32];
    for (i, byte) in v.iter().enumerate() {
        vec[i] = *byte;
    }
    vec*/
    let mut vec = [0u8; 32];
    vec.copy_from_slice(&v);
    vec
}

pub fn create_private_key() -> StaticSecret {
    StaticSecret::new(rand_core::OsRng)
}

pub fn serialize_private_key(secret: StaticSecret) -> Vec<u8> {
    secret.to_bytes().to_vec()
}

pub fn deserialize_private_key(secret: Vec<u8>) -> StaticSecret {
    let mut secret_bytes = convert_vec_u8(secret);
    StaticSecret::from(secret_bytes)
}

pub fn serialize_public_key(public_key: PublicKey) -> Vec<u8> {
    public_key.as_bytes().to_vec()
}

pub fn deserialize_public_key(public_key: Vec<u8>) -> PublicKey {
    let public_key_bytes = convert_vec_u8(public_key);
    PublicKey::from(public_key_bytes)
}

pub fn create_shared_key(private_key: StaticSecret, public_key: PublicKey) -> Vec<u8> {
    private_key.diffie_hellman(&public_key).as_bytes().to_vec()
}

pub fn create_public_key(private_key: StaticSecret) -> PublicKey {
   PublicKey::from(&private_key)
}

pub fn encrypt_message(message: Vec<u8>, shared_key: Vec<u8>) -> Vec<u8> {

}

pub fn decrypt_message(encrypted_message: Vec<u8>, shared_key: Vec<u8>) -> Vec<u8> {

}
