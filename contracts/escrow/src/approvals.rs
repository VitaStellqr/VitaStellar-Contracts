use soroban_sdk::{symbol_short, Address, Env, Map};

use crate::errors::Error;
use crate::status::update_stats;
use crate::types::{Escrow, EscrowStatus, CREDITS, ESCROWS};

pub fn add_credit(env: &Env, addr: &Address, delta: i128) {
    let mut credits: Map<Address, i128> = env
        .storage()
        .persistent()
        .get(&CREDITS)
        .unwrap_or(Map::new(env));
    let current = credits.get(addr.clone()).unwrap_or(0);
    credits.set(addr.clone(), current.saturating_add(delta));
    env.storage().persistent().set(&CREDITS, &credits);
}

pub fn approve_release(env: &Env, order_id: u64, approver: Address) -> Result<(), Error> {
    approver.require_auth();
    let mut escrows: Map<u64, Escrow> = env
        .storage()
        .persistent()
        .get(&ESCROWS)
        .unwrap_or(Map::new(env));
    let mut e = escrows.get(order_id).ok_or(Error::EscrowNotFound)?;

    if e.status == EscrowStatus::Settled || e.status == EscrowStatus::Refunded {
        return Err(Error::AlreadySettled);
    }

    let mut approvals = e.approvals.clone();
    if !approvals.contains(&approver) {
        approvals.push_back(approver);
    }
    e.approvals = approvals;

    if e.status == EscrowStatus::Pending && !e.approvals.is_empty() {
        e.status = EscrowStatus::Active;
        update_stats(env, 0, false, false, false, false, 1);
    }

    escrows.set(order_id, e);
    env.storage().persistent().set(&ESCROWS, &escrows);
    Ok(())
}

pub fn mark_disputed(env: &Env, caller: Address, order_id: u64) -> Result<(), Error> {
    caller.require_auth();
    let mut escrows: Map<u64, Escrow> = env
        .storage()
        .persistent()
        .get(&ESCROWS)
        .unwrap_or(Map::new(env));
    let mut e = escrows.get(order_id).ok_or(Error::EscrowNotFound)?;

    if e.status == EscrowStatus::Settled || e.status == EscrowStatus::Refunded {
        return Err(Error::AlreadySettled);
    }

    e.status = EscrowStatus::Disputed;
    escrows.set(order_id, e);
    env.storage().persistent().set(&ESCROWS, &escrows);

    update_stats(env, 0, false, false, false, true, 0);

    env.events()
        .publish((symbol_short!("EscDisput"), order_id), ());
    Ok(())
}

#[cfg(all(test, feature = "testutils"))]
mod tests {
    use crate::{EscrowContract, EscrowContractClient, EscrowStatus};
    use soroban_sdk::{testutils::Address as _, Address, Env};

    fn setup() -> (
        Env,
        EscrowContractClient<'static>,
        Address,
        Address,
        Address,
    ) {
        let env = Env::default();
        let cid = env.register_contract(None, EscrowContract);
        let client = EscrowContractClient::new(&env, &cid);
        let admin = Address::generate(&env);
        let payer = Address::generate(&env);
        let payee = Address::generate(&env);
        let token = Address::generate(&env);
        client.mock_all_auths().initialize(&admin);
        client
            .mock_all_auths()
            .set_fee_config(&admin, &Address::generate(&env), &250u32);
        client
            .mock_all_auths()
            .create_escrow(&1u64, &payer, &payee, &1000i128, &token);
        (env, client, payer, payee, token)
    }

    #[test]
    fn test_approve_transitions_to_active() {
        let (_, client, payer, _, _) = setup();
        client.mock_all_auths().approve_release(&1u64, &payer);
        assert_eq!(
            client.get_escrow(&1u64).unwrap().status,
            EscrowStatus::Active
        );
        assert_eq!(client.get_escrow(&1u64).unwrap().approvals.len(), 1);
    }

    #[test]
    fn test_approve_no_duplicate() {
        let (_, client, payer, _, _) = setup();
        client.mock_all_auths().approve_release(&1u64, &payer);
        client.mock_all_auths().approve_release(&1u64, &payer);
        assert_eq!(client.get_escrow(&1u64).unwrap().approvals.len(), 1);
    }

    #[test]
    fn test_mark_disputed() {
        let (_, client, payer, _, _) = setup();
        client.mock_all_auths().mark_disputed(&payer, &1u64);
        assert_eq!(
            client.get_escrow(&1u64).unwrap().status,
            EscrowStatus::Disputed
        );
    }

    #[test]
    fn test_approve_already_settled_errors() {
        let (env, client, payer, payee, token) = setup();
        // create a second escrow, settle it, then try to approve again
        client
            .mock_all_auths()
            .create_escrow(&2u64, &payer, &payee, &500i128, &token);
        client.mock_all_auths().approve_release(&2u64, &payer);
        client
            .mock_all_auths()
            .approve_release(&2u64, &Address::generate(&env));
        client.release_escrow(&2u64);
        assert!(client.try_approve_release(&2u64, &payer).is_err());
    }

    #[test]
    fn test_add_credit_accumulates() {
        let (env, client, payer, payee, token) = setup();
        // Release two escrows to the same payee to accumulate credits
        client
            .mock_all_auths()
            .create_escrow(&2u64, &payer, &payee, &500i128, &token);
        client.mock_all_auths().approve_release(&1u64, &payer);
        client
            .mock_all_auths()
            .approve_release(&1u64, &Address::generate(&env));
        client.release_escrow(&1u64);
        client.mock_all_auths().approve_release(&2u64, &payer);
        client
            .mock_all_auths()
            .approve_release(&2u64, &Address::generate(&env));
        client.release_escrow(&2u64);
        // 1000 * 0.975 + 500 * 0.975 = 975 + 487 = 1462
        assert!(client.get_credit(&payee) > 0);
    }
}
