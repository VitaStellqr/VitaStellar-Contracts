//! # Medical Records Contract
//!
//! A secure contract for managing encrypted medical records on Soroban.
//! Implements Issue #65 refactoring: splits the long `write_record` function
//! into focused, testable helpers.
//!
//! ## Architecture
//! - **validation**: Input validation for record fields
//! - **crypto**: Encryption and persistent storage with event emission
//!
//! ## Security
//! All medical records are encrypted before storage. The contract emits events
//! for off-chain indexing while keeping sensitive data on-chain in encrypted form.

#![no_std]

mod crypto;
mod validation;

use soroban_sdk::{
    contract, contracterror, contractimpl, contracttype, symbol_short, Address, Bytes, Env,
    String,
};

pub use crypto::{encrypt_payload, persist_and_emit, CryptoError, EncryptedRecord};
pub use validation::{validate_record_fields, ValidationError};

// ── Types ─────────────────────────────────────────────────────────────────────

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[contracterror]
#[repr(u32)]
pub enum RecordError {
    /// Initialization has not occurred.
    NotInitialized = 1,
    /// Already initialized.
    AlreadyInitialized = 2,
    /// Unauthorized: caller is not the record owner.
    Unauthorized = 3,
    /// Validation failed (see ValidationError variants).
    ValidationFailed = 4,
    /// Encryption or persistence failed (see CryptoError variants).
    CryptoFailed = 5,
    /// Record not found.
    RecordNotFound = 6,
}

#[derive(Clone, PartialEq, Eq)]
#[contracttype]
pub enum DataKey {
    Admin,
    /// Mapping of record_id -> owner address
    RecordOwner(String),
}

#[contracttype]
pub struct RecordMetadata {
    pub record_id: String,
    pub owner: Address,
    pub timestamp: u64,
    pub record_type: String,
}

// ── Contract ──────────────────────────────────────────────────────────────────

#[contract]
pub struct MedicalRecords;

#[contractimpl]
impl MedicalRecords {
    /// Initialize the contract with an admin address.
    pub fn initialize(env: Env, admin: Address) -> Result<(), RecordError> {
        if env.storage().instance().has(&DataKey::Admin) {
            return Err(RecordError::AlreadyInitialized);
        }
        admin.require_auth();
        env.storage().instance().set(&DataKey::Admin, &admin);
        Ok(())
    }

    /// Write a new encrypted medical record.
    ///
    /// This function coordinates the record lifecycle:
    /// 1. Validates input fields
    /// 2. Encrypts the payload
    /// 3. Persists to storage and emits events
    ///
    /// Refactored from ~120 lines into ~20 lines via helper functions.
    /// See Issue #65 for details.
    ///
    /// # Arguments
    /// * `patient_id` – Unique patient identifier
    /// * `record_type` – Type of record (e.g., "diagnosis", "prescription")
    /// * `content` – The unencrypted record content
    /// * `timestamp` – Record creation timestamp
    ///
    /// # Returns
    /// * `Ok(())` on success
    /// * `Err(RecordError)` on failure (validation, encryption, or persistence)
    pub fn write_record(
        env: Env,
        owner: Address,
        patient_id: String,
        record_type: String,
        content: String,
        timestamp: u64,
    ) -> Result<(), RecordError> {
        owner.require_auth();

        // 1. Validate input fields
        validate_record_fields(&env, &patient_id, &record_type, &content, timestamp)
            .map_err(|_| RecordError::ValidationFailed)?;

        // 2. Generate unique record ID
        let record_id = Self::generate_record_id(&env, &patient_id, timestamp);

        // 3. Encrypt the payload
        let encrypted = encrypt_payload(&env, &record_id, &owner, &content, timestamp)
            .map_err(|_| RecordError::CryptoFailed)?;

        // 4. Persist to storage and emit event
        persist_and_emit(&env, &encrypted).map_err(|_| RecordError::CryptoFailed)?;

        // 5. Record owner mapping
        env.storage()
            .persistent()
            .set(&DataKey::RecordOwner(record_id), &owner);

        Ok(())
    }

    /// Retrieve metadata for a record (does NOT return encrypted content).
    ///
    /// # Arguments
    /// * `record_id` – The unique record identifier
    ///
    /// # Returns
    /// * `Ok(RecordMetadata)` if the record exists
    /// * `Err(RecordError::RecordNotFound)` if not found
    pub fn get_record_metadata(env: Env, record_id: String) -> Result<RecordMetadata, RecordError> {
        env.storage()
            .persistent()
            .get::<DataKey, Address>(&DataKey::RecordOwner(record_id.clone()))
            .ok_or(RecordError::RecordNotFound)
            .map(|owner| RecordMetadata {
                record_id,
                owner,
                timestamp: env.ledger().timestamp(),
                record_type: String::from_slice(&env, "unknown"),
            })
    }

    // ── Helpers ───────────────────────────────────────────────────────────────

    /// Generate a unique record ID from patient ID and timestamp.
    fn generate_record_id(env: &Env, patient_id: &String, timestamp: u64) -> String {
        // Simple ID generation: concatenate patient_id with timestamp
        // In production, use a counter or proper UUID generation
        let ts_str = String::from_slice(env, &timestamp.to_string());
        let separator = String::from_slice(env, "-");
        
        let mut combined = String::new();
        combined = combined.concat(patient_id);
        combined = combined.concat(&separator);
        combined = combined.concat(&ts_str);
        
        combined
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_initialize_success() {
        let env = soroban_sdk::Env::default();
        let admin = Address::random(&env);

        let result = MedicalRecords::initialize(env.clone(), admin.clone());
        assert!(result.is_ok());
    }

    #[test]
    fn test_initialize_already_initialized() {
        let env = soroban_sdk::Env::default();
        let admin = Address::random(&env);

        MedicalRecords::initialize(env.clone(), admin.clone()).expect("first init should succeed");

        let result = MedicalRecords::initialize(env.clone(), admin.clone());
        assert_eq!(result, Err(RecordError::AlreadyInitialized));
    }

    #[test]
    fn test_write_record_success() {
        let env = soroban_sdk::Env::default();
        let owner = Address::random(&env);
        let admin = Address::random(&env);

        MedicalRecords::initialize(env.clone(), admin).expect("init should succeed");

        let patient_id = String::from_slice(&env, "patient-123");
        let record_type = String::from_slice(&env, "diagnosis");
        let content = String::from_slice(&env, "Patient has hypertension");
        let timestamp = env.ledger().timestamp() - 3600;

        let result = MedicalRecords::write_record(
            env.clone(),
            owner.clone(),
            patient_id,
            record_type,
            content,
            timestamp,
        );

        assert!(result.is_ok());
    }

    #[test]
    fn test_write_record_validation_failure() {
        let env = soroban_sdk::Env::default();
        let owner = Address::random(&env);
        let admin = Address::random(&env);

        MedicalRecords::initialize(env.clone(), admin).expect("init should succeed");

        let patient_id = String::from_slice(&env, "");
        let record_type = String::from_slice(&env, "diagnosis");
        let content = String::from_slice(&env, "Patient has hypertension");
        let timestamp = env.ledger().timestamp() - 3600;

        let result = MedicalRecords::write_record(
            env.clone(),
            owner.clone(),
            patient_id,
            record_type,
            content,
            timestamp,
        );

        assert_eq!(result, Err(RecordError::ValidationFailed));
    }

    #[test]
    fn test_write_record_invalid_timestamp() {
        let env = soroban_sdk::Env::default();
        let owner = Address::random(&env);
        let admin = Address::random(&env);

        MedicalRecords::initialize(env.clone(), admin).expect("init should succeed");

        let patient_id = String::from_slice(&env, "patient-123");
        let record_type = String::from_slice(&env, "diagnosis");
        let content = String::from_slice(&env, "Patient has hypertension");
        let timestamp = env.ledger().timestamp() + 3600; // future timestamp

        let result = MedicalRecords::write_record(
            env.clone(),
            owner.clone(),
            patient_id,
            record_type,
            content,
            timestamp,
        );

        assert_eq!(result, Err(RecordError::ValidationFailed));
    }

    #[test]
    fn test_get_record_metadata_success() {
        let env = soroban_sdk::Env::default();
        let owner = Address::random(&env);
        let admin = Address::random(&env);

        MedicalRecords::initialize(env.clone(), admin).expect("init should succeed");

        let patient_id = String::from_slice(&env, "patient-123");
        let record_type = String::from_slice(&env, "diagnosis");
        let content = String::from_slice(&env, "Patient has hypertension");
        let timestamp = env.ledger().timestamp() - 3600;

        MedicalRecords::write_record(
            env.clone(),
            owner.clone(),
            patient_id,
            record_type,
            content,
            timestamp,
        )
        .expect("write should succeed");

        let record_id = String::from_slice(&env, "patient-123-");
        let result = MedicalRecords::get_record_metadata(env.clone(), record_id);
        assert!(result.is_ok());
    }

    #[test]
    fn test_get_record_metadata_not_found() {
        let env = soroban_sdk::Env::default();

        let record_id = String::from_slice(&env, "nonexistent-123");
        let result = MedicalRecords::get_record_metadata(env.clone(), record_id);
        assert_eq!(result, Err(RecordError::RecordNotFound));
    }
}
