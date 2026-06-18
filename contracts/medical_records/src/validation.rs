#![allow(dead_code)]

use soroban_sdk::{Env, String};
use crate::RecordError;

pub fn validate_record_fields(
    _env: &Env,
    patient_id: &String,
    record_type: &String,
    content: &String,
    timestamp: u64,
) -> Result<(), RecordError> {
    if patient_id.len() == 0 {
        return Err(RecordError::InvalidInput);
    }
    if record_type.len() == 0 {
        return Err(RecordError::InvalidInput);
    }
    if content.len() == 0 {
        return Err(RecordError::InvalidInput);
    }
    if timestamp == 0 {
        return Err(RecordError::InvalidInput);
    }
    Ok(())
}
