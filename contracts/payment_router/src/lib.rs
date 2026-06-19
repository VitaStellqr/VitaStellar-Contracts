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
}
