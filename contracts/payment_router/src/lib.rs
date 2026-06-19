#![no_std]

extern crate fp_math;

use soroban_sdk::{
    contract, contracterror, contractimpl, contracttype, symbol_short, Address, Env, String,
    Symbol,
};

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum Error {
    InvalidFeeBps = 1,
    FeeNotSet = 2,
    Overflow = 3,
    InsufficientFunds = 10,
    DeadlineExceeded = 11,
    InvalidSignature = 12,
    UnauthorizedCaller = 13,
    ContractPaused = 14,
    StorageFull = 15,
    CrossChainTimeout = 16,
    ReplayDetected = 17,
}

#[derive(Clone)]
#[contracttype]
pub struct RouterFeeConfig {
    pub platform_fee_bps: u32,
    pub fee_receiver: Address,
}

#[derive(Clone)]
#[contracttype]
pub enum MetricsConfig {
    Enabled,
    Disabled,
}

const FEE_CONF: Symbol = symbol_short!("feeconf");
const METRICS_ENABLED: Symbol = symbol_short!("mtrcsen");

// Helper function to emit metrics (no-op if metrics are disabled)
fn emit_metric(
    env: &Env,
    function_name: &str,
    caller: &Address,
    success: bool,
    cpu_usage: u64,
) {
    let metrics_config: MetricsConfig = env
        .storage()
        .instance()
        .get(&METRICS_ENABLED)
        .unwrap_or(MetricsConfig::Disabled);

    match metrics_config {
        MetricsConfig::Enabled => {
            // Publish a metric event that contract_usage_analytics will consume
            env.events().publish(
                (symbol_short!("metric"),),
                (
                    String::from_small_str(function_name),
                    caller,
                    success,
                    cpu_usage,
                ),
            );
        }
        MetricsConfig::Disabled => {
            // Metrics are disabled, no-op
        }
    }
}
enum DataKey {
    Nonce(Address),
}

const FEE_CONF: Symbol = symbol_short!("feeconf");
const NONCE_WRAP_HALF: u64 = u64::MAX / 2;

#[contract]
pub struct PaymentRouter;

#[contractimpl]
impl PaymentRouter {
    pub fn enable_metrics(env: Env) -> Result<(), Error> {
        env.storage()
            .instance()
            .set(&METRICS_ENABLED, &MetricsConfig::Enabled);
        Ok(())
    }

    pub fn disable_metrics(env: Env) -> Result<(), Error> {
        env.storage()
            .instance()
            .set(&METRICS_ENABLED, &MetricsConfig::Disabled);
        Ok(())
    }

    pub fn set_fee_config(
        env: Env,
        fee_receiver: Address,
        platform_fee_bps: u32,
    ) -> Result<(), Error> {
        if platform_fee_bps > 10_000 {
            emit_metric(&env, "set_fee_config", &fee_receiver, false, 0);
            return Err(Error::InvalidFeeBps);
        }
        let conf = RouterFeeConfig {
            fee_receiver: fee_receiver.clone(),
            platform_fee_bps,
        };
        env.storage().persistent().set(&FEE_CONF, &conf);
        emit_metric(&env, "set_fee_config", &fee_receiver, true, 0);
        Ok(())
    }

    pub fn get_fee_config(env: Env) -> Option<RouterFeeConfig> {
        env.storage().persistent().get(&FEE_CONF)
    }

    pub fn get_nonce(env: Env, caller: Address) -> u64 {
        env.storage()
            .persistent()
            .get(&DataKey::Nonce(caller))
            .unwrap_or(0)
    }

    pub fn compute_split(env: Env, amount: i128) -> Result<(i128, i128), Error> {
        let conf: RouterFeeConfig = match env.storage().persistent().get(&FEE_CONF) {
            Some(c) => c,
            None => {
                return Err(Error::FeeNotSet);
            }
        };
        let fee =
            fp_math::mul_bps(amount, conf.platform_fee_bps).ok_or(Error::Overflow)?;
        let provider = amount.saturating_sub(fee);
        env.events()
            .publish((symbol_short!("FeeSplit"),), (provider, fee));
        
        // Emit metric for compute_split
        emit_metric(&env, "compute_split", &conf.fee_receiver, true, 0);
        
        Ok((provider, fee))
    }

    fn consume_nonce(env: &Env, caller: &Address, next_nonce: u64) -> Result<(), Error> {
        let stored_nonce: u64 = env
            .storage()
            .persistent()
            .get(&DataKey::Nonce(caller.clone()))
            .unwrap_or(0);
        if !Self::nonce_is_newer(next_nonce, stored_nonce) {
            return Err(Error::ReplayDetected);
        }

        env.storage()
            .persistent()
            .set(&DataKey::Nonce(caller.clone()), &next_nonce);
        env.events()
            .publish(("NonceConsumed",), (caller.clone(), next_nonce));
        Ok(())
    }

    fn nonce_is_newer(next_nonce: u64, stored_nonce: u64) -> bool {
        let delta = next_nonce.wrapping_sub(stored_nonce);
        delta != 0 && delta <= NONCE_WRAP_HALF
    }
}

#[cfg(all(test, feature = "testutils"))]
#[allow(clippy::unwrap_used)]
mod test {
    use super::*;
    use soroban_sdk::testutils::Address as _;
    use soroban_sdk::{Address, Env};

    #[test]
    fn test_fee_split() {
        let env = Env::default();
        let cid = env.register_contract(None, PaymentRouter);
        let client = PaymentRouterClient::new(&env, &cid);
        // Soroban contract clients auto-unwrap Result types
        client.set_fee_config(&Address::generate(&env), &1000u32); // 10%
        let (provider, fee) = client.compute_split(&1000i128);
        assert_eq!(provider, 900);
        assert_eq!(fee, 100);
    }

    #[test]
    fn route_payment_rejects_replay_and_accepts_next_nonce() {
        let env = Env::default();
        env.mock_all_auths();
        let cid = env.register_contract(None, PaymentRouter);
        let client = PaymentRouterClient::new(&env, &cid);
        let payer = Address::generate(&env);
        let recipient = Address::generate(&env);

        client.set_fee_config(&Address::generate(&env), &1000u32);
        client.route_payment(&payer, &recipient, &1000i128, &1u64);

        let replay = client.try_route_payment(&payer, &recipient, &1000i128, &1u64);
        assert_eq!(replay, Err(Ok(Error::ReplayDetected)));

        client.route_payment(&payer, &recipient, &1000i128, &2u64);
    }

    #[test]
    fn route_payment_accepts_sequential_nonces_and_rejects_out_of_order() {
        let env = Env::default();
        env.mock_all_auths();
        let cid = env.register_contract(None, PaymentRouter);
        let client = PaymentRouterClient::new(&env, &cid);
        let payer = Address::generate(&env);

        client.set_fee_config(&Address::generate(&env), &1000u32);
        for nonce in 1..=100u64 {
            let recipient = Address::generate(&env);
            client.route_payment(&payer, &recipient, &(nonce as i128), &nonce);
        }

        let recipient = Address::generate(&env);
        let replay = client.try_route_payment(&payer, &recipient, &50i128, &50u64);
        assert_eq!(replay, Err(Ok(Error::ReplayDetected)));
    }

    #[test]
    fn route_payment_wraps_nonce_at_u64_max() {
        let env = Env::default();
        env.mock_all_auths();
        let cid = env.register_contract(None, PaymentRouter);
        let client = PaymentRouterClient::new(&env, &cid);
        let payer = Address::generate(&env);
        let first_recipient = Address::generate(&env);
        let second_recipient = Address::generate(&env);

        client.set_fee_config(&Address::generate(&env), &1000u32);
        client.route_payment(&payer, &first_recipient, &1i128, &1u64);
        client.route_payment(&payer, &first_recipient, &1i128, &((u64::MAX / 2) + 1));
        client.route_payment(&payer, &first_recipient, &1i128, &u64::MAX);
        client.route_payment(&payer, &second_recipient, &2i128, &0u64);

        let replay = client.try_route_payment(&payer, &first_recipient, &1i128, &u64::MAX);
        assert_eq!(replay, Err(Ok(Error::ReplayDetected)));
    }
}
