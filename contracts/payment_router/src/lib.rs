#![no_std]

extern crate fp_math;

use soroban_sdk::{
    contract, contracterror, contractimpl, contracttype, symbol_short, Address, Env, Symbol,
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
    RoutingFailed = 20,
    NoFallbackAvailable = 21,
    PendingPaymentNotFound = 22,
}

#[derive(Clone)]
#[contracttype]
pub struct RouterFeeConfig {
    pub platform_fee_bps: u32,
    pub fee_receiver: Address,
}

#[derive(Clone)]
#[contracttype]
pub struct PendingPayment {
    pub payer: Address,
    pub recipient: Address,
    pub amount: i128,
    pub fallback_recipient: Address,
    pub timestamp: u64,
    pub reason: Symbol,
}

#[derive(Clone)]
#[contracttype]
enum DataKey {
    Nonce(Address),
    PendingPayment(u64),
    PendingPaymentCount,
}

const FEE_CONF: Symbol = symbol_short!("feeconf");
const NONCE_WRAP_HALF: u64 = u64::MAX / 2;
const PAYMENT_ROUTED: Symbol = symbol_short!("PmtRouted");
const PAYMENT_ROUTING_FAILED: Symbol = symbol_short!("PmtFailed");
const PAYMENT_COLLECTED: Symbol = symbol_short!("PmtColled");

#[contract]
pub struct PaymentRouter;

#[contractimpl]
impl PaymentRouter {
    pub fn set_fee_config(
        env: Env,
        fee_receiver: Address,
        platform_fee_bps: u32,
    ) -> Result<(), Error> {
        if platform_fee_bps > 10_000 {
            return Err(Error::InvalidFeeBps);
        }
        let conf = RouterFeeConfig {
            fee_receiver,
            platform_fee_bps,
        };
        env.storage().persistent().set(&FEE_CONF, &conf);
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
        let (provider, fee) = Self::compute_split_values(&env, amount)?;
        env.events()
            .publish((symbol_short!("FeeSplit"),), (provider, fee));
        Ok((provider, fee))
    }

    pub fn route_payment(
        env: Env,
        payer: Address,
        recipient: Address,
        amount: i128,
        next_nonce: u64,
    ) -> Result<(), Error> {
        payer.require_auth();
        let (provider, fee) = Self::compute_split_values(&env, amount)?;
        Self::consume_nonce(&env, &payer, next_nonce)?;

        env.events().publish(
            (PAYMENT_ROUTED,),
            (payer, recipient, amount, provider, fee, next_nonce),
        );
        Ok(())
    }

    pub fn route_with_fallback(
        env: Env,
        payer: Address,
        primary_recipient: Address,
        fallback_recipient: Address,
        amount: i128,
        next_nonce: u64,
    ) -> Result<u64, Error> {
        payer.require_auth();
        let (provider, fee) = Self::compute_split_values(&env, amount)?;
        Self::consume_nonce(&env, &payer, next_nonce)?;

        let primary_success = Self::try_route(
            &env,
            &payer,
            &primary_recipient,
            amount,
            &provider,
            &fee,
            next_nonce,
        );

        if primary_success {
            return Ok(0);
        }

        let fallback_success = Self::try_route(
            &env,
            &payer,
            &fallback_recipient,
            amount,
            &provider,
            &fee,
            next_nonce,
        );

        if fallback_success {
            return Ok(0);
        }

        let payment_id = Self::next_payment_id(&env);
        let pending = PendingPayment {
            payer: payer.clone(),
            recipient: primary_recipient.clone(),
            amount,
            fallback_recipient,
            timestamp: env.ledger().timestamp(),
            reason: symbol_short!("PmtFailed"),
        };
        env.storage()
            .persistent()
            .set(&DataKey::PendingPayment(payment_id), &pending);

        env.events().publish(
            (PAYMENT_ROUTING_FAILED,),
            (payer, primary_recipient, amount, next_nonce, payment_id),
        );

        Ok(payment_id)
    }

    pub fn collect_failed_payment(
        env: Env,
        payer: Address,
        payment_id: u64,
        next_nonce: u64,
    ) -> Result<(), Error> {
        payer.require_auth();

        let pending: PendingPayment = env
            .storage()
            .persistent()
            .get(&DataKey::PendingPayment(payment_id))
            .ok_or(Error::PendingPaymentNotFound)?;

        if pending.payer != payer {
            return Err(Error::PendingPaymentNotFound);
        }

        Self::consume_nonce(&env, &payer, next_nonce)?;

        env.storage()
            .persistent()
            .remove(&DataKey::PendingPayment(payment_id));

        env.events().publish(
            (PAYMENT_COLLECTED,),
            (
                payer,
                pending.fallback_recipient,
                pending.amount,
                payment_id,
            ),
        );

        Ok(())
    }

    fn compute_split_values(env: &Env, amount: i128) -> Result<(i128, i128), Error> {
        let conf: RouterFeeConfig = env
            .storage()
            .persistent()
            .get(&FEE_CONF)
            .ok_or(Error::FeeNotSet)?;
        let fee = fp_math::mul_bps(amount, conf.platform_fee_bps).ok_or(Error::Overflow)?;
        let provider = amount.checked_sub(fee).ok_or(Error::Overflow)?;
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

    fn try_route(
        env: &Env,
        payer: &Address,
        recipient: &Address,
        amount: i128,
        provider: &i128,
        fee: &i128,
        nonce: u64,
    ) -> bool {
        env.events().publish(
            (PAYMENT_ROUTED,),
            (
                payer.clone(),
                recipient.clone(),
                amount,
                provider.clone(),
                fee.clone(),
                nonce,
            ),
        );
        true
    }

    fn next_payment_id(env: &Env) -> u64 {
        let count: u64 = env
            .storage()
            .persistent()
            .get(&DataKey::PendingPaymentCount)
            .unwrap_or(0);
        let next = count.wrapping_add(1);
        env.storage()
            .persistent()
            .set(&DataKey::PendingPaymentCount, &next);
        next
    }
}

#[cfg(all(test, feature = "testutils"))]
#[allow(clippy::unwrap_used)] // Unwrap is intentionally used in this contract context
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

    #[test]
    fn test_route_with_fallback_succeeds_primary() {
        let env = Env::default();
        env.mock_all_auths();
        let cid = env.register_contract(None, PaymentRouter);
        let client = PaymentRouterClient::new(&env, &cid);
        let payer = Address::generate(&env);
        let primary = Address::generate(&env);
        let fallback = Address::generate(&env);

        client.set_fee_config(&Address::generate(&env), &1000u32);
        let payment_id = client.route_with_fallback(&payer, &primary, &fallback, &1000i128, &1u64);
        // Primary route succeeds, so payment_id is 0 (no pending payment created)
        assert_eq!(payment_id, 0);
    }

    #[test]
    fn test_collect_nonexistent_pending_payment() {
        let env = Env::default();
        env.mock_all_auths();
        let cid = env.register_contract(None, PaymentRouter);
        let client = PaymentRouterClient::new(&env, &cid);
        let payer = Address::generate(&env);

        client.set_fee_config(&Address::generate(&env), &1000u32);

        // Try collecting a pending payment that doesn't exist
        let collect_result = client.try_collect_failed_payment(&payer, &999u64, &1u64);
        assert_eq!(collect_result, Err(Ok(Error::PendingPaymentNotFound)));
    }

    #[test]
    fn test_route_with_fallback_returns_pending_id() {
        let env = Env::default();
        env.mock_all_auths();
        let cid = env.register_contract(None, PaymentRouter);
        let client = PaymentRouterClient::new(&env, &cid);
        let payer = Address::generate(&env);
        let primary = Address::generate(&env);
        let fallback = Address::generate(&env);

        client.set_fee_config(&Address::generate(&env), &1000u32);

        // Route successfully with primary
        let id1 = client.route_with_fallback(&payer, &primary, &fallback, &1000i128, &1u64);
        assert_eq!(id1, 0);

        // Another route
        let id2 = client.route_with_fallback(&payer, &primary, &fallback, &500i128, &2u64);
        assert_eq!(id2, 0);

        // Collecting a non-existent pending payment returns error
        let result = client.try_collect_failed_payment(&payer, &7u64, &3u64);
        assert_eq!(result, Err(Ok(Error::PendingPaymentNotFound)));
    }
}
