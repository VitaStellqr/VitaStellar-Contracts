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

// Integration tests for healthcare_reputation contract (Issue #193).
//
// These exercise the real Soroban contract via `soroban_sdk::Env::default()`
// and replace the previously-mocked Python suite at
// `tests/healthcare_reputation_test.py`. See tests/FRAMEWORK.md for rationale.
//
// The module is named `healthcare_reputation_tests` (not `healthcare_reputation`)
// to avoid shadowing the external `healthcare_reputation` contract crate
// imported via `tests/Cargo.toml`. With the original name, the bare
// `healthcare_reputation::...` path resolved to the local file itself, causing
// a circular import the compiler cannot satisfy.
mod healthcare_reputation_tests;
