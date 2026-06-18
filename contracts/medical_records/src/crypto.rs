use soroban_sdk::{Bytes, Env, Vec};

pub fn encrypt_payload(env: &Env, record_id: u64, plaintext: &str) -> Result<Vec<u8>, ()> {
    let mut encrypted = Vec::new(env);
    // Simple XOR encryption for demo (replace with real encryption in production)
    let key = record_id.to_le_bytes();
    let plaintext_bytes = plaintext.as_bytes();
    
    for (i, byte) in plaintext_bytes.iter().enumerate() {
        let key_byte = key[i % key.len()];
        encrypted.push_back(byte ^ key_byte);
    }
    
    Ok(encrypted)
}
