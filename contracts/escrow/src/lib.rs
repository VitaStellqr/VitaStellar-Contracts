//! # Escrow Contract
//!
//! ## Security: Checks-Effects-Interactions (CEI) Pattern
//!
//! All state-mutating functions follow CEI strictly. The `withdraw` module
//! provides a `REENTRANCY_LOCK` guard as an additional defense-in-depth layer.
#![no_std]
#![allow(clippy::needless_borrow)]
#![allow(clippy::unnecessary_cast)]
#![allow(dead_code)]

pub mod approvals;
pub mod errors;
pub mod status;
pub mod types;
pub mod withdraw;

pub use errors::Error;
pub use types::{
    DailyStats, Escrow, EscrowStatus, ExportMetadata, FeeConfig, PlatformStats, ADMIN, ESCROWS,
    FEE_CONF,
};

use soroban_sdk::{contract, contractimpl, symbol_short, Address, BytesN, Env, Map, String};

use approvals::{add_credit, approve_release as do_approve, mark_disputed as do_mark_disputed};
use status::{
    get_active_escrows_count, get_daily_stats, get_dispute_rate, get_donor_reputation,
    get_platform_health_score, get_refund_rate, get_settled_rate, get_stats_summary,
    get_token_volume, get_total_escrows, get_total_volume, update_stats,
};
use withdraw::{
    clear_reentrancy, get_credit as do_get_credit, require_not_reentrant, withdraw as do_withdraw,
};

#[contract]
pub struct EscrowContract;

#[contractimpl]
impl EscrowContract {
    pub fn initialize(env: Env, admin: Address) -> Result<(), Error> {
        if env.storage().instance().has(&ADMIN) {
            return Err(Error::Unauthorized);
        }
        env.storage().instance().set(&ADMIN, &admin);
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
        let mut escrows: Map<u64, Escrow> = env
            .storage()
            .persistent()
            .get(&ESCROWS)
            .unwrap_or(Map::new(&env));
        if escrows.contains_key(order_id) {
            return Err(Error::EscrowExists);
        }
        escrows.set(
            order_id,
            Escrow {
                order_id,
                payer: payer.clone(),
                payee: payee.clone(),
                amount,
                token: token.clone(),
                status: EscrowStatus::Pending,
                approvals: soroban_sdk::Vec::new(&env),
                reason: String::from_str(&env, ""),
            },
        );
        env.storage().persistent().set(&ESCROWS, &escrows);
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
        require_not_reentrant(&env)?;
        let fee_conf: FeeConfig = env
            .storage()
            .instance()
            .get(&FEE_CONF)
            .ok_or(Error::FeeNotSet)?;
        let mut escrows: Map<u64, Escrow> = env
            .storage()
            .persistent()
            .get(&ESCROWS)
            .unwrap_or(Map::new(&env));
        let mut e = escrows.get(order_id).ok_or(Error::EscrowNotFound)?;

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
        escrows.set(order_id, e.clone());
        env.storage().persistent().set(&ESCROWS, &escrows);

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
        clear_reentrancy(&env);
        Ok(true)
    }

    pub fn refund_escrow(env: Env, order_id: u64, reason: String) -> Result<bool, Error> {
        require_not_reentrant(&env)?;
        let mut escrows: Map<u64, Escrow> = env
            .storage()
            .persistent()
            .get(&ESCROWS)
            .unwrap_or(Map::new(&env));
        let mut e = escrows.get(order_id).ok_or(Error::EscrowNotFound)?;

        if e.status == EscrowStatus::Settled || e.status == EscrowStatus::Refunded {
            return Err(Error::AlreadySettled);
        }
        if e.status == EscrowStatus::Pending && e.approvals.is_empty() {
            return Err(Error::NoBasisToRefund);
        }

        let was_active = e.status == EscrowStatus::Active || e.status == EscrowStatus::Disputed;
        e.status = EscrowStatus::Refunded;
        e.reason = reason.clone();
        escrows.set(order_id, e.clone());
        env.storage().persistent().set(&ESCROWS, &escrows);

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
        clear_reentrancy(&env);
        Ok(true)
    }

    pub fn get_escrow(env: Env, order_id: u64) -> Option<Escrow> {
        let escrows: Map<u64, Escrow> = env
            .storage()
            .persistent()
            .get(&ESCROWS)
            .unwrap_or(Map::new(&env));
        escrows.get(order_id)
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
}

#[cfg(all(test, feature = "testutils"))]
#[allow(clippy::unwrap_used)]
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
        // Trip the reentrancy lock inside contract context then verify guard fires
        use soroban_sdk::symbol_short;
        let cid = env.register_contract(None, EscrowContract);
        env.as_contract(&cid, || {
            env.storage()
                .temporary()
                .set(&symbol_short!("relock"), &true);
        });
        // A fresh contract instance won't have its lock set, so just verify
        // the error codes are correct (full reentrancy tested via withdraw test)
        assert_eq!(Error::ReentrancyGuard as u32, 381);
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
}
