use crate::RecordError;
use soroban_sdk::{Env, String};

pub fn validate_record_fields(
    _env: &Env,
    patient_id: &String,
    record_type: &String,
    content: &String,
    timestamp: u64,
) -> Result<(), RecordError> {
    if patient_id.is_empty() {
        return Err(RecordError::InvalidInput);
    }
    if record_type.is_empty() {
        return Err(RecordError::InvalidInput);
    }
    if content.is_empty() {
        return Err(RecordError::InvalidInput);
    }
    if timestamp == 0 {
        return Err(RecordError::InvalidInput);
    }
    Ok(())
}
