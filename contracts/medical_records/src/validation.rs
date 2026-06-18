//! Record validation helpers.
//!
//! This module provides validation logic for medical records, ensuring
//! that all required fields are present, non-empty, and conform to expected formats.

use soroban_sdk::{Env, String};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum ValidationError {
    /// Patient ID is missing or empty.
    MissingPatientId = 1,
    /// Record type is missing or empty.
    MissingRecordType = 2,
    /// Record content is empty.
    EmptyContent = 3,
    /// Record timestamp is invalid (0 or in future).
    InvalidTimestamp = 4,
}

/// Validates all required fields of a medical record.
///
/// # Arguments
/// * `env` – The Soroban environment
/// * `patient_id` – Patient identifier
/// * `record_type` – Type of medical record (e.g., "diagnosis", "prescription")
/// * `content` – The actual record content
/// * `timestamp` – Unix timestamp of record creation
///
/// # Returns
/// * `Ok(())` if all fields are valid
/// * `Err(ValidationError)` if any field fails validation
pub fn validate_record_fields(
    env: &Env,
    patient_id: &String,
    record_type: &String,
    content: &String,
    timestamp: u64,
) -> Result<(), ValidationError> {
    // Validate patient ID
    if patient_id.is_empty() {
        return Err(ValidationError::MissingPatientId);
    }

    // Validate record type
    if record_type.is_empty() {
        return Err(ValidationError::MissingRecordType);
    }

    // Validate content
    if content.is_empty() {
        return Err(ValidationError::EmptyContent);
    }

    // Validate timestamp: must be positive and not in future
    let current_timestamp = env.ledger().timestamp();
    if timestamp == 0 || timestamp > current_timestamp {
        return Err(ValidationError::InvalidTimestamp);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use soroban_sdk::String;

    #[test]
    fn test_validate_record_fields_success() {
        let env = soroban_sdk::Env::default();
        let patient_id = String::from_slice(&env, "patient-123");
        let record_type = String::from_slice(&env, "diagnosis");
        let content = String::from_slice(&env, "Patient diagnosed with hypertension");
        let timestamp = env.ledger().timestamp() - 3600; // 1 hour ago

        let result = validate_record_fields(&env, &patient_id, &record_type, &content, &timestamp);
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_missing_patient_id() {
        let env = soroban_sdk::Env::default();
        let patient_id = String::from_slice(&env, "");
        let record_type = String::from_slice(&env, "diagnosis");
        let content = String::from_slice(&env, "Patient diagnosed with hypertension");
        let timestamp = env.ledger().timestamp() - 3600;

        let result = validate_record_fields(&env, &patient_id, &record_type, &content, &timestamp);
        assert_eq!(result, Err(ValidationError::MissingPatientId));
    }

    #[test]
    fn test_validate_missing_record_type() {
        let env = soroban_sdk::Env::default();
        let patient_id = String::from_slice(&env, "patient-123");
        let record_type = String::from_slice(&env, "");
        let content = String::from_slice(&env, "Patient diagnosed with hypertension");
        let timestamp = env.ledger().timestamp() - 3600;

        let result = validate_record_fields(&env, &patient_id, &record_type, &content, &timestamp);
        assert_eq!(result, Err(ValidationError::MissingRecordType));
    }

    #[test]
    fn test_validate_empty_content() {
        let env = soroban_sdk::Env::default();
        let patient_id = String::from_slice(&env, "patient-123");
        let record_type = String::from_slice(&env, "diagnosis");
        let content = String::from_slice(&env, "");
        let timestamp = env.ledger().timestamp() - 3600;

        let result = validate_record_fields(&env, &patient_id, &record_type, &content, &timestamp);
        assert_eq!(result, Err(ValidationError::EmptyContent));
    }

    #[test]
    fn test_validate_invalid_timestamp_zero() {
        let env = soroban_sdk::Env::default();
        let patient_id = String::from_slice(&env, "patient-123");
        let record_type = String::from_slice(&env, "diagnosis");
        let content = String::from_slice(&env, "Patient diagnosed with hypertension");

        let result = validate_record_fields(&env, &patient_id, &record_type, &content, 0);
        assert_eq!(result, Err(ValidationError::InvalidTimestamp));
    }

    #[test]
    fn test_validate_invalid_timestamp_future() {
        let env = soroban_sdk::Env::default();
        let patient_id = String::from_slice(&env, "patient-123");
        let record_type = String::from_slice(&env, "diagnosis");
        let content = String::from_slice(&env, "Patient diagnosed with hypertension");
        let future_timestamp = env.ledger().timestamp() + 3600; // 1 hour in future

        let result = validate_record_fields(&env, &patient_id, &record_type, &content, future_timestamp);
        assert_eq!(result, Err(ValidationError::InvalidTimestamp));
    }
}
