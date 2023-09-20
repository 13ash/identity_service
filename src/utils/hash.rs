use argon2rs::defaults::{KIB, LANES, PASSES};
use argon2rs::verifier::Encoded;
use argon2rs::{Argon2, Variant};
use rand::{thread_rng, Rng};
use std::error::Error;
use std::result::Result;

pub struct HashParams {
    pub hashed_data: String,
    pub random_salt: String,
    pub local_salt: String,
}

pub fn hash_password(pwd: &str, local_salt: &str) -> Result<HashParams, Box<dyn Error>> {
    let argon2 = Argon2::new(PASSES, LANES, KIB, Variant::Argon2d)?;
    let random_salt: String = thread_rng()
        .sample_iter(&rand::distributions::Alphanumeric)
        .take(32)
        .map(char::from)
        .collect();

    let random_salt_hash = Encoded::new(
        argon2,
        random_salt.as_bytes(),
        local_salt.as_bytes(),
        b"",
        b"",
    )
    .to_u8();
    let random_salt_hash_storable_encoding = String::from_utf8(random_salt_hash)?;

    let a2 = Argon2::new(PASSES, LANES, KIB, Variant::Argon2d)?;
    let data_hash = Encoded::new(
        a2,
        pwd.as_bytes(),
        random_salt_hash_storable_encoding.as_bytes(),
        b"",
        b"",
    )
    .to_u8();

    Ok(HashParams {
        hashed_data: String::from_utf8(data_hash).unwrap(),
        random_salt,
        local_salt: local_salt.to_string(),
    })
}

pub fn verify_hashed_data(
    stored_params: &HashParams,
    incoming_pwd: &str,
) -> Result<bool, Box<dyn Error>> {
    let argon2 = Argon2::new(PASSES, LANES, KIB, Variant::Argon2d)?;

    let stored_salt_hash = Encoded::new(
        argon2,
        stored_params.random_salt.as_bytes(),
        stored_params.local_salt.as_bytes(),
        b"",
        b"",
    )
    .to_u8();
    let stored_salt_hash_storable_encoding = String::from_utf8(stored_salt_hash)?;

    let a2 = Argon2::new(PASSES, LANES, KIB, Variant::Argon2d)?;

    let incoming_data_hash = Encoded::new(
        a2,
        incoming_pwd.as_bytes(),
        stored_salt_hash_storable_encoding.as_bytes(),
        b"",
        b"",
    )
    .to_u8();

    let incoming_data_hash_string = String::from_utf8(incoming_data_hash).unwrap();

    Ok(stored_params.hashed_data == incoming_data_hash_string)
}
