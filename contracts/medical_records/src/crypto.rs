#![allow(dead_code)]

use soroban_sdk::{Bytes, Env};

pub fn encrypt_payload(env: &Env, _record_id: u64, plaintext: &str) -> Result<Bytes, ()> {
    Ok(Bytes::from_slice(env, plaintext.as_bytes()))
}
