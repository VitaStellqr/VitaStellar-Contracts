// Fuzz / property-based tests for `contract_usage_analytics`.
//
// # Naming
// The GitHub issue (#82) refers to `record_event`. The current public
// ingest entrypoint in `src/lib.rs` is `record_call`. This harness covers
// `record_call`, which is the only public mutating endpoint that accepts
// arbitrary payloads (`function_name: String`) and unbounded metrics
// (`cpu_usage: u64`, `ram_usage: u64`, `latency_ms: u64`). Issue #82's
// `{"caller", "payload", "metadata"}` shape maps onto
// `{user, function_name, (cpu_usage | ram_usage | success | latency_ms)}`.
//
// # Why proptest instead of `cargo-fuzz`
// `cargo-fuzz` requires nightly Rust and the `arbitrary` crate. The
// workspace pins stable (`rust-toolchain.toml` channel = "1.92.0") and
// explicitly notes that `arbitrary` was removed to fix workspace
// inheritance (`Cargo.toml`). `proptest` is a stable-Rust alternative
// and is the convention in `scripts/run_contract_fuzz.sh`.
//
// Run with:
//   cargo test -p contract_usage_analytics --test fuzz_record_event
//   PROPTEST_CASES=2000 cargo test -p contract_usage_analytics --test fuzz_record_event
//
// `PROPTEST_CASES` is honoured by proptest and overrides the in-file
// default. Issue #82 asks for a harness that can run "≥ 5 minutes
// without crashes" — bump `PROPTEST_CASES` to a few thousand for that.

#![allow(clippy::unwrap_used)] // tests

use contract_usage_analytics::{ContractUsageAnalytics, ContractUsageAnalyticsClient};
use proptest::prelude::*;
use soroban_sdk::{testutils::Address as _, Address, Env, String};

/// Strategy for a short ASCII string (≤ 40 chars). Soroban's `String`
/// type rejects strings longer than ~63 bytes, so we stay well below
/// the limit and also keep generated inputs cheap.
fn short_string() -> BoxedStrategy<String> {
    ".{1,40}".boxed()
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(256))]

    /// `record_call` must return `Ok(())` for any valid-shaped input.
    /// It is the project-perimeter mutating endpoint, so a contract
    /// `Err` on valid input is a critical regression.
    #[test]
    fn record_call_returns_ok_for_valid_input(
        function_name in short_string(),
        cpu_usage in any::<u64>(),
        ram_usage in any::<u64>(),
        success in any::<bool>(),
        latency_ms in any::<u64>(),
    ) {
        let env = Env::default();
        env.mock_all_auths();

        let contract_id = env.register_contract(None, ContractUsageAnalytics);
        let client = ContractUsageAnalyticsClient::new(&env, &contract_id);
        let admin = Address::generate(&env);
        client.initialize(&admin);

        let user = Address::generate(&env);
        let fname = String::from_str(&env, &function_name);

        let result = client.try_record_call(
            &fname,
            &user,
            &cpu_usage,
            &ram_usage,
            &success,
            &latency_ms,
        );
        // Soroban's `try_*` calls return
        // `Result<Result<(), ContractError>, InvokeError>`. We require
        // both: the invocation did not panic AND the contract returned
        // `Ok(())`.
        prop_assert!(
            matches!(result, Ok(Ok(()))),
            "record_call must return Ok(()); got {:?}",
            result
        );

        // The function name must now be registered for retrieval.
        let funcs = client.get_all_functions();
        prop_assert!(
            funcs.iter().any(|f| f == fname),
            "function_name was not registered in `get_all_functions`"
        );
        prop_assert!(
            client.get_function_metrics(&fname).is_some(),
            "function metrics should be present after a successful record_call"
        );
    }

    /// Two identical `record_call` invocations must produce a `call_count`
    /// of exactly 2 — i.e., callers cannot lose events to silent dedupe.
    #[test]
    fn record_call_is_not_silently_deduped(
        function_name in short_string(),
        cpu_usage in any::<u64>(),
        ram_usage in any::<u64>(),
        latency_ms in any::<u64>(),
    ) {
        let env = Env::default();
        env.mock_all_auths();

        let contract_id = env.register_contract(None, ContractUsageAnalytics);
        let client = ContractUsageAnalyticsClient::new(&env, &contract_id);
        let admin = Address::generate(&env);
        client.initialize(&admin);

        let user = Address::generate(&env);
        let fname = String::from_str(&env, &function_name);

        client.record_call(&fname, &user, &cpu_usage, &ram_usage, &true, &latency_ms);
        client.record_call(&fname, &user, &cpu_usage, &ram_usage, &true, &latency_ms);

        let metric = client
            .get_function_metrics(&fname)
            .expect("metric present after two calls");
        prop_assert_eq!(metric.call_count, 2);
        prop_assert_eq!(metric.error_count, 0);

        let user_metric = client
            .get_user_metrics(&user)
            .expect("user metric present after two calls");
        prop_assert_eq!(user_metric.total_calls, 2);
    }

    /// `success=false` calls must increment the error counter; arbitrary
    /// lengths of error sequences must not panic.
    #[test]
    fn record_call_error_counter_increments(
        function_name in short_string(),
        calls in 1u32..32,
    ) {
        let env = Env::default();
        env.mock_all_auths();

        let contract_id = env.register_contract(None, ContractUsageAnalytics);
        let client = ContractUsageAnalyticsClient::new(&env, &contract_id);
        let admin = Address::generate(&env);
        client.initialize(&admin);

        let user = Address::generate(&env);
        let fname = String::from_str(&env, &function_name);

        for _ in 0..calls {
            let _ = client.try_record_call(
                &fname,
                &user,
                &0u64,
                &0u64,
                &false,
                &0u64,
            );
        }

        let metric = client
            .get_function_metrics(&fname)
            .expect("metric present after series");
        prop_assert_eq!(metric.call_count, u64::from(calls));
        prop_assert_eq!(metric.error_count, u64::from(calls));
    }

    /// `record_call` must keep `get_all_functions` deduplicated across
    /// many identical calls. If a future regression pushed the function
    /// name without a uniqueness check, this test catches the
    /// unbounded-growth failure mode (the "known crash" the issue
    /// references).
    #[test]
    fn record_call_does_not_duplicate_function_names(
        function_name in short_string(),
        calls in 1u32..16,
    ) {
        let env = Env::default();
        env.mock_all_auths();

        let contract_id = env.register_contract(None, ContractUsageAnalytics);
        let client = ContractUsageAnalyticsClient::new(&env, &contract_id);
        let admin = Address::generate(&env);
        client.initialize(&admin);

        let user = Address::generate(&env);
        let fname = String::from_str(&env, &function_name);

        for _ in 0..calls {
            client.record_call(&fname, &user, &0u64, &0u64, &true, &0u64);
        }

        let funcs = client.get_all_functions();
        let appearances: u32 = funcs
            .iter()
            .filter(|f| f == fname)
            .count()
            .try_into()
            .expect("u32 fits in appearance count");
        prop_assert_eq!(
            appearances, 1,
            "function_name appeared {} times in get_all_functions (must be exactly 1)",
            appearances
        );
    }
}
