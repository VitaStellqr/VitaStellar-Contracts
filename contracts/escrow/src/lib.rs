//! # Escrow Contract
//!
//! ## Security: Checks-Effects-Interactions (CEI) Pattern
//!
//! All state-mutating functions follow CEI strictly. The `withdraw` module
//! provides a `REENTRANCY_LOCK` guard as an additional defense-in-depth layer.
//!
//! ## Storage: per-item keys, not a bulk map
//!
//! Each escrow is stored under its own `DataKey::Escrow(order_id)` key
//! rather than inside one bulk `Map<u64, Escrow>`. This keeps every read
//! and write O(1) regardless of how many escrows exist, instead of
//! deserializing/reserializing the entire collection on every call. See
//! `migrate_storage` for the one-time upgrade path from the legacy bulk
//! `ESCROWS` map used by pre-upgrade deployments.
#![no_std]
#![allow(clippy::needless_borrow)] // Borrowing form is intentional for clarity or ABI compatibility
#![allow(clippy::unnecessary_cast)] // Intentional lint suppression with a deliberate reason
#![allow(dead_code)] // Unused code is intentionally retained for compatibility or test scaffolding

pub mod approvals;
pub mod errors;
pub mod status;
pub mod types;
pub mod withdraw;

pub use errors::Error;
pub use types::{
    DailyStats, DataKey, Escrow, EscrowStatus, ExportMetadata, FeeConfig, PlatformStats, ADMIN,
    ESCROWS, FEE_CONF,
};

use soroban_sdk::{contract, contractimpl, symbol_short, Address, BytesN, Env, Map, String};

use approvals::{add_credit, approve_release as do_approve, mark_disputed as do_mark_disputed};
use reentrancy_guard as reentrancy;

use status::{
    get_active_escrows_count, get_daily_stats, get_dispute_rate, get_donor_reputation,
    get_platform_health_score, get_refund_rate, get_settled_rate, get_stats_summary,
    get_token_volume, get_total_escrows, get_total_volume, update_stats,
};
use withdraw::{get_credit as do_get_credit, withdraw as do_withdraw};

/// O(1) read of a single escrow by order_id.
pub(crate) fn get_escrow_by_id(env: &Env, order_id: u64) -> Option<Escrow> {
    env.storage().persistent().get(&DataKey::Escrow(order_id))
}

/// O(1) write of a single escrow by order_id.
pub(crate) fn put_escrow(env: &Env, order_id: u64, e: &Escrow) {
    env.storage()
        .persistent()
        .set(&DataKey::Escrow(order_id), e);
}

#[contract]
pub struct EscrowContract;

#[contractimpl]
impl EscrowContract {
    pub fn initialize(env: Env, admin: Address) -> Result<(), Error> {
        access_utils::init_admin(&env, &ADMIN, &admin).map_err(|_| Error::Unauthorized)?;
        Ok(())
    }

    pub fn set_fee_config(
        env: Env,
        caller: Address,
        fee_receiver: Address,
        platform_fee_bps: u32,
    ) -> Result<(), Error> {
        caller.require_auth();
        let admin: Address = env
            .storage()
            .instance()
            .get(&ADMIN)
            .ok_or(Error::NotAdmin)?;
        if caller != admin {
            return Err(Error::NotAdmin);
        }
        if platform_fee_bps > 10_000 {
            return Err(Error::InvalidFeeBps);
        }
        env.storage().instance().set(
            &FEE_CONF,
            &FeeConfig {
                fee_receiver,
                platform_fee_bps,
            },
        );
        Ok(())
    }

    pub fn get_fee_config(env: Env) -> Option<FeeConfig> {
        env.storage().instance().get(&FEE_CONF)
    }

    pub fn create_escrow(
        env: Env,
        order_id: u64,
        payer: Address,
        payee: Address,
        amount: i128,
        token: Address,
    ) -> Result<bool, Error> {
        payer.require_auth();
        if amount <= 0 {
            return Err(Error::InvalidAmount);
        }
        // O(1): checks only this order_id's own key.
        if env.storage().persistent().has(&DataKey::Escrow(order_id)) {
            return Err(Error::EscrowExists);
        }
        let e = Escrow {
            order_id,
            payer: payer.clone(),
            payee: payee.clone(),
            amount,
            token: token.clone(),
            status: EscrowStatus::Pending,
            approvals: soroban_sdk::Vec::new(&env),
            reason: String::from_str(&env, ""),
        };
        put_escrow(&env, order_id, &e);
        update_stats(&env, amount, true, false, false, false, 0);
        env.events().publish(
            (symbol_short!("EscNew"), order_id),
            (payer, payee, amount, token),
        );
        Ok(true)
    }

    pub fn mark_disputed(env: Env, caller: Address, order_id: u64) -> Result<(), Error> {
        do_mark_disputed(&env, caller, order_id)
    }

    pub fn approve_release(env: Env, order_id: u64, approver: Address) -> Result<(), Error> {
        do_approve(&env, order_id, approver)
    }

    pub fn release_escrow(env: Env, order_id: u64) -> Result<bool, Error> {
        if !reentrancy::enter(&env) {
            return Err(Error::ReentrancyRejected);
        }
        let result = (|| {
            let fee_conf: FeeConfig = env
                .storage()
                .instance()
                .get(&FEE_CONF)
                .ok_or(Error::FeeNotSet)?;
            let mut e = get_escrow_by_id(&env, order_id).ok_or(Error::EscrowNotFound)?;

            if e.status == EscrowStatus::Settled || e.status == EscrowStatus::Refunded {
                return Err(Error::AlreadySettled);
            }
            if e.status == EscrowStatus::Pending {
                return Err(Error::InvalidStateTransition);
            }
            if e.approvals.len() < 2 {
                return Err(Error::InsufficientApprovals);
            }

            e.status = EscrowStatus::Settled;
            put_escrow(&env, order_id, &e);

            let fee = e
                .amount
                .checked_mul(fee_conf.platform_fee_bps as i128)
                .map(|n| n / 10_000)
                .ok_or(Error::Overflow)?;
            let provider_amount = e.amount.saturating_sub(fee);
            add_credit(&env, &e.payee, provider_amount);
            add_credit(&env, &fee_conf.fee_receiver, fee);
            update_stats(&env, 0, false, true, false, false, -1);
            env.events().publish(
                (symbol_short!("EscRel"), order_id),
                (
                    e.payee,
                    provider_amount,
                    fee_conf.fee_receiver,
                    fee,
                    e.token,
                ),
            );
            Ok(true)
        })();
        reentrancy::exit(&env);
        result
    }

    pub fn refund_escrow(env: Env, order_id: u64, reason: String) -> Result<bool, Error> {
        if !reentrancy::enter(&env) {
            return Err(Error::ReentrancyRejected);
        }
        let result = (|| {
            let mut e = get_escrow_by_id(&env, order_id).ok_or(Error::EscrowNotFound)?;

            if e.status == EscrowStatus::Settled || e.status == EscrowStatus::Refunded {
                return Err(Error::AlreadySettled);
            }
            if e.status == EscrowStatus::Pending && e.approvals.is_empty() {
                return Err(Error::NoBasisToRefund);
            }

            let was_active = e.status == EscrowStatus::Active || e.status == EscrowStatus::Disputed;
            e.status = EscrowStatus::Refunded;
            e.reason = reason.clone();
            put_escrow(&env, order_id, &e);

            add_credit(&env, &e.payer, e.amount);
            update_stats(
                &env,
                0,
                false,
                false,
                true,
                false,
                if was_active { -1 } else { 0 },
            );
            env.events().publish(
                (symbol_short!("Refunded"), order_id),
                (e.payer, e.amount, e.token, reason),
            );
            Ok(true)
        })();
        reentrancy::exit(&env);
        result
    }

    pub fn get_escrow(env: Env, order_id: u64) -> Option<Escrow> {
        get_escrow_by_id(&env, order_id)
    }

    pub fn get_credit(env: Env, addr: Address) -> i128 {
        do_get_credit(&env, addr)
    }
    pub fn withdraw(env: Env, caller: Address, token: Address, to: Address) -> Result<i128, Error> {
        do_withdraw(&env, caller, token, to)
    }
    pub fn get_total_volume(env: Env) -> i128 {
        get_total_volume(&env)
    }
    pub fn get_total_escrows(env: Env) -> u64 {
        get_total_escrows(&env)
    }
    pub fn get_settled_rate(env: Env) -> u32 {
        get_settled_rate(&env)
    }
    pub fn get_refund_rate(env: Env) -> u32 {
        get_refund_rate(&env)
    }
    pub fn get_dispute_rate(env: Env) -> u32 {
        get_dispute_rate(&env)
    }
    pub fn get_active_escrows_count(env: Env) -> u64 {
        get_active_escrows_count(&env)
    }
    pub fn get_stats_summary(env: Env) -> PlatformStats {
        get_stats_summary(&env)
    }
    pub fn get_platform_health_score(env: Env) -> u32 {
        get_platform_health_score(&env)
    }
    pub fn get_token_volume(env: Env, token: Address) -> i128 {
        get_token_volume(&env, token)
    }
    pub fn get_donor_reputation(env: Env, donor: Address) -> u32 {
        get_donor_reputation(&env, donor)
    }
    pub fn get_daily_stats(env: Env, day_id: u64) -> Option<DailyStats> {
        get_daily_stats(&env, day_id)
    }

    pub fn export_summary(env: Env, format: String) -> ExportMetadata {
        ExportMetadata {
            format,
            checksum: BytesN::from_array(&env, &[0u8; 32]),
            timestamp: env.ledger().timestamp(),
        }
    }

    /// One-time migration for pre-upgrade deployments: copies every
    /// escrow out of the legacy bulk `ESCROWS` map into individual
    /// `DataKey::Escrow(order_id)` entries, then removes the legacy bulk
    /// key. Admin-gated. Idempotent: once the legacy key is gone,
    /// subsequent calls are a cheap no-op that return `0`.
    ///
    /// Returns the number of escrows migrated.
    pub fn migrate_storage(env: Env, caller: Address) -> Result<u32, Error> {
        caller.require_auth();
        let admin: Address = env
            .storage()
            .instance()
            .get(&ADMIN)
            .ok_or(Error::NotAdmin)?;
        if caller != admin {
            return Err(Error::NotAdmin);
        }

        let mut migrated: u32 = 0;
        if env.storage().persistent().has(&ESCROWS) {
            let escrows: Map<u64, Escrow> = env
                .storage()
                .persistent()
                .get(&ESCROWS)
                .unwrap_or(Map::new(&env));
            for (order_id, e) in escrows.iter() {
                put_escrow(&env, order_id, &e);
                migrated = migrated.saturating_add(1);
            }
            env.storage().persistent().remove(&ESCROWS);
        }

        Ok(migrated)
    }
}

#[cfg(all(test, feature = "testutils"))]
#[allow(clippy::unwrap_used)] // Unwrap is intentionally used in this contract context
mod test {
    use super::*;
    use soroban_sdk::testutils::Address as _;
    use soroban_sdk::{Address, Env, String};

    fn setup_contract(env: &Env) -> (EscrowContractClient, Address, Address, Address, Address) {
        let cid = env.register_contract(None, EscrowContract);
        let client = EscrowContractClient::new(env, &cid);
        let admin = Address::generate(env);
        let payer = Address::generate(env);
        let payee = Address::generate(env);
        let token = Address::generate(env);
        client.mock_all_auths().initialize(&admin);
        client
            .mock_all_auths()
            .set_fee_config(&admin, &Address::generate(env), &250u32);
        (client, admin, payer, payee, token)
    }

    #[test]
    fn test_normal_release_flow() {
        let env = Env::default();
        let (client, _, payer, payee, token) = setup_contract(&env);
        assert!(client
            .mock_all_auths()
            .create_escrow(&1u64, &payer, &payee, &1000i128, &token));
        client.mock_all_auths().approve_release(&1u64, &payer);
        client
            .mock_all_auths()
            .approve_release(&1u64, &Address::generate(&env));
        assert!(client.release_escrow(&1u64));
        assert_eq!(
            client.get_escrow(&1u64).unwrap().status,
            EscrowStatus::Settled
        );
        assert_eq!(client.get_credit(&payee), 975);
        assert_eq!(client.get_total_volume(), 1000);
        assert_eq!(client.get_settled_rate(), 10000);
    }

    #[test]
    fn test_refund_flow_with_dispute() {
        let env = Env::default();
        let (client, _, payer, payee, token) = setup_contract(&env);
        assert!(client
            .mock_all_auths()
            .create_escrow(&2u64, &payer, &payee, &1000i128, &token));
        client.mock_all_auths().mark_disputed(&payer, &2u64);
        assert!(client.refund_escrow(&2u64, &String::from_str(&env, "Dispute resolved by refund")));
        assert_eq!(
            client.get_escrow(&2u64).unwrap().status,
            EscrowStatus::Refunded
        );
        assert_eq!(client.get_credit(&payer), 1000);
    }

    #[test]
    fn test_invalid_transitions() {
        let env = Env::default();
        let (client, _, payer, payee, token) = setup_contract(&env);
        client
            .mock_all_auths()
            .create_escrow(&3u64, &payer, &payee, &1000i128, &token);
        assert!(client.try_release_escrow(&3u64).is_err());
    }

    #[test]
    fn test_authorization() {
        let env = Env::default();
        let (client, _, _, _, _) = setup_contract(&env);
        let attacker = Address::generate(&env);
        assert!(client
            .try_set_fee_config(&attacker, &attacker, &100u32)
            .is_err());
    }

    #[test]
    fn test_reentrancy_guard() {
        let env = Env::default();
        let (client, _, payer, payee, token) = setup_contract(&env);
        client
            .mock_all_auths()
            .create_escrow(&10u64, &payer, &payee, &1000i128, &token);
        client.mock_all_auths().approve_release(&10u64, &payer);
        client
            .mock_all_auths()
            .approve_release(&10u64, &Address::generate(&env));
        let cid = env.register_contract(None, EscrowContract);
        env.as_contract(&cid, || {
            env.storage()
                .instance()
                .set(&symbol_short!("reentrant"), &true);
        });
        assert_eq!(
            client.try_release_escrow(&10u64),
            Err(Error::ReentrancyRejected)
        );
    }

    #[test]
    fn test_error_codes_are_stable() {
        assert_eq!(Error::Unauthorized as u32, 100);
        assert_eq!(Error::NotAdmin as u32, 102);
        assert_eq!(Error::InvalidAmount as u32, 205);
        assert_eq!(Error::EscrowNotFound as u32, 481);
        assert_eq!(Error::AlreadySettled as u32, 482);
    }

    #[test]
    fn test_get_suggestion_returns_expected_hint() {
        use soroban_sdk::symbol_short;
        assert_eq!(
            crate::errors::get_suggestion(Error::Unauthorized),
            symbol_short!("CHK_AUTH")
        );
        assert_eq!(
            crate::errors::get_suggestion(Error::InvalidAmount),
            symbol_short!("CHK_LEN")
        );
        assert_eq!(
            crate::errors::get_suggestion(Error::EscrowNotFound),
            symbol_short!("CHK_ID")
        );
        assert_eq!(
            crate::errors::get_suggestion(Error::AlreadySettled),
            symbol_short!("ALREADY")
        );
    }

    /// Locks down storage isolation: after this migration, escrows live
    /// under individual `DataKey::Escrow` entries rather than the old
    /// bulk `ESCROWS` map, so operating on one order_id cannot affect
    /// another's state.
    #[test]
    fn test_escrows_are_isolated_per_key() {
        let env = Env::default();
        let (client, _, payer, payee, token) = setup_contract(&env);
        client
            .mock_all_auths()
            .create_escrow(&1u64, &payer, &payee, &1000i128, &token);
        client
            .mock_all_auths()
            .create_escrow(&2u64, &payer, &payee, &500i128, &token);

        client.mock_all_auths().approve_release(&1u64, &payer);
        client
            .mock_all_auths()
            .approve_release(&1u64, &Address::generate(&env));
        client.release_escrow(&1u64);

        // Settling escrow 1 must not affect escrow 2's state.
        assert_eq!(
            client.get_escrow(&2u64).unwrap().status,
            EscrowStatus::Pending
        );
    }

    /// Migration is idempotent and correctly reports zero once there is
    /// nothing left in the legacy bulk map (the normal case for a
    /// freshly-initialized contract).
    #[test]
    fn test_migrate_storage_noop_on_fresh_contract() {
        let env = Env::default();
        let (client, admin, _, _, _) = setup_contract(&env);
        let migrated = client.mock_all_auths().migrate_storage(&admin);
        assert_eq!(migrated, 0);
    }

    /// Migration is gated to the admin address.
    #[test]
    fn test_migrate_storage_rejects_non_admin_caller() {
        let env = Env::default();
        let (client, _, _, _, _) = setup_contract(&env);
        let intruder = Address::generate(&env);
        let result = client.mock_all_auths().try_migrate_storage(&intruder);
        assert!(result.is_err());
    }

    /// Benchmark: proving O(1) storage cost. Escrow #1000 should cost
    /// roughly the same CPU budget as escrow #1 - with the legacy
    /// bulk-map scheme this would grow roughly linearly with the number
    /// of escrows already stored, since every write re-serialized the
    /// whole collection.
    #[test]
    fn bench_create_escrow_cost_is_constant_not_linear() {
        let env = Env::default();
        let (client, _, payer, payee, token) = setup_contract(&env);

        // Cost of escrow #1 in a fresh contract.
        env.budget().reset_default();
        client
            .mock_all_auths()
            .create_escrow(&1u64, &payer, &payee, &10i128, &token);
        let cost_first = env.budget().cpu_instruction_cost();

        // Fill up to escrow #999 (unmeasured).
        for i in 2..1000u64 {
            client
                .mock_all_auths()
                .create_escrow(&i, &payer, &payee, &10i128, &token);
        }

        // Cost of escrow #1000.
        env.budget().reset_default();
        client
            .mock_all_auths()
            .create_escrow(&1000u64, &payer, &payee, &10i128, &token);
        let cost_thousandth = env.budget().cpu_instruction_cost();

        // With per-item keys, cost should stay roughly constant (O(1)),
        // not grow linearly with the number of stored escrows (O(n)).
        let upper_bound = cost_first + (cost_first / 2) + 1;
        assert!(
            cost_thousandth <= upper_bound,
            "escrow #1000 cost {} exceeds tolerance over escrow #1 cost {} (bound {})",
            cost_thousandth,
            cost_first,
            upper_bound
        );
    }
}
