use std::env::args;
use common::crypt::{encrypt_data, decrypt_data, deserialize_private_key, deserialize_public_key, create_shared_key};

extern crate common;

fn main() {
    // load args
    // --encrypt encrypts a base64 encoded payload
    // --decrypt decrypts a base64 encoded payload
    // --key the key to use for encryption/decryption, base64 encoded
    // --payload the payload to encrypt/decrypt, base64 encoded
    // --generate creates a diffie-hellman shared key

    // get the args
    let args: Vec<String> = std::env::args().collect();

    // convert arg[1] to a &str
    let arg1 = &args[1];

    let choice = arg1.as_str();

    // check if we are encrypting or decrypting
    match choice {
        "--encrypt" => {
            // get the key
            let key = base64::decode(&args[3]).unwrap();
            // get the payload
            let payload = base64::decode(&args[5]).unwrap();
            // encrypt the payload
            let encrypted = encrypt_data(payload, key);
            // print the encrypted payload
            println!("{}", base64::encode(encrypted));
        }
        "--decrypt" => {
            // get the key
            let key = base64::decode(&args[3]).unwrap();
            // get the payload
            let payload = base64::decode(&args[5]).unwrap();
            // decrypt the payload
            let decrypted = decrypt_data(payload, key);
            // print the decrypted payload
            println!("{}", base64::encode(decrypted));
        }
        "--generate" => {
            let priv_key_b64 = args[3].clone();
            let pub_key_b64 = args[5].clone();

            let priv_key = deserialize_private_key(base64::decode(priv_key_b64).unwrap());
            let pub_key = deserialize_public_key(base64::decode(pub_key_b64).unwrap());

            let shared_key = create_shared_key(priv_key, pub_key);

            println!("{}", base64::encode(shared_key));
        }
        _ => {
            println!("Invalid choice");
        }
    }
}

fn help_menu() {

    let literal = r#"Usage: yutcrypt [options]
        --encrypt [data]                        Encrypts the data
        --decrypt [data]                        Decrypts the data
        --key [key]                             The shared key to use for the data encryption/decryption
        --payload [payload]                     The payload to encrypt/decrypt
        --generate [private key] [public key]   Generates a shared key using diffie-helman and the key pair
    "#;

    println!("{}", literal);
}

