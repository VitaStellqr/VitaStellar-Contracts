#![no_std]

use soroban_sdk::{contract, contractimpl, contracttype, Address, Env, String, Vec};

mod validation;
mod crypto;

pub use validation::validate_record_fields;
pub use crypto::encrypt_payload;

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Record {
    pub id: u64,
    pub patient_id: String,
    pub record_type: String,
    pub content: String,
    pub timestamp: u64,
    pub owner: Address,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum RecordError {
    InvalidInput,
    Unauthorized,
    RecordNotFound,
    EncryptionFailed,
}

#[contract]
pub struct MedicalRecords;

#[contractimpl]
impl MedicalRecords {
    pub fn write_record(
        env: Env,
        owner: Address,
        patient_id: String,
        record_type: String,
        content: String,
        timestamp: u64,
    ) -> Result<(), RecordError> {
        owner.require_auth();
        
        validate_record_fields(&env, &patient_id, &record_type, &content, timestamp)?;
        
        let record_id = env.ledger().sequence() as u64;
        let encrypted_content = encrypt_payload(&env, record_id, content.as_str())
            .map_err(|_| RecordError::EncryptionFailed)?;
        
        let record = Record {
            id: record_id,
            patient_id,
            record_type,
            content: String::from_str(&env, &format!("encrypted:{}", record_id)),
            timestamp,
            owner,
        };
        
        env.storage().persistent().set(&record_id, &record);
        
        // Emit event
        env.events().publish(
            ("record_written", record_id),
            (record.owner.clone(), record.patient_id.clone(), record.record_type.clone(), record.timestamp),
        );
        
        Ok(())
    }

    pub fn get_record(env: Env, record_id: u64) -> Option<Record> {
        env.storage().persistent().get(&record_id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use soroban_sdk::testutils::Address as _;
    use soroban_sdk::String;
    
    #[test]
    fn test_write_record() {
        let env = Env::default();
        let owner = Address::generate(&env);
        let patient_id = String::from_str(&env, "patient-001");
        let record_type = String::from_str(&env, "consultation");
        let content = String::from_str(&env, "Patient reports feeling better");
        let timestamp = 1718640000;
        
        let result = MedicalRecords::write_record(
            env,
            owner,
            patient_id,
            record_type,
            content,
            timestamp,
        );
        assert!(result.is_ok());
    }
}
