use rand::prelude::*;
use sodiumoxide::crypto::pwhash::argon2id13;

pub fn hash(passwd: String) -> (String, argon2id13::HashedPassword) {
    sodiumoxide::init().unwrap();
    let hash = argon2id13::pwhash(
        passwd.as_bytes(),
        argon2id13::OPSLIMIT_INTERACTIVE,
        argon2id13::MEMLIMIT_INTERACTIVE,
    )
    .unwrap();
    let texthash = std::str::from_utf8(&hash.0).unwrap().to_string();
    (texthash, hash)
}

pub fn verify(hash: &str, passwd: String) -> bool {
    sodiumoxide::init().unwrap();
    let mut padded_hash = [0u8; 128];
    hash.as_bytes().iter().enumerate().for_each(|(i, val)| {
        padded_hash[i] = *val;
    });
    match argon2id13::HashedPassword::from_slice(&padded_hash) {
        Some(hp) => argon2id13::pwhash_verify(&hp, passwd.as_bytes()),
        _ => false,
    }
}

// generates a random [u8; 32];
pub fn random_redis_key() -> [u8; 32] {
    let mut key: [u8; 32] = [0u8; 32];
    rand::thread_rng().fill_bytes(&mut key);
    key
}
