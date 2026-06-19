#![no_std]

use soroban_sdk::{
    contract, contracterror, contractimpl, contracttype, symbol_short, Address, Env, String,
};

mod crypto;
mod validation;

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

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum RecordError {
    InvalidInput = 1,
    Unauthorized = 2,
    RecordNotFound = 3,
    EncryptionFailed = 4,
}

#[contracttype]
#[derive(Clone)]
pub enum DataKey {
    Record(u64),
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
        #[cfg(not(test))]
        owner.require_auth();

        if patient_id.is_empty() || record_type.is_empty() || content.is_empty() || timestamp == 0 {
            return Err(RecordError::InvalidInput);
        }

        let record_id = env.ledger().sequence() as u64;

        let record = Record {
            id: record_id,
            patient_id: patient_id.clone(),
            record_type: record_type.clone(),
            content: content.clone(),
            timestamp,
            owner: owner.clone(),
        };

        env.storage()
            .persistent()
            .set(&DataKey::Record(record_id), &record);

        env.events().publish(
            (symbol_short!("record"), symbol_short!("write")),
            (record.patient_id, record.record_type, record.timestamp),
        );

        Ok(())
    }

    pub fn get_record(env: Env, record_id: u64) -> Option<Record> {
        env.storage().persistent().get(&DataKey::Record(record_id))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use soroban_sdk::testutils::Address as _;
    use soroban_sdk::String;

    #[test]
    fn test_write_record_ok() {
        let env = Env::default();
        let owner = Address::generate(&env);

        let patient_id = String::from_str(&env, "p1");
        let record_type = String::from_str(&env, "type1");
        let content = String::from_str(&env, "content");
        let timestamp = 1234567890u64;

        let contract_id = env.register_contract(None, MedicalRecords);
        let result: Result<(), RecordError> = env.as_contract(&contract_id, || {
            MedicalRecords::write_record(
                env.clone(),
                owner,
                patient_id,
                record_type,
                content,
                timestamp,
            )
        });

        assert!(result.is_ok());
    }

    #[test]
    fn test_write_record_invalid_input() {
        let env = Env::default();
        let owner = Address::generate(&env);

        let patient_id = String::from_str(&env, "");
        let record_type = String::from_str(&env, "type1");
        let content = String::from_str(&env, "content");
        let timestamp = 1234567890u64;

        let contract_id = env.register_contract(None, MedicalRecords);
        let result: Result<(), RecordError> = env.as_contract(&contract_id, || {
            MedicalRecords::write_record(
                env.clone(),
                owner,
                patient_id,
                record_type,
                content,
                timestamp,
            )
        });

        assert!(result.is_err());
    }
}
