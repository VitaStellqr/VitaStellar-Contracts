use soroban_sdk::{Address, Env, Map};

use crate::types::{DailyStats, PlatformStats, DAILY_STATS, STATS};

#[allow(clippy::too_many_arguments)]
pub fn update_stats(
    env: &Env,
    volume: i128,
    is_new: bool,
    settled: bool,
    refunded: bool,
    disputed: bool,
    active_delta: i32,
) {
    let mut stats: PlatformStats = env
        .storage()
        .instance()
        .get(&STATS)
        .unwrap_or_else(PlatformStats::zero);

    if is_new {
        stats.total_escrows += 1;
        stats.total_volume = stats.total_volume.saturating_add(volume);

        let day_id = env.ledger().timestamp() / 86400;
        let mut daily_map: Map<u64, DailyStats> = env
            .storage()
            .persistent()
            .get(&DAILY_STATS)
            .unwrap_or(Map::new(env));
        let mut daily = daily_map.get(day_id).unwrap_or(DailyStats {
            day_id,
            volume: 0,
            count: 0,
        });
        daily.volume = daily.volume.saturating_add(volume);
        daily.count += 1;
        daily_map.set(day_id, daily);
        env.storage().persistent().set(&DAILY_STATS, &daily_map);
    }
    if settled {
        stats.settled_count += 1;
    }
    if refunded {
        stats.refunded_count += 1;
    }
    if disputed {
        stats.disputed_count += 1;
    }

    if active_delta > 0 {
        stats.active_count = stats.active_count.saturating_add(active_delta as u64);
    } else if active_delta < 0 {
        stats.active_count = stats
            .active_count
            .saturating_sub(active_delta.unsigned_abs().into());
    }

    env.storage().instance().set(&STATS, &stats);
}

pub fn get_stats_summary(env: &Env) -> PlatformStats {
    env.storage()
        .instance()
        .get(&STATS)
        .unwrap_or_else(PlatformStats::zero)
}

pub fn get_total_volume(env: &Env) -> i128 {
    get_stats_summary(env).total_volume
}

pub fn get_total_escrows(env: &Env) -> u64 {
    get_stats_summary(env).total_escrows
}

pub fn get_settled_rate(env: &Env) -> u32 {
    let s = get_stats_summary(env);
    if s.total_escrows == 0 {
        return 0;
    }
    ((s.settled_count * 10000) / s.total_escrows) as u32
}

pub fn get_refund_rate(env: &Env) -> u32 {
    let s = get_stats_summary(env);
    if s.total_escrows == 0 {
        return 0;
    }
    ((s.refunded_count * 10000) / s.total_escrows) as u32
}

pub fn get_dispute_rate(env: &Env) -> u32 {
    let s = get_stats_summary(env);
    if s.total_escrows == 0 {
        return 0;
    }
    ((s.disputed_count * 10000) / s.total_escrows) as u32
}

pub fn get_active_escrows_count(env: &Env) -> u64 {
    get_stats_summary(env).active_count
}

pub fn get_platform_health_score(env: &Env) -> u32 {
    let s = get_stats_summary(env);
    if s.total_escrows == 0 {
        return 10000;
    }
    let failure_rate = (s.disputed_count + s.refunded_count) * 10000 / s.total_escrows;
    10000u32.saturating_sub(failure_rate as u32)
}

pub fn get_token_volume(env: &Env, _token: Address) -> i128 {
    // Simplified: return global volume (a per-token index would require additional storage)
    get_total_volume(env)
}

pub fn get_donor_reputation(env: &Env, _donor: Address) -> u32 {
    let s = get_stats_summary(env);
    if s.total_escrows == 0 {
        return 5000;
    }
    5000 + (get_settled_rate(env) / 2)
}

pub fn get_daily_stats(env: &Env, day_id: u64) -> Option<DailyStats> {
    let daily_map: Map<u64, DailyStats> = env
        .storage()
        .persistent()
        .get(&DAILY_STATS)
        .unwrap_or(Map::new(env));
    daily_map.get(day_id)
}

#[cfg(all(test, feature = "testutils"))]
mod tests {
    use super::*;
    use crate::{EscrowContract, EscrowContractClient};
    use soroban_sdk::{testutils::Address as _, Address, Env};

    fn setup() -> (Env, EscrowContractClient<'static>) {
        let env = Env::default();
        let cid = env.register_contract(None, EscrowContract);
        let client = EscrowContractClient::new(&env, &cid);
        let admin = Address::generate(&env);
        client.mock_all_auths().initialize(&admin);
        client
            .mock_all_auths()
            .set_fee_config(&admin, &Address::generate(&env), &250u32);
        (env, client)
    }

    #[test]
    fn test_stats_zero_on_empty() {
        let (_, client) = setup();
        assert_eq!(client.get_total_volume(), 0);
        assert_eq!(client.get_total_escrows(), 0);
        assert_eq!(client.get_settled_rate(), 0);
        assert_eq!(client.get_refund_rate(), 0);
        assert_eq!(client.get_dispute_rate(), 0);
        assert_eq!(client.get_active_escrows_count(), 0);
        assert_eq!(client.get_platform_health_score(), 10000);
    }

    #[test]
    fn test_update_stats_new_escrow() {
        let (env, client) = setup();
        let payer = Address::generate(&env);
        let payee = Address::generate(&env);
        let token = Address::generate(&env);
        client
            .mock_all_auths()
            .create_escrow(&1u64, &payer, &payee, &1000i128, &token);
        assert_eq!(client.get_total_volume(), 1000);
        assert_eq!(client.get_total_escrows(), 1);
    }

    #[test]
    fn test_update_stats_settled() {
        let (env, client) = setup();
        let payer = Address::generate(&env);
        let payee = Address::generate(&env);
        let token = Address::generate(&env);
        client
            .mock_all_auths()
            .create_escrow(&1u64, &payer, &payee, &500i128, &token);
        client.mock_all_auths().approve_release(&1u64, &payer);
        client
            .mock_all_auths()
            .approve_release(&1u64, &Address::generate(&env));
        client.release_escrow(&1u64);
        let s = client.get_stats_summary();
        assert_eq!(s.settled_count, 1);
        assert_eq!(s.active_count, 0);
        assert_eq!(client.get_settled_rate(), 10000);
    }

    #[test]
    fn test_health_score_decreases_with_failures() {
        let (env, client) = setup();
        let payer = Address::generate(&env);
        let payee = Address::generate(&env);
        let token = Address::generate(&env);
        client
            .mock_all_auths()
            .create_escrow(&1u64, &payer, &payee, &100i128, &token);
        client.mock_all_auths().mark_disputed(&payer, &1u64);
        assert!(client.get_platform_health_score() < 10000);
    }
}
