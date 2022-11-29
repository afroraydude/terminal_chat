extern crate common;

use std::io::Write;
use common::message::{Message, MessageType};
use log::{debug, LevelFilter};
use rand::Rng;
use simplelog::{SimpleLogger, Config};
use common::crypt;

fn main() {
    SimpleLogger::init(LevelFilter::Debug, Config::default()).unwrap();
    // create two key pairs
    let private_key1 = crypt::create_private_key();
    let public_key1 = crypt::create_public_key(private_key1.clone());
    let private_key2 = crypt::create_private_key();
    let public_key2 = crypt::create_public_key(private_key2.clone());

    // create shared key from the two key pairs
    let shared_key1 = crypt::create_shared_key(private_key1, public_key2);
    let shared_key2 = crypt::create_shared_key(private_key2, public_key1);

    // verify that the shared keys are the same
    if shared_key1 == shared_key2 {
        debug!("Shared keys are the same");
    } else {
        debug!("Shared keys are not the same");
    }

    let original_message = "Hello, world!".to_string();

    // encrypt the message
    let encrypted_message = crypt::encrypt_data(original_message.clone().as_bytes().to_vec(), shared_key1.clone());
    let decrypted_message = crypt::decrypt_data(encrypted_message.clone(), shared_key2.clone());

    // print the message
    debug!("Original message: {}", original_message);
    debug!("Encrypted message: {:?}", encrypted_message);
    debug!("Decrypted message: {:?}", decrypted_message);
    debug!("Decrypted message: {}", String::from_utf8(decrypted_message.clone()).unwrap());

    // check if the message is the same
    if original_message == String::from_utf8(decrypted_message.clone()).unwrap() {
        debug!("Messages are the same");
    } else {
        debug!("Messages are not the same");
    }

    let list_of_chars = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ1234567890!@#$%^&*()_+{}|:<>?[];',./`~".chars().collect::<Vec<char>>();
    let mut compute_times: Vec<u128> = Vec::new();
    // create a message of random bytes
    for i in 0..1000 {
        // start stopwatch
        let start = std::time::Instant::now();

        let mut random_message = Vec::new();

        let message_length = rand::thread_rng().gen_range(1..1000);

        for _ in 0..message_length {
            random_message.push(list_of_chars[rand::random::<usize>() % list_of_chars.len()] as u8);
        }

        // encrypt the message
        let encrypted_message = crypt::encrypt_data(random_message.clone(), shared_key1.clone());
        let decrypted_message = crypt::decrypt_data(encrypted_message.clone(), shared_key2.clone());

        debug!("Test {} complete", i);

        // stop stopwatch
        let duration = start.elapsed();
        compute_times.push(duration.as_millis());
    }

    let mut sum = 0;
    for time in compute_times.clone() {
        sum += time;
    }
    debug!("Average time: {}ms", sum / compute_times.len() as u128);
    debug!("Total time: {}ms", sum);
    debug!("Total time: {}s", sum / 1000);
}