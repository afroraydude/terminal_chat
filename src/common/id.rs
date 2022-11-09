use rand::Rng;
use std::time::SystemTime;
use std::time::UNIX_EPOCH;
use chrono::prelude::*;

pub enum IdType {
    Unknown,
    Message,
    User,
}

pub fn create_id(id_type: IdType) -> u64 {
    // create a random id based on three factors
    // 1. seconds since Jan 1, 2000 00:00:00 UTC (no more than 48 bits)
    // 2. the id type (2 bits)
    // 3. a random number (14 bits)

    let mut id_string = String::new();

    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
    let timestamp = timestamp - 946684800; // Jan 1, 2000 00:00:00 UTC
                                           // convert to binary then to string
    let timestamp = format!("{:b}", timestamp);
    id_string.push_str(&timestamp);

    // do the same for the id type
    let id_type: u8 = match id_type {
        IdType::Unknown => 0,
        IdType::Message => 1,
        IdType::User => 2,
    };

    let mut id_type = format!("{:b}", id_type);

    // force the id type to be 2 bit
    while id_type.len() < 2 {
        id_type.insert(0, '0');
    }

    // add the id type to the id string
    id_string.push_str(&id_type);

    // add a random number to the id string
    let random_number = rand::thread_rng().gen_range(0..16384);
    let mut random_number = format!("{:b}", random_number);

    // force the random number to be 14 bits
    while random_number.len() < 14 {
        // add a 0 to the front of the string
        random_number.insert(0, '0');
    }

    id_string.push_str(&random_number);

    // convert the id string to a u64
    let id = u64::from_str_radix(&id_string, 2).unwrap();

    id
}

pub fn to_timestamp(id: u64) -> u64 {
    // convert the id to a string
    let id = format!("{:b}", id);

    // get the timestamp from the id by ommiiing the last 16 bits
    let timestamp = &id[0..id.len() - 16];

    // convert the timestamp to a u64
    let timestamp = u64::from_str_radix(&timestamp, 2).unwrap();

    // add the Jan 1, 2000 00:00:00 UTC timestamp
    let timestamp = timestamp + 946684800;

    timestamp
}

pub fn to_timestamp_string(id: u64) -> String {
    // convert the id to a string
    let id = format!("{:b}", id);

    // get the timestamp from the id by ommiiing the last 16 bits
    let timestamp = &id[0..id.len() - 16];

    // convert the timestamp to a u64
    let timestamp = u64::from_str_radix(&timestamp, 2).unwrap();

    // add the Jan 1, 2000 00:00:00 UTC timestamp
    let timestamp = timestamp + 946684800;

    let native = NaiveDateTime::from_timestamp(timestamp as i64, 0);

    let datetime: DateTime<Utc> = DateTime::from_utc(native, Utc);

    // convert to local time
    let datetime: DateTime<Local> = DateTime::from(datetime);

    datetime.format("%Y-%m-%d %H:%M:%S").to_string()
}

pub fn to_formatted_timestamp(id: u64, format: &str) -> String {
    // convert the id to a string
    let id = format!("{:b}", id);

    // get the timestamp from the id by ommiiing the last 16 bits
    let timestamp = &id[0..id.len() - 16];

    // convert the timestamp to a u64
    let timestamp = u64::from_str_radix(&timestamp, 2).unwrap();

    // add the Jan 1, 2000 00:00:00 UTC timestamp
    let timestamp = timestamp + 946684800;

    let native = NaiveDateTime::from_timestamp(timestamp as i64, 0);

    let datetime: DateTime<Utc> = DateTime::from_utc(native, Utc);

    // convert to local time
    let datetime: DateTime<Local> = DateTime::from(datetime);

    datetime.format(format).to_string()
}
