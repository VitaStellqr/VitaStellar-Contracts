#![allow(dead_code)]

use soroban_sdk::{Bytes, Env};

pub fn encrypt_payload(_env: &Env, _record_id: u64, plaintext: &str) -> Result<Bytes, ()> {
    // Simple encryption for demo - in production use proper encryption
    let bytes = plaintext.as_bytes();
    Ok(Bytes::from_slice(_env, bytes))
}
