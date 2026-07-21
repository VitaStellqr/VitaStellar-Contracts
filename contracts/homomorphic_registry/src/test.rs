use super::*;
use soroban_sdk::testutils::Address as _;
use soroban_sdk::testutils::Ledger as _;
use soroban_sdk::{Address, BytesN, Env, String, Vec};

fn setup(env: &Env) -> (HomomorphicRegistryClient<'_>, Address) {
    let id = Address::generate(env);
    env.register_contract(&id, HomomorphicRegistry);
    (HomomorphicRegistryClient::new(env, &id), id)
}

#[test]
fn context_and_submission_flow() {
    let env = Env::default();
    env.mock_all_auths();

    let (client, _id) = setup(&env);
    let admin = Address::generate(&env);
    client.initialize(&admin);

    let ctx_id = BytesN::from_array(&env, &[7u8; 32]);
    let params_ref = String::from_str(&env, "ipfs://he-params");
    let params_hash = BytesN::from_array(&env, &[9u8; 32]);

    client.register_context(
        &admin,
        &ctx_id,
        &HEScheme::Paillier,
        &params_ref,
        &params_hash,
    );

    let submitter = Address::generate(&env);
    let comp_id = BytesN::from_array(&env, &[1u8; 32]);
    let c_ref = String::from_str(&env, "ipfs://ciphertext");
    let c_hash = BytesN::from_array(&env, &[2u8; 32]);
    let empty_proof_ref = String::from_str(&env, "");
    let zero_hash = BytesN::from_array(&env, &[0u8; 32]);
    client.submit_encrypted_computation(
        &submitter,
        &comp_id,
        &ctx_id,
        &c_ref,
        &c_hash,
        &empty_proof_ref,
        &zero_hash,
    );

    let ciphertext_ref = client.get_computation(&comp_id).map(|c| c.ciphertext_ref);
    assert_eq!(ciphertext_ref, Some(c_ref));
}

#[test]
fn ckks_secure_stats_and_ml_inference_flow() {
    let env = Env::default();
    env.mock_all_auths();
    env.ledger().set_timestamp(1_000);
    let (client, _id) = setup(&env);

    let admin = Address::generate(&env);
    let analyst = Address::generate(&env);
    client.initialize(&admin);

    let ckks_ctx = BytesN::from_array(&env, &[11u8; 32]);
    client.register_context(
        &admin,
        &ckks_ctx,
        &HEScheme::CKKS,
        &String::from_str(&env, "ipfs://he/ckks/params-medical-v1"),
        &BytesN::from_array(&env, &[12u8; 32]),
    );

    client.set_performance_profile(&admin, &ckks_ctx, &true, &4096, &true, &true, &8);
    client.register_key_bundle(
        &admin,
        &BytesN::from_array(&env, &[13u8; 32]),
        &ckks_ctx,
        &String::from_str(&env, "kms://hospital-a/public-key-v1"),
        &String::from_str(&env, "kms://hospital-a/eval-key-v1"),
        &String::from_str(&env, "kms://hospital-a/relin-key-v1"),
        &String::from_str(&env, "kms://hospital-a/galois-key-v1"),
        &BytesN::from_array(&env, &[14u8; 32]),
    );

    // Glucose values in mg/dL represented as fixed-point (x100): 101.20, 98.50, 110.30
    let mut glucose = Vec::new(&env);
    glucose.push_back(10_120);
    glucose.push_back(9_850);
    glucose.push_back(11_030);
    let glucose_ct = BytesN::from_array(&env, &[21u8; 32]);
    client.encrypt_ckks_vector(&analyst, &glucose_ct, &ckks_ctx, &glucose, &2);

    let stats = client.encrypted_statistics(&analyst, &glucose_ct);
    assert_eq!(stats.count, 3);
    assert_eq!(stats.mean_scaled, 10_333);
    assert_eq!(stats.min, 9_850);
    assert_eq!(stats.max, 11_030);

    let mut weights = Vec::new(&env);
    // Model weights for [age, bmi, resting_hr], fixed-point x1000
    weights.push_back(12);
    weights.push_back(34);
    weights.push_back(56);

    let mut features = Vec::new(&env);
    // age 44, bmi 27.8, resting_hr 72 -> fixed-point x10
    features.push_back(440);
    features.push_back(278);
    features.push_back(720);
    let features_ct = BytesN::from_array(&env, &[22u8; 32]);
    client.encrypt_ckks_vector(&analyst, &features_ct, &ckks_ctx, &features, &1);

    let score_ct = BytesN::from_array(&env, &[23u8; 32]);
    client.encrypted_linear_inference(&analyst, &score_ct, &features_ct, &weights, &1000);
    let score = client.get_ciphertext(&score_ct);
    assert!(score.is_some(), "expected ciphertext for score_ct");
    let score = match score {
        Some(value) => value,
        None => return,
    };
    assert_eq!(score.slots.len(), 1);
    assert_eq!(score.slots.get(0), Some(56_052));
}

#[test]
fn bgv_exact_computation_and_noise_bootstrap() {
    let env = Env::default();
    env.mock_all_auths();
    env.ledger().set_timestamp(2_000);
    let (client, _id) = setup(&env);

    let admin = Address::generate(&env);
    let submitter = Address::generate(&env);
    client.initialize(&admin);

    let bgv_ctx = BytesN::from_array(&env, &[31u8; 32]);
    client.register_context(
        &admin,
        &bgv_ctx,
        &HEScheme::BGV,
        &String::from_str(&env, "ipfs://he/bgv/params-discrete-v2"),
        &BytesN::from_array(&env, &[32u8; 32]),
    );
    client.set_performance_profile(&admin, &bgv_ctx, &true, &2048, &false, &false, &8);

    // Discrete medical code counts for a cohort across 4 ICD buckets.
    let mut a = Vec::new(&env);
    a.push_back(2);
    a.push_back(5);
    a.push_back(7);
    a.push_back(3);

    let mut b = Vec::new(&env);
    b.push_back(1);
    b.push_back(2);
    b.push_back(3);
    b.push_back(4);

    let a_ct = BytesN::from_array(&env, &[33u8; 32]);
    let b_ct = BytesN::from_array(&env, &[34u8; 32]);
    let sum_ct = BytesN::from_array(&env, &[35u8; 32]);
    let prod_ct = BytesN::from_array(&env, &[36u8; 32]);

    client.encrypt_bgv_vector(&submitter, &a_ct, &bgv_ctx, &a);
    client.encrypt_bgv_vector(&submitter, &b_ct, &bgv_ctx, &b);
    client.fhe_add(&submitter, &sum_ct, &a_ct, &b_ct);
    client.fhe_multiply(&submitter, &prod_ct, &a_ct, &b_ct);

    let sum = client.get_ciphertext(&sum_ct);
    assert!(sum.is_some(), "expected ciphertext for sum_ct");
    let sum = match sum {
        Some(value) => value,
        None => return,
    };
    assert_eq!(sum.slots.get(0), Some(3));
    assert_eq!(sum.slots.get(3), Some(7));

    let prod = client.get_ciphertext(&prod_ct);
    assert!(prod.is_some(), "expected ciphertext for prod_ct");
    let prod = match prod {
        Some(value) => value,
        None => return,
    };
    assert_eq!(prod.slots.get(0), Some(2));
    assert_eq!(prod.slots.get(2), Some(21));
    assert!(prod.noise_budget < 64);

    client.bootstrap_ciphertext(&admin, &prod_ct);
    let refreshed = client.get_ciphertext(&prod_ct);
    assert!(
        refreshed.is_some(),
        "expected refreshed ciphertext for prod_ct"
    );
    let refreshed = match refreshed {
        Some(value) => value,
        None => return,
    };
    assert_eq!(refreshed.noise_budget, 64);
    assert!(refreshed.last_bootstrapped_at > 0);
}

#[test]
fn key_management_rotation_and_cost_optimization() {
    let env = Env::default();
    env.mock_all_auths();
    let (client, _id) = setup(&env);

    let admin = Address::generate(&env);
    client.initialize(&admin);

    let ctx = BytesN::from_array(&env, &[41u8; 32]);
    client.register_context(
        &admin,
        &ctx,
        &HEScheme::CKKS,
        &String::from_str(&env, "ipfs://he/ckks/hospital-b"),
        &BytesN::from_array(&env, &[42u8; 32]),
    );

    client.register_key_bundle(
        &admin,
        &BytesN::from_array(&env, &[43u8; 32]),
        &ctx,
        &String::from_str(&env, "kms://hosp-b/public-v1"),
        &String::from_str(&env, "kms://hosp-b/eval-v1"),
        &String::from_str(&env, "kms://hosp-b/relin-v1"),
        &String::from_str(&env, "kms://hosp-b/galois-v1"),
        &BytesN::from_array(&env, &[44u8; 32]),
    );
    let k1 = client.get_active_key_bundle(&ctx);
    assert!(
        k1.is_some(),
        "expected active key bundle after first registration"
    );
    let k1 = match k1 {
        Some(value) => value,
        None => return,
    };
    assert_eq!(k1.version, 1);

    client.register_key_bundle(
        &admin,
        &BytesN::from_array(&env, &[45u8; 32]),
        &ctx,
        &String::from_str(&env, "kms://hosp-b/public-v2"),
        &String::from_str(&env, "kms://hosp-b/eval-v2"),
        &String::from_str(&env, "kms://hosp-b/relin-v2"),
        &String::from_str(&env, "kms://hosp-b/galois-v2"),
        &BytesN::from_array(&env, &[46u8; 32]),
    );
    let k2 = client.get_active_key_bundle(&ctx);
    assert!(k2.is_some(), "expected active key bundle after rotation");
    let k2 = match k2 {
        Some(value) => value,
        None => return,
    };
    assert_eq!(k2.version, 2);
    assert_eq!(
        k2.public_key_ref,
        String::from_str(&env, "kms://hosp-b/public-v2")
    );

    let base_cost = client.estimate_operation_cost(&ctx, &6, &2048);
    client.set_performance_profile(&admin, &ctx, &true, &4096, &true, &true, &8);
    let optimized_cost = client.estimate_operation_cost(&ctx, &6, &2048);
    assert!(optimized_cost < base_cost);
}

#[test]
fn ciphertext_size_limit_enforced() {
    let env = Env::default();
    env.mock_all_auths();
    let (client, _id) = setup(&env);

    let admin = Address::generate(&env);
    let submitter = Address::generate(&env);
    client.initialize(&admin);

    let ctx_id = BytesN::from_array(&env, &[51u8; 32]);
    client.register_context(
        &admin,
        &ctx_id,
        &HEScheme::CKKS,
        &String::from_str(&env, "ipfs://he/ckks/params"),
        &BytesN::from_array(&env, &[52u8; 32]),
    );

    // Default limit is 64 KB.
    let limit = client.get_ciphertext_limit();
    assert_eq!(limit, 65536);

    // Reduce limit to minimum (1024 bytes).
    client.set_ciphertext_limit(&admin, &1024);

    // 3 slots * 16 = 48 bytes = well under the limit.
    let mut values_ok = Vec::new(&env);
    values_ok.push_back(100);
    values_ok.push_back(200);
    values_ok.push_back(300);
    let ct_id = BytesN::from_array(&env, &[53u8; 32]);
    client.encrypt_ckks_vector(&submitter, &ct_id, &ctx_id, &values_ok, &2);
    assert!(client.get_ciphertext(&ct_id).is_some(), "ciphertext under the limit should be stored");

    // Now set a very tight limit: 48 bytes (3 slots exactly).
    client.set_ciphertext_limit(&admin, &48);

    // 3 slots * 16 = 48 bytes = exactly at the limit.
    let mut values_at = Vec::new(&env);
    values_at.push_back(10);
    values_at.push_back(20);
    values_at.push_back(30);
    let ct_id_at = BytesN::from_array(&env, &[55u8; 32]);
    client.encrypt_ckks_vector(&submitter, &ct_id_at, &ctx_id, &values_at, &2);
    assert!(client.get_ciphertext(&ct_id_at).is_some(), "ciphertext at exactly the limit should be stored");

    // 4 slots * 16 = 64 bytes = over the limit.
    let mut values_over = Vec::new(&env);
    values_over.push_back(10);
    values_over.push_back(20);
    values_over.push_back(30);
    values_over.push_back(40);
    let ct_id_over = BytesN::from_array(&env, &[54u8; 32]);
    let result = client.try_encrypt_ckks_vector(&submitter, &ct_id_over, &ctx_id, &values_over, &2);
    assert!(result.is_err(), "ciphertext over the limit should be rejected");
}

#[test]
fn ciphertext_limit_admin_configurable() {
    let env = Env::default();
    env.mock_all_auths();
    let (client, _id) = setup(&env);

    let admin = Address::generate(&env);
    let submitter = Address::generate(&env);
    client.initialize(&admin);

    let ctx_id = BytesN::from_array(&env, &[61u8; 32]);
    client.register_context(
        &admin,
        &ctx_id,
        &HEScheme::CKKS,
        &String::from_str(&env, "ipfs://he/ckks/params"),
        &BytesN::from_array(&env, &[62u8; 32]),
    );

    // Set limit to 48 bytes (3 slots exactly).
    client.set_ciphertext_limit(&admin, &48);
    assert_eq!(client.get_ciphertext_limit(), 48);

    // 3 slots * 16 = 48 bytes = exactly at the limit.
    let mut values_at_limit = Vec::new(&env);
    values_at_limit.push_back(1);
    values_at_limit.push_back(2);
    values_at_limit.push_back(3);
    let ct_id = BytesN::from_array(&env, &[63u8; 32]);
    client.encrypt_ckks_vector(&submitter, &ct_id, &ctx_id, &values_at_limit, &2);
    assert!(client.get_ciphertext(&ct_id).is_some());

    // 4 slots * 16 = 64 bytes = over the limit.
    let mut values_over = Vec::new(&env);
    values_over.push_back(1);
    values_over.push_back(2);
    values_over.push_back(3);
    values_over.push_back(4);
    let ct_id_over = BytesN::from_array(&env, &[64u8; 32]);
    let result = client.try_encrypt_ckks_vector(&submitter, &ct_id_over, &ctx_id, &values_over, &2);
    assert!(result.is_err(), "over the reduced limit should be rejected");

    // Limit below minimum (48 bytes) should fail.
    let result = client.try_set_ciphertext_limit(&admin, &32);
    assert!(result.is_err(), "limit below minimum should be rejected");

    // Limit above maximum (128 KB) should fail.
    let result = client.try_set_ciphertext_limit(&admin, &200_000);
    assert!(result.is_err(), "limit above 128 KB should be rejected");
}
