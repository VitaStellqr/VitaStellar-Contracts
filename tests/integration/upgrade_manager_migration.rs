//! Integration tests for upgrade_manager migration shim.
//!
//! Asserts that:
//! 1. User balances written to persistent storage before migration are
//!    preserved intact after `NoOpMigration::migrate` runs (balance preservation
//!    across the v1→v2 upgrade path).
//! 2. Calling `execute_upgrade` with a new_version ≤ current_version is
//!    rejected with `UpgradeError::IncompatibleVersion` (version mismatch error).

#![allow(clippy::unwrap_used)]

use soroban_sdk::{
    contract, contractimpl, symbol_short, testutils::Address as _, Address, BytesN, Env, Map,
    Symbol,
};
use upgradeability::{
    migration::{Migratable, UpgradeValidation},
    storage, UpgradeError,
};

// ---------------------------------------------------------------------------
// Minimal stub contract – needed so env.as_contract() has a registered ID.
// ---------------------------------------------------------------------------

#[contract]
pub struct StubContract;

#[contractimpl]
impl StubContract {}

// ---------------------------------------------------------------------------
// Storage helpers
// ---------------------------------------------------------------------------

const BALANCES: Symbol = symbol_short!("BALS");

fn write_balance(env: &Env, owner: &Address, amount: i128) {
    let mut map: Map<Address, i128> = env
        .storage()
        .persistent()
        .get(&BALANCES)
        .unwrap_or(Map::new(env));
    map.set(owner.clone(), amount);
    env.storage().persistent().set(&BALANCES, &map);
}

fn read_balance(env: &Env, owner: &Address) -> i128 {
    let map: Map<Address, i128> = env
        .storage()
        .persistent()
        .get(&BALANCES)
        .unwrap_or(Map::new(env));
    map.get(owner.clone()).unwrap_or(0)
}

// ---------------------------------------------------------------------------
// A no-op Migratable implementation: migration preserves storage untouched.
// ---------------------------------------------------------------------------

pub struct NoOpMigration;

impl Migratable for NoOpMigration {
    fn migrate(_env: &Env, _from_version: u32) -> Result<(), UpgradeError> {
        Ok(())
    }

    fn verify_integrity(env: &Env) -> Result<BytesN<32>, UpgradeError> {
        Ok(BytesN::from_array(env, &[0xAB_u8; 32]))
    }

    fn validate(env: &Env, _new_wasm_hash: &BytesN<32>) -> Result<UpgradeValidation, UpgradeError> {
        Ok(UpgradeValidation {
            state_compatible: true,
            api_compatible: true,
            storage_layout_valid: true,
            tests_passed: true,
            gas_impact: 0,
            report: soroban_sdk::Vec::new(env),
        })
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

/// Balances written before migration must be readable without change after
/// `NoOpMigration::migrate` executes (simulating a v1→v2 state migration).
#[test]
fn test_balance_preserved_across_upgrade() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register_contract(None, StubContract);

    let admin = Address::generate(&env);
    let user_a = Address::generate(&env);
    let user_b = Address::generate(&env);

    // Write v1 state inside the contract context.
    env.as_contract(&contract_id, || {
        storage::set_admin(&env, &admin);
        storage::set_version(&env, 1);
        write_balance(&env, &user_a, 500_i128);
        write_balance(&env, &user_b, 1_200_i128);
    });

    // Verify balances are present before migration.
    env.as_contract(&contract_id, || {
        assert_eq!(read_balance(&env, &user_a), 500_i128);
        assert_eq!(read_balance(&env, &user_b), 1_200_i128);
    });

    // Run the migration (balance-neutral no-op, as in the typical v1→v2 shim).
    env.as_contract(&contract_id, || {
        NoOpMigration::migrate(&env, 1).expect("migration must succeed");
    });

    // Post-migration: balances must be identical.
    env.as_contract(&contract_id, || {
        assert_eq!(
            read_balance(&env, &user_a),
            500_i128,
            "user_a balance must be preserved across upgrade"
        );
        assert_eq!(
            read_balance(&env, &user_b),
            1_200_i128,
            "user_b balance must be preserved across upgrade"
        );
    });
}

/// Attempting to upgrade to a version ≤ current must return
/// `UpgradeError::IncompatibleVersion`.
#[test]
fn test_version_mismatch_error() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register_contract(None, StubContract);

    let admin = Address::generate(&env);
    let dummy_hash = BytesN::from_array(&env, &[0x00u8; 32]);

    // Contract is already at v2.
    env.as_contract(&contract_id, || {
        storage::set_admin(&env, &admin);
        storage::set_version(&env, 2);
    });

    // Same version (2 → 2) must be rejected.
    let result = env.as_contract(&contract_id, || {
        upgradeability::execute_upgrade::<NoOpMigration>(
            &env,
            dummy_hash.clone(),
            2,
            symbol_short!("same"),
        )
    });
    assert_eq!(
        result,
        Err(UpgradeError::IncompatibleVersion),
        "upgrade to same version must fail with IncompatibleVersion"
    );

    // Downgrade (2 → 1) must also be rejected.
    let result_down = env.as_contract(&contract_id, || {
        upgradeability::execute_upgrade::<NoOpMigration>(&env, dummy_hash, 1, symbol_short!("down"))
    });
    assert_eq!(
        result_down,
        Err(UpgradeError::IncompatibleVersion),
        "downgrade must fail with IncompatibleVersion"
    );
}
