pub mod upgrade_manager_migration;

/// Basic environment integration tests
#[cfg(test)]
mod unit_tests {
    use soroban_sdk::{Env, String};

    #[test]
    fn test_string_operations() {
        let env = Env::default();
        let test_string = String::from_str(&env, "test_patient_id");
        assert_eq!(test_string.len(), 15);
    }

    #[test]
    fn test_environment_setup() {
        let env = Env::default();
        assert_eq!(env.ledger().timestamp(), 0);
        assert_eq!(env.ledger().sequence(), 0);
    }
}

// Integration tests for medical_records contract (Issue #65)
mod medical_records;
