use soroban_sdk::{Env, String};
use crate::RecordError;

pub fn validate_record_fields(
    env: &Env,
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

#[cfg(test)]
mod tests {
    use super::*;
    use soroban_sdk::{Env, String};

    #[test]
    fn test_validate_record_fields_valid() {
        let env = Env::default();
        let patient_id = String::from_str(&env, "patient-001");
        let record_type = String::from_str(&env, "consultation");
        let content = String::from_str(&env, "content");
        let result = validate_record_fields(&env, &patient_id, &record_type, &content, 123);
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_record_fields_invalid_patient_id() {
        let env = Env::default();
        let patient_id = String::from_str(&env, "");
        let record_type = String::from_str(&env, "consultation");
        let content = String::from_str(&env, "content");
        let result = validate_record_fields(&env, &patient_id, &record_type, &content, 123);
        assert!(result.is_err());
    }
}
