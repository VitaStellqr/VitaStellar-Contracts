//! Storage helpers — re-exported from `common_error` for backwards compatibility.
//!
//! Prefer importing directly from `common_error::storage` in new code.

pub use common_error::{read_or_default, try_read};

// ---------------------------------------------------------------------------
// Unit tests
// ---------------------------------------------------------------------------

#[cfg(all(test, feature = "testutils"))]
mod tests {
    use super::*;
    use soroban_sdk::{contract, contractimpl, symbol_short, Env};

    #[contract]
    struct StorageTestContract;
    #[contractimpl]
    impl StorageTestContract {}

    #[test]
    fn read_or_default_returns_default_when_key_missing() {
        let env = Env::default();
        let id = env.register_contract(None, StorageTestContract);
        env.as_contract(&id, || {
            let key = symbol_short!("COUNTER");
            let val: u64 = read_or_default(&env, &key);
            assert_eq!(val, 0u64);
        });
    }

    #[test]
    fn read_or_default_returns_stored_value() {
        let env = Env::default();
        let id = env.register_contract(None, StorageTestContract);
        env.as_contract(&id, || {
            let key = symbol_short!("COUNTER");
            env.storage().instance().set(&key, &42u64);
            let val: u64 = read_or_default(&env, &key);
            assert_eq!(val, 42u64);
        });
    }

    #[test]
    fn try_read_returns_none_when_key_missing() {
        let env = Env::default();
        let id = env.register_contract(None, StorageTestContract);
        env.as_contract(&id, || {
            let key = symbol_short!("FLAG");
            let val: Option<bool> = try_read(&env, &key);
            assert_eq!(val, None);
        });
    }

    #[test]
    fn try_read_returns_some_when_key_present() {
        let env = Env::default();
        let id = env.register_contract(None, StorageTestContract);
        env.as_contract(&id, || {
            let key = symbol_short!("FLAG");
            env.storage().instance().set(&key, &true);
            let val: Option<bool> = try_read(&env, &key);
            assert_eq!(val, Some(true));
        });
    }

    #[test]
    fn read_or_default_bool_defaults_to_false() {
        let env = Env::default();
        let id = env.register_contract(None, StorageTestContract);
        env.as_contract(&id, || {
            let key = symbol_short!("PAUSED");
            let val: bool = read_or_default(&env, &key);
            assert!(!val);
        });
    }
}
