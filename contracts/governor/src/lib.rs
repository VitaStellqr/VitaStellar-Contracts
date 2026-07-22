#![no_std]
#![allow(clippy::too_many_arguments)] // Contract/API entrypoint requires explicit parameters for Soroban ABI
#![allow(clippy::needless_borrow)] // Borrowing form is intentional for clarity or ABI compatibility
#![allow(clippy::needless_return)] // Explicit return form is intentional for readability
#![allow(dead_code)] // Unused code is intentionally retained for compatibility or test scaffolding

pub mod errors;
use common_error::read_or_default;
pub use errors::Error;
use soroban_sdk::{
    contract, contractimpl, contracttype, symbol_short, vec, Address, Bytes, Env, IntoVal, Map,
    Symbol,
};

#[derive(Clone)]
#[contracttype]
pub struct GovernorConfig {
    pub voting_delay: u64,
    pub voting_period: u64,
    pub quorum_bps: u32,
    pub timelock: Address,
    pub token: Address,
    pub rep_contract: Option<Address>,
    pub dispute_contract: Option<Address>,
    pub prop_threshold: i128,
}

#[derive(Clone)]
#[contracttype]
pub struct Proposal {
    pub id: u64,
    pub proposer: Address,
    pub desc_hash: Bytes,
    pub start_time: u64,
    pub end_time: u64,
    pub for_votes: i128,
    pub against_votes: i128,
    pub abstain_votes: i128,
    pub canceled: bool,
    pub queued: bool,
    pub executed: bool,
    pub exec_data: Bytes,
}

/// Per-item persistent storage keys. Replaces the legacy bulk
/// `PROPS`/`VOTES` maps: each proposal and vote now lives under its own
/// key, so reading or writing one proposal/vote no longer deserializes
/// the entire collection (was O(n), now O(1)).
#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    /// A single proposal, keyed by its id.
    Proposal(u64),
    /// A single vote, keyed by (proposal_id, voter).
    Vote(u64, Address),
}

const CFG: Symbol = symbol_short!("cfg");
/// Legacy bulk proposal map key. No longer written to; retained only so
/// `migrate_storage` can detect and migrate data from pre-upgrade
/// deployments.
const PROPS: Symbol = symbol_short!("props");
const P_COUNT: Symbol = symbol_short!("p_count");
/// Legacy bulk vote map key. No longer written to; retained only so
/// `migrate_storage` can detect and migrate data from pre-upgrade
/// deployments.
const VOTES: Symbol = symbol_short!("votes");

#[contract]
pub struct Governor;

fn now(env: &Env) -> u64 {
    env.ledger().timestamp()
}

/// Read GovernorConfig from instance storage (cheap, cached by the host).
/// Instance storage is cheaper than persistent for frequently-read values.
fn get_cfg(env: &Env) -> Result<GovernorConfig, Error> {
    env.storage()
        .instance()
        .get(&CFG)
        .ok_or(Error::NotInitialized)
}

/// O(1) read of a single proposal by id.
fn get_proposal(env: &Env, id: u64) -> Result<Proposal, Error> {
    env.storage()
        .persistent()
        .get(&DataKey::Proposal(id))
        .ok_or(Error::ProposalNotFound)
}

/// O(1) write of a single proposal by id.
fn put_proposal(env: &Env, id: u64, p: &Proposal) {
    env.storage().persistent().set(&DataKey::Proposal(id), p);
}

#[contractimpl]
impl Governor {
    pub fn initialize(
        env: Env,
        token: Address,
        timelock: Address,
        voting_delay: u64,
        voting_period: u64,
        quorum_bps: u32,
        proposal_threshold: i128,
        reputation_contract: Option<Address>,
        dispute_contract: Option<Address>,
    ) -> Result<(), Error> {
        if access_utils::is_initialized(&env, &CFG) {
            return Err(Error::AlreadyInitialized);
        }
        let cfg = GovernorConfig {
            voting_delay,
            voting_period,
            quorum_bps,
            timelock,
            token,
            rep_contract: reputation_contract,
            dispute_contract,
            prop_threshold: proposal_threshold,
        };
        env.storage().instance().set(&CFG, &cfg);
        env.storage().instance().set(&P_COUNT, &0u64);
        Ok(())
    }

    pub fn propose(
        env: Env,
        proposer: Address,
        description_hash: Bytes,
        execution_data: Bytes,
    ) -> Result<u64, Error> {
        proposer.require_auth();
        // Single instance-storage read; host caches instance storage per tx.
        let cfg = get_cfg(&env)?;

        let voting_power = Self::get_power(&env, &cfg, &proposer);
        if voting_power < cfg.prop_threshold {
            return Err(Error::ProposalThresholdNotMet);
        }

        let count = read_or_default::<_, u64>(&env, &P_COUNT);
        let id = count.checked_add(1).ok_or(Error::Overflow)?;

        let start = now(&env)
            .checked_add(cfg.voting_delay)
            .ok_or(Error::Overflow)?;
        let end = start
            .checked_add(cfg.voting_period)
            .ok_or(Error::Overflow)?;

        let proposal = Proposal {
            id,
            proposer: proposer.clone(),
            desc_hash: description_hash,
            start_time: start,
            end_time: end,
            for_votes: 0,
            against_votes: 0,
            abstain_votes: 0,
            canceled: false,
            queued: false,
            executed: false,
            exec_data: execution_data,
        };

        // O(1): writes only this proposal's own key, not the whole set.
        put_proposal(&env, id, &proposal);
        env.storage().instance().set(&P_COUNT, &id);

        env.events()
            .publish((symbol_short!("Propose"), id), proposer);
        Ok(id)
    }

    pub fn cast_vote(
        env: Env,
        proposal_id: u64,
        voter: Address,
        support: u32,
    ) -> Result<(), Error> {
        voter.require_auth();
        // Reuse cached instance read - no extra storage round-trip.
        let cfg = get_cfg(&env)?;
        let mut p = get_proposal(&env, proposal_id)?;

        let t = now(&env);
        if t < p.start_time || t > p.end_time {
            return Err(Error::VotingClosed);
        }
        if p.canceled || p.executed || p.queued {
            return Err(Error::InvalidState);
        }

        // O(1): touches only this (proposal_id, voter) vote key.
        let vote_key = DataKey::Vote(proposal_id, voter.clone());
        if env.storage().persistent().has(&vote_key) {
            return Err(Error::AlreadyVoted);
        }

        let weight = Self::get_power(&env, &cfg, &voter);
        if weight == 0 {
            return Err(Error::NoVotingPower);
        }

        match support {
            0 => p.against_votes = p.against_votes.checked_add(weight).ok_or(Error::Overflow)?,
            1 => p.for_votes = p.for_votes.checked_add(weight).ok_or(Error::Overflow)?,
            2 => p.abstain_votes = p.abstain_votes.checked_add(weight).ok_or(Error::Overflow)?,
            _ => return Err(Error::InvalidVoteType),
        }

        env.storage().persistent().set(&vote_key, &support);
        put_proposal(&env, proposal_id, &p);

        env.events().publish(
            (symbol_short!("Vote"), proposal_id),
            (voter, support, weight),
        );
        Ok(())
    }

    pub fn state(env: Env, proposal_id: u64) -> Result<u32, Error> {
        let cfg = get_cfg(&env)?;
        let p = get_proposal(&env, proposal_id)?;
        let t = now(&env);

        if p.canceled {
            return Ok(2);
        }
        if p.executed {
            return Ok(5);
        }
        if p.queued {
            return Ok(4);
        }

        if let Some(dispute_addr) = cfg.dispute_contract {
            let args = vec![&env, proposal_id.into_val(&env)];
            let is_disputed: bool =
                env.invoke_contract(&dispute_addr, &Symbol::new(&env, "is_disputed"), args);
            if is_disputed {
                return Ok(6);
            }
        }

        if t < p.start_time {
            return Ok(0);
        }
        if t <= p.end_time {
            return Ok(1);
        }

        if p.for_votes > p.against_votes {
            return Ok(3);
        }

        Ok(2)
    }

    pub fn queue(env: Env, proposal_id: u64) -> Result<(), Error> {
        let state = Self::state(env.clone(), proposal_id)?;
        if state != 3 {
            return Err(Error::ProposalNotSuccessful);
        }

        let mut p = get_proposal(&env, proposal_id)?;
        p.queued = true;
        put_proposal(&env, proposal_id, &p);

        env.events()
            .publish((symbol_short!("Queue"), proposal_id), ());
        Ok(())
    }

    pub fn execute(env: Env, proposal_id: u64) -> Result<(), Error> {
        let mut p = get_proposal(&env, proposal_id)?;

        if !p.queued {
            return Err(Error::NotQueued);
        }
        if p.executed {
            return Err(Error::AlreadyExecuted);
        }

        let cfg = get_cfg(&env)?;
        if let Some(dispute_addr) = cfg.dispute_contract {
            let args = vec![&env, proposal_id.into_val(&env)];
            let is_disputed: bool =
                env.invoke_contract(&dispute_addr, &Symbol::new(&env, "is_disputed"), args);
            if is_disputed {
                return Err(Error::ProposalDisputed);
            }
        }

        p.executed = true;
        put_proposal(&env, proposal_id, &p);

        env.events()
            .publish((symbol_short!("Execute"), proposal_id), ());
        Ok(())
    }

    /// One-time migration for pre-upgrade deployments: copies every
    /// proposal/vote out of the legacy bulk `PROPS`/`VOTES` maps into
    /// individual `DataKey::Proposal`/`DataKey::Vote` entries, then
    /// removes the legacy bulk keys. Gated to the configured `timelock`
    /// address (Governor has no separate admin role). Idempotent: once
    /// the legacy keys are gone, subsequent calls are a cheap no-op that
    /// return `0`.
    ///
    /// Returns the number of proposals migrated.
    pub fn migrate_storage(env: Env, caller: Address) -> Result<u32, Error> {
        caller.require_auth();
        let cfg = get_cfg(&env)?;
        if caller != cfg.timelock {
            return Err(Error::Unauthorized);
        }

        let mut migrated: u32 = 0;
        if env.storage().persistent().has(&PROPS) {
            let props: Map<u64, Proposal> = env
                .storage()
                .persistent()
                .get(&PROPS)
                .unwrap_or(Map::new(&env));
            for (id, proposal) in props.iter() {
                put_proposal(&env, id, &proposal);
                migrated = migrated.saturating_add(1);
            }
            env.storage().persistent().remove(&PROPS);
        }

        if env.storage().persistent().has(&VOTES) {
            let votes: Map<(u64, Address), u32> = env
                .storage()
                .persistent()
                .get(&VOTES)
                .unwrap_or(Map::new(&env));
            for ((proposal_id, voter), support) in votes.iter() {
                env.storage()
                    .persistent()
                    .set(&DataKey::Vote(proposal_id, voter), &support);
            }
            env.storage().persistent().remove(&VOTES);
        }

        Ok(migrated)
    }

    fn get_power(env: &Env, cfg: &GovernorConfig, voter: &Address) -> i128 {
        let token_args = vec![&env, voter.into_val(env)];
        let balance: i128 =
            env.invoke_contract(&cfg.token, &Symbol::new(&env, "balance_of"), token_args);

        let rep: i128 = if let Some(rep_addr) = &cfg.rep_contract {
            let rep_args = vec![&env, voter.into_val(env)];
            env.invoke_contract(rep_addr, &Symbol::new(&env, "get_score"), rep_args)
        } else {
            0
        };

        balance.saturating_add(rep)
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::panic)] // Unwrap is intentionally used in this contract context
mod test {
    use super::*;
    use soroban_sdk::testutils::{Address as _, Ledger};

    #[contract]
    pub struct MockToken;
    #[contractimpl]
    impl MockToken {
        pub fn balance_of(env: Env, user: Address) -> i128 {
            let key = (symbol_short!("bal"), user);
            read_or_default::<_, i128>(&env, &key)
        }

        pub fn set_bal(env: Env, user: Address, amount: i128) {
            let key = (symbol_short!("bal"), user);
            env.storage().instance().set(&key, &amount);
        }
    }

    #[test]
    fn lifecycle_succeeds() {
        let env = Env::default();
        env.mock_all_auths();

        let token_id = env.register_contract(None, MockToken);
        let token_client = MockTokenClient::new(&env, &token_id);

        let tl = Address::generate(&env);
        let voter = Address::generate(&env);

        let gov_id = env.register_contract(None, Governor);
        let gov_client = GovernorClient::new(&env, &gov_id);

        gov_client.initialize(&token_id, &tl, &5, &10, &100, &1, &None, &None);

        token_client.set_bal(&voter, &200);

        let prop_id = gov_client.propose(
            &voter,
            &Bytes::from_array(&env, &[1, 2, 3]),
            &Bytes::from_array(&env, &[0]),
        );

        env.ledger().set_timestamp(env.ledger().timestamp() + 6);
        assert_eq!(gov_client.state(&prop_id), 1);

        gov_client.cast_vote(&prop_id, &voter, &1);

        env.ledger().set_timestamp(env.ledger().timestamp() + 20);
        assert_eq!(gov_client.state(&prop_id), 3);

        gov_client.queue(&prop_id);
        assert_eq!(gov_client.state(&prop_id), 4);

        gov_client.execute(&prop_id);
        assert_eq!(gov_client.state(&prop_id), 5);
    }

    #[test]
    fn test_error_codes_are_stable() {
        assert_eq!(Error::NotInitialized as u32, 300);
        assert_eq!(Error::AlreadyInitialized as u32, 301);
        assert_eq!(Error::InvalidState as u32, 304);
        assert_eq!(Error::ProposalNotFound as u32, 450);
        assert_eq!(Error::NoVotingPower as u32, 531);
    }

    #[test]
    fn test_get_suggestion_returns_expected_hint() {
        use soroban_sdk::symbol_short;
        assert_eq!(
            crate::errors::get_suggestion(Error::NotInitialized),
            symbol_short!("INIT_CTR")
        );
        assert_eq!(
            crate::errors::get_suggestion(Error::AlreadyInitialized),
            symbol_short!("ALREADY")
        );
        assert_eq!(
            crate::errors::get_suggestion(Error::ProposalNotFound),
            symbol_short!("CHK_ID")
        );
        assert_eq!(
            crate::errors::get_suggestion(Error::VotingClosed),
            symbol_short!("RE_TRY_L")
        );
    }

    /// Locks down storage isolation: after this migration, proposals and
    /// votes live under individual `DataKey` entries rather than the old
    /// bulk `PROPS`/`VOTES` maps, so voting on one proposal cannot affect
    /// another's state.
    #[test]
    fn test_proposals_are_isolated_per_key() {
        let env = Env::default();
        env.mock_all_auths();
        env.budget().reset_unlimited();
        let token_id = env.register_contract(None, MockToken);
        let token_client = MockTokenClient::new(&env, &token_id);
        let tl = Address::generate(&env);
        let voter = Address::generate(&env);
        let gov_id = env.register_contract(None, Governor);
        let gov_client = GovernorClient::new(&env, &gov_id);
        gov_client.initialize(&token_id, &tl, &5, &10, &100, &1, &None, &None);
        token_client.set_bal(&voter, &200);

        let id1 = gov_client.propose(
            &voter,
            &Bytes::from_array(&env, &[1]),
            &Bytes::from_array(&env, &[0]),
        );
        let id2 = gov_client.propose(
            &voter,
            &Bytes::from_array(&env, &[2]),
            &Bytes::from_array(&env, &[0]),
        );

        env.ledger().set_timestamp(env.ledger().timestamp() + 6);
        gov_client.cast_vote(&id1, &voter, &1);

        // Voting on proposal 1 must not affect proposal 2's state.
        assert_eq!(gov_client.state(&id2), 1);
    }

    /// Migration is idempotent and correctly reports zero once there is
    /// nothing left in the legacy bulk maps (the normal case for a
    /// freshly-initialized contract).
    #[test]
    fn test_migrate_storage_noop_on_fresh_contract() {
        let env = Env::default();
        env.mock_all_auths();
        env.budget().reset_unlimited();
        let token_id = env.register_contract(None, MockToken);
        let tl = Address::generate(&env);
        let gov_id = env.register_contract(None, Governor);
        let gov_client = GovernorClient::new(&env, &gov_id);
        gov_client.initialize(&token_id, &tl, &5, &10, &100, &1, &None, &None);

        let migrated = gov_client.migrate_storage(&tl);
        assert_eq!(migrated, 0);
    }

    /// Migration is gated to the timelock address.
    #[test]
    fn test_migrate_storage_rejects_non_timelock_caller() {
        let env = Env::default();
        env.mock_all_auths();
        env.budget().reset_unlimited();
        let token_id = env.register_contract(None, MockToken);
        let tl = Address::generate(&env);
        let intruder = Address::generate(&env);
        let gov_id = env.register_contract(None, Governor);
        let gov_client = GovernorClient::new(&env, &gov_id);
        gov_client.initialize(&token_id, &tl, &5, &10, &100, &1, &None, &None);

        let result = gov_client.try_migrate_storage(&intruder);
        assert!(result.is_err());
    }

    /// Benchmark: proving O(1) storage cost. Proposal #1000 should cost
    /// roughly the same CPU budget as proposal #1 - with the legacy
    /// bulk-map scheme this would grow roughly linearly with the number
    /// of proposals already stored, since every write re-serialized the
    /// whole collection.
    #[test]
    fn bench_propose_cost_is_constant_not_linear() {
        let env = Env::default();
        env.mock_all_auths();
        env.budget().reset_unlimited();
        let token_id = env.register_contract(None, MockToken);
        let token_client = MockTokenClient::new(&env, &token_id);
        let tl = Address::generate(&env);
        let voter = Address::generate(&env);
        let gov_id = env.register_contract(None, Governor);
        let gov_client = GovernorClient::new(&env, &gov_id);
        gov_client.initialize(&token_id, &tl, &5, &10, &100, &1, &None, &None);
        token_client.set_bal(&voter, &1_000_000);

        // Cost of proposal #1 in a fresh contract.
        env.budget().reset_default();
        gov_client.propose(
            &voter,
            &Bytes::from_array(&env, &[1]),
            &Bytes::from_array(&env, &[0]),
        );
        let cost_first = env.budget().cpu_instruction_cost();

        // Fill up to proposal #999 (unmeasured).
        for _ in 0..998 {
            gov_client.propose(
                &voter,
                &Bytes::from_array(&env, &[1]),
                &Bytes::from_array(&env, &[0]),
            );
        }

        // Cost of proposal #1000.
        env.budget().reset_default();
        gov_client.propose(
            &voter,
            &Bytes::from_array(&env, &[1]),
            &Bytes::from_array(&env, &[0]),
        );
        let cost_thousandth = env.budget().cpu_instruction_cost();

        // With per-item keys, cost should stay roughly constant (O(1)),
        // not grow linearly with the number of stored proposals (O(n)).
        // Generous tolerance since exact instruction counts can vary
        // slightly with ledger/event state size.
        let upper_bound = cost_first + (cost_first / 2) + 1;
        assert!(
            cost_thousandth <= upper_bound,
            "proposal #1000 cost {} exceeds tolerance over proposal #1 cost {} (bound {})",
            cost_thousandth,
            cost_first,
            upper_bound
        );
    }
}
