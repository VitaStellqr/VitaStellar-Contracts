//! Cryptographic and persistence helpers for medical records.
//!
//! This module handles encryption of record payloads and persistent storage
//! with event emission.

use soroban_sdk::{Address, Bytes, Env, String};

#[derive(Clone)]
pub struct EncryptedRecord {
    pub record_id: String,
    pub owner: Address,
    pub encrypted_content: Bytes,
    pub timestamp: u64,
    pub version: u32,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum CryptoError {
    /// Encryption failed due to invalid input.
    EncryptionFailed = 1,
    /// Storage persistence failed.
    PersistenceFailed = 2,
}

/// Encrypts a record payload using a simple XOR cipher with timestamp-based key.
///
/// **Note**: This is a simplified implementation for demonstration.
/// Production systems should use proper AES-GCM or similar authenticated encryption.
///
/// # Arguments
/// * `env` – The Soroban environment
/// * `record_id` – Unique identifier for the record
/// * `owner` – Address of the record owner
/// * `plaintext` – The unencrypted record content
/// * `timestamp` – Record creation timestamp
///
/// # Returns
/// Encrypted record wrapper containing the encrypted content
pub fn encrypt_payload(
    env: &Env,
    record_id: &String,
    owner: &Address,
    plaintext: &String,
    timestamp: u64,
) -> Result<EncryptedRecord, CryptoError> {
    // Convert plaintext to bytes
    let plaintext_bytes: Bytes = Bytes::from_slice(env, plaintext.as_bytes());

    // Generate a simple encryption key from timestamp and record_id
    // In production, use proper key derivation (HKDF) or key management service
    let key_seed = (timestamp as u32).wrapping_mul(31) ^ plaintext_bytes.len() as u32;

    // Apply simple XOR encryption (for demo; use AES-GCM in production)
    let mut encrypted = Vec::new();
    for (i, byte) in plaintext_bytes.iter().enumerate() {
        let key_byte = ((key_seed as u8).wrapping_mul((i as u8).wrapping_add(1))) ^ 0xAA;
        encrypted.push(byte ^ key_byte);
    }

    let encrypted_bytes = Bytes::from_slice(env, &encrypted);

    Ok(EncryptedRecord {
        record_id: record_id.clone(),
        owner: owner.clone(),
        encrypted_content: encrypted_bytes,
        timestamp,
        version: 1,
    })
}

/// Persists an encrypted record to storage and emits a RecordWritten event.
///
/// # Arguments
/// * `env` – The Soroban environment
/// * `encrypted` – The encrypted record to store
///
/// # Returns
/// * `Ok(())` on success
/// * `Err(CryptoError::PersistenceFailed)` if storage write fails
pub fn persist_and_emit(env: &Env, encrypted: &EncryptedRecord) -> Result<(), CryptoError> {
    // In production, store in contract persistent storage
    // Here we just emit an event indicating the record was written
    env.events().publish(
        ("MedicalRecords", "RecordWritten"),
        (
            encrypted.record_id.clone(),
            encrypted.owner.clone(),
            encrypted.timestamp,
            encrypted.version,
        ),
    );

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use soroban_sdk::String;

    #[test]
    fn test_encrypt_payload_success() {
        let env = soroban_sdk::Env::default();
        let record_id = String::from_slice(&env, "rec-001");
        let owner = Address::random(&env);
        let plaintext = String::from_slice(&env, "Confidential patient data");
        let timestamp = 1000;

        let result = encrypt_payload(&env, &record_id, &owner, &plaintext, timestamp);
        assert!(result.is_ok());

        let encrypted = result.unwrap();
        assert_eq!(encrypted.record_id, record_id);
        assert_eq!(encrypted.owner, owner);
        assert_eq!(encrypted.timestamp, timestamp);
        assert_eq!(encrypted.version, 1);
        assert!(!encrypted.encrypted_content.is_empty());
    }

    #[test]
    fn test_encrypt_payload_consistency() {
        let env = soroban_sdk::Env::default();
        let record_id = String::from_slice(&env, "rec-001");
        let owner = Address::random(&env);
        let plaintext = String::from_slice(&env, "Confidential patient data");
        let timestamp = 1000;

        let result1 = encrypt_payload(&env, &record_id, &owner, &plaintext, timestamp);
        let result2 = encrypt_payload(&env, &record_id, &owner, &plaintext, timestamp);

        assert!(result1.is_ok() && result2.is_ok());
        // Same input should produce same encrypted output
        assert_eq!(
            result1.unwrap().encrypted_content,
            result2.unwrap().encrypted_content
        );
    }

    #[test]
    fn test_persist_and_emit_success() {
        let env = soroban_sdk::Env::default();
        let record_id = String::from_slice(&env, "rec-001");
        let owner = Address::random(&env);
        let plaintext = String::from_slice(&env, "Confidential patient data");
        let timestamp = 1000;

        let encrypted = encrypt_payload(&env, &record_id, &owner, &plaintext, timestamp)
            .expect("encryption should succeed");

        let result = persist_and_emit(&env, &encrypted);
        assert!(result.is_ok());
    }

    #[test]
    fn test_encrypt_different_plaintexts_produce_different_ciphertexts() {
        let env = soroban_sdk::Env::default();
        let record_id = String::from_slice(&env, "rec-001");
        let owner = Address::random(&env);
        let plaintext1 = String::from_slice(&env, "Data A");
        let plaintext2 = String::from_slice(&env, "Data B");
        let timestamp = 1000;

        let encrypted1 = encrypt_payload(&env, &record_id, &owner, &plaintext1, timestamp)
            .expect("encryption 1 should succeed");
        let encrypted2 = encrypt_payload(&env, &record_id, &owner, &plaintext2, timestamp)
            .expect("encryption 2 should succeed");

        assert_ne!(
            encrypted1.encrypted_content,
            encrypted2.encrypted_content
        );
    }
}
