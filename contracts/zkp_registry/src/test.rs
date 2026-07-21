// Several helpers (MIN_PROOF_LEN, make_proof_data attachments, submitter
// locals, make_range_proof_fixture, valid_zkp, etc.) were left as
// scaffolding after the TD-001 / TD-003 ZKP-verifier upgrades. They are
// scheduled to be wired up in the next ZKP iteration; for now they are
// dead but would create noise as `cargo clippy -- -D warnings` lints.
#![allow(dead_code, unused_imports, unused_variables)]

extern crate std;

use super::verifier::proof_commitment;
use super::*;
use soroban_sdk::testutils::{Address as _, Ledger};
use soroban_sdk::{vec, Address, Bytes, BytesN, Env, String};

fn setup(env: &Env) -> (ZKPRegistryClient<'_>, Address) {
    let contract_id = env.register_contract(None, ZKPRegistry);
    let client = ZKPRegistryClient::new(env, &contract_id);
    (client, contract_id)
}

/// Build a valid 32-byte proof_data commitment for the given vk_hash and inputs.
fn make_proof_data(env: &Env, vk_hash: &BytesN<32>, inputs: &soroban_sdk::Vec<Bytes>) -> Bytes {
    let commitment = proof_commitment(env, vk_hash, inputs);
    Bytes::from_slice(env, &commitment.to_array())
}

fn make_proof(
    env: &Env,
    label: &'static [u8],
    proof_type: ZKPType,
    hash: ZKPHashFunction,
) -> ZKProof {
    let vk_hash = BytesN::from_array(env, &[1u8; 32]);
    let public_inputs = vec![env, Bytes::from_slice(env, label)];
    let proof_data = make_proof_data(env, &vk_hash, &public_inputs);
    ZKProof {
        proof_type,
        hash_function: hash,
        circuit_id: String::from_str(env, "circuit"),
        public_inputs,
        proof_data,
        vk_hash,
        verification_gas: 50_000,
        created_at: env.ledger().timestamp(),
    }
}

fn make_expiration_payload(env: &Env, valid_until: u64) -> Bytes {
    let mut out = Bytes::new(env);
    out.append(&Bytes::from_slice(env, &valid_until.to_be_bytes()));
    let mut commitment_payload = Bytes::new(env);
    commitment_payload.append(&Bytes::from_slice(env, b"zkp_registry:cred_exp"));
    commitment_payload.append(&Bytes::from_slice(env, &valid_until.to_be_bytes()));
    let commitment: BytesN<32> = env.crypto().sha256(&commitment_payload).into();
    out.append(&Bytes::from_slice(env, &commitment.to_array()));
    out
}

fn init_contract(env: &Env) -> (ZKPRegistryClient<'_>, Address) {
    let (client, contract_id) = setup(env);
    let admin = Address::generate(env);
    client.initialize(&admin);
    (client, contract_id)
}

// ==================== Existing functionality tests ====================

#[test]
fn test_initialize_and_register_circuit() {
    let env = Env::default();
    env.mock_all_auths();
    env.ledger().set_timestamp(1_000_000);

    let (client, _id) = setup(&env);
    let admin = Address::generate(&env);
    client.initialize(&admin);

    let circuit_id = String::from_str(&env, "circuit-a");
    let vk_hash = BytesN::from_array(&env, &[2u8; 32]);
    let pk_hash = BytesN::from_array(&env, &[3u8; 32]);
    client.register_circuit(
        &admin,
        &circuit_id,
        &ZKPType::SNARK,
        &2u32,
        &3u32,
        &100u32,
        &128u32,
        &vk_hash,
        &pk_hash,
        &true,
    );

    let params = client.get_circuit_params(&circuit_id);
    assert_eq!(params.circuit_id, circuit_id);
    assert_eq!(params.circuit_type, ZKPType::SNARK);
    assert_eq!(params.num_public_inputs, 2);
}

#[test]
fn test_submit_zkp_smoke() {
    let env = Env::default();
    env.mock_all_auths();
    env.ledger().set_timestamp(1_000_000);

    let (client, _) = setup(&env);
    let admin = Address::generate(&env);
    client.initialize(&admin);

    let circuit_id = String::from_str(&env, "circuit-b");
    let vk_hash = BytesN::from_array(&env, &[4u8; 32]);
    let pk_hash = BytesN::from_array(&env, &[5u8; 32]);
    client.register_circuit(
        &admin,
        &circuit_id,
        &ZKPType::SNARK,
        &1u32,
        &1u32,
        &50u32,
        &128u32,
        &vk_hash,
        &pk_hash,
        &false,
    );

    let submitter = Address::generate(&env);
    let proof_id = BytesN::from_array(&env, &[6u8; 32]);
    let inputs = vec![&env, Bytes::from_slice(&env, b"input")];
    let proof_data = make_proof_data(&env, &vk_hash, &inputs);

    client.submit_zkp(
        &submitter,
        &proof_id,
        &ZKPType::SNARK,
        &ZKPHashFunction::Poseidon,
        &circuit_id,
        &inputs,
        &proof_data,
        &vk_hash,
        &50_000u64,
    );

    let result = client.get_verification_result(&proof_id);
    assert!(result.is_valid);
    assert_eq!(result.verifier, submitter);
}

#[test]
fn test_create_credential_proof_valid_future_window() {
    let env = Env::default();
    env.mock_all_auths();
    env.ledger().set_timestamp(1_000_000);
    let (client, _) = init_contract(&env);

    let holder = Address::generate(&env);
    let issuer = Address::generate(&env);
    let credential_type = String::from_str(&env, "medical_license");
    let validity_proof = make_proof(&env, b"validity", ZKPType::SNARK, ZKPHashFunction::SHA256);
    let attribute_proof = make_proof(
        &env,
        b"attribute",
        ZKPType::Bulletproof,
        ZKPHashFunction::Poseidon,
    );
    let encrypted_expiration = make_expiration_payload(&env, 1_000_100);

    client.create_credential_proof(
        &holder,
        &credential_type,
        &issuer,
        &validity_proof,
        &attribute_proof,
        &encrypted_expiration,
    );

    let proof = client.get_credential_proof(&holder, &credential_type);
    assert_eq!(proof.issuer, issuer);
    assert!(proof.is_verified);
}

#[test]
fn test_create_credential_proof_about_to_expire() {
    let env = Env::default();
    env.mock_all_auths();
    env.ledger().set_timestamp(1_000_000);
    let (client, _) = init_contract(&env);

    let holder = Address::generate(&env);
    let issuer = Address::generate(&env);
    let credential_type = String::from_str(&env, "researcher");
    let validity_proof = make_proof(&env, b"validity", ZKPType::SNARK, ZKPHashFunction::SHA256);
    let attribute_proof = make_proof(
        &env,
        b"attribute",
        ZKPType::Bulletproof,
        ZKPHashFunction::Poseidon,
    );
    let encrypted_expiration = make_expiration_payload(&env, 1_000_001);

    client.create_credential_proof(
        &holder,
        &credential_type,
        &issuer,
        &validity_proof,
        &attribute_proof,
        &encrypted_expiration,
    );

    assert!(
        client
            .get_credential_proof(&holder, &credential_type)
            .is_verified
    );
}

#[test]
fn test_create_credential_proof_exact_boundary_is_valid() {
    let env = Env::default();
    env.mock_all_auths();
    env.ledger().set_timestamp(1_000_000);
    let (client, _) = init_contract(&env);

    let holder = Address::generate(&env);
    let issuer = Address::generate(&env);
    let credential_type = String::from_str(&env, "nurse");
    let validity_proof = make_proof(&env, b"validity", ZKPType::SNARK, ZKPHashFunction::SHA256);
    let attribute_proof = make_proof(
        &env,
        b"attribute",
        ZKPType::Bulletproof,
        ZKPHashFunction::Poseidon,
    );
    let encrypted_expiration = make_expiration_payload(&env, 1_000_000);

    client.create_credential_proof(
        &holder,
        &credential_type,
        &issuer,
        &validity_proof,
        &attribute_proof,
        &encrypted_expiration,
    );

    assert!(
        client
            .get_credential_proof(&holder, &credential_type)
            .is_verified
    );
}

#[test]
fn test_create_credential_proof_future_far_is_valid() {
    let env = Env::default();
    env.mock_all_auths();
    env.ledger().set_timestamp(1_000_000);
    let (client, _) = init_contract(&env);

    let holder = Address::generate(&env);
    let issuer = Address::generate(&env);
    let credential_type = String::from_str(&env, "surgeon");
    let validity_proof = make_proof(&env, b"validity", ZKPType::SNARK, ZKPHashFunction::SHA256);
    let attribute_proof = make_proof(
        &env,
        b"attribute",
        ZKPType::Bulletproof,
        ZKPHashFunction::Poseidon,
    );
    let encrypted_expiration = make_expiration_payload(&env, 9_999_999);

    client.create_credential_proof(
        &holder,
        &credential_type,
        &issuer,
        &validity_proof,
        &attribute_proof,
        &encrypted_expiration,
    );

    assert!(
        client
            .get_credential_proof(&holder, &credential_type)
            .is_verified
    );
}

#[test]
fn test_create_credential_proof_expired_is_rejected() {
    let env = Env::default();
    env.mock_all_auths();
    env.ledger().set_timestamp(1_000_000);
    let (client, _) = init_contract(&env);

    let holder = Address::generate(&env);
    let issuer = Address::generate(&env);
    let credential_type = String::from_str(&env, "pharmacist");
    let validity_proof = make_proof(&env, b"validity", ZKPType::SNARK, ZKPHashFunction::SHA256);
    let attribute_proof = make_proof(
        &env,
        b"attribute",
        ZKPType::Bulletproof,
        ZKPHashFunction::Poseidon,
    );
    let encrypted_expiration = make_expiration_payload(&env, 999_999);

    let result = client.try_create_credential_proof(
        &holder,
        &credential_type,
        &issuer,
        &validity_proof,
        &attribute_proof,
        &encrypted_expiration,
    );

    assert_eq!(result, Err(Ok(Error::CredentialExpired)));
}

#[test]
fn test_create_credential_proof_tampered_commitment_is_rejected() {
    let env = Env::default();
    env.mock_all_auths();
    env.ledger().set_timestamp(1_000_000);
    let (client, _) = init_contract(&env);

    let holder = Address::generate(&env);
    let issuer = Address::generate(&env);
    let credential_type = String::from_str(&env, "pathologist");
    let validity_proof = make_proof(&env, b"validity", ZKPType::SNARK, ZKPHashFunction::SHA256);
    let attribute_proof = make_proof(
        &env,
        b"attribute",
        ZKPType::Bulletproof,
        ZKPHashFunction::Poseidon,
    );
    let mut encrypted_expiration = make_expiration_payload(&env, 1_000_050);
    let mut tampered = [0u8; 40];
    encrypted_expiration.copy_into_slice(&mut tampered);
    tampered[39] ^= 0x01;
    encrypted_expiration = Bytes::from_slice(&env, &tampered);

    let result = client.try_create_credential_proof(
        &holder,
        &credential_type,
        &issuer,
        &validity_proof,
        &attribute_proof,
        &encrypted_expiration,
    );

    assert_eq!(result, Err(Ok(Error::CommitmentMismatch)));
}

#[test]
fn test_create_credential_proof_short_payload_is_rejected() {
    let env = Env::default();
    env.mock_all_auths();
    env.ledger().set_timestamp(1_000_000);
    let (client, _) = init_contract(&env);

    let holder = Address::generate(&env);
    let issuer = Address::generate(&env);
    let credential_type = String::from_str(&env, "therapist");
    let validity_proof = make_proof(&env, b"validity", ZKPType::SNARK, ZKPHashFunction::SHA256);
    let attribute_proof = make_proof(
        &env,
        b"attribute",
        ZKPType::Bulletproof,
        ZKPHashFunction::Poseidon,
    );
    let encrypted_expiration = Bytes::from_slice(&env, b"short");

    let result = client.try_create_credential_proof(
        &holder,
        &credential_type,
        &issuer,
        &validity_proof,
        &attribute_proof,
        &encrypted_expiration,
    );

    assert_eq!(result, Err(Ok(Error::InvalidInput)));
}

// ==================== verify_range_proof_internal tests ====================

use super::range_proof::{make_range_proof_data, range_commitment, MIN_PROOF_LEN};

fn make_range_proof_fixture(
    env: &Env,
    enc_value: &[u8],
    min: u64,
    max: u64,
    vk: [u8; 32],
) -> (Address, BytesN<32>, Bytes, BytesN<32>) {
    let prover = Address::generate(env);
    let proof_id = BytesN::from_array(env, &[0xddu8; 32]);
    let encrypted_value = Bytes::from_slice(env, enc_value);
    let vk_hash = BytesN::from_array(env, &vk);
    let rp = RangeProof {
        prover: prover.clone(),
        encrypted_value: encrypted_value.clone(),
        min_value: min,
        max_value: max,
        proof_data: Bytes::new(env),
        vk_hash: vk_hash.clone(),
        verification_gas: 1_000,
        created_at: 0,
    };
    let proof_data = make_range_proof_data(env, &rp);
    (
        prover,
        vk_hash,
        encrypted_value,
        BytesN::from_array(env, &proof_id.to_array()),
    )
}

fn init_rng(env: &Env) -> ZKPRegistryClient<'_> {
    let id = env.register_contract(None, ZKPRegistry);
    let client = ZKPRegistryClient::new(env, &id);
    let admin = Address::generate(env);
    client.initialize(&admin);
    client
}

#[test]
fn test_range_proof_valid_passes() {
    let env = Env::default();
    env.mock_all_auths();
    env.ledger().set_timestamp(1_000_000);
    let client = init_rng(&env);

    let enc_value = Bytes::from_slice(&env, b"secret_amount");
    let vk = BytesN::from_array(&env, &[0x11u8; 32]);
    let min = 1u64;
    let max = 100u64;
    let rp = RangeProof {
        prover: Address::generate(&env),
        encrypted_value: enc_value.clone(),
        min_value: min,
        max_value: max,
        proof_data: Bytes::new(&env),
        vk_hash: vk.clone(),
        verification_gas: 1_000,
        created_at: 0,
    };
    let proof_data = make_range_proof_data(&env, &rp);
    let prover = Address::generate(&env);
    let proof_id = BytesN::from_array(&env, &[0x01u8; 32]);

    client.create_range_proof(
        &prover,
        &proof_id,
        &enc_value,
        &min,
        &max,
        &proof_data,
        &vk,
        &1_000,
    );
    let stored = client.get_range_proof(&proof_id);
    assert_eq!(stored.min_value, min);
    assert_eq!(stored.max_value, max);
}

#[test]
fn test_range_proof_empty_proof_data_rejected() {
    let env = Env::default();
    env.mock_all_auths();
    let client = init_rng(&env);
    let prover = Address::generate(&env);
    let enc = Bytes::from_slice(&env, b"v");
    let vk = BytesN::from_array(&env, &[1u8; 32]);
    let r = client.try_create_range_proof(
        &prover,
        &BytesN::from_array(&env, &[0x02u8; 32]),
        &enc,
        &1,
        &100,
        &Bytes::new(&env),
        &vk,
        &1_000,
    );
    assert!(r.is_err());
}

#[test]
fn test_range_proof_short_proof_data_rejected() {
    let env = Env::default();
    env.mock_all_auths();
    let client = init_rng(&env);
    let prover = Address::generate(&env);
    let enc = Bytes::from_slice(&env, b"v");
    let vk = BytesN::from_array(&env, &[1u8; 32]);
    let short_data = Bytes::from_slice(&env, b"tooshort");
    let r = client.try_create_range_proof(
        &prover,
        &BytesN::from_array(&env, &[0x03u8; 32]),
        &enc,
        &1,
        &100,
        &short_data,
        &vk,
        &1_000,
    );
    assert!(r.is_err());
}

#[test]
fn test_range_proof_wrong_version_returns_version_mismatch() {
    let env = Env::default();
    env.mock_all_auths();
    let client = init_rng(&env);
    let prover = Address::generate(&env);
    let enc = Bytes::from_slice(&env, b"v");
    let vk = BytesN::from_array(&env, &[1u8; 32]);
    let rp = RangeProof {
        prover: prover.clone(),
        encrypted_value: enc.clone(),
        min_value: 1,
        max_value: 100,
        proof_data: Bytes::new(&env),
        vk_hash: vk.clone(),
        verification_gas: 1_000,
        created_at: 0,
    };
    let mut pd_bytes = [0u8; 36];
    make_range_proof_data(&env, &rp).copy_into_slice(&mut pd_bytes);
    pd_bytes[0] = 0xff;
    let bad_pd = Bytes::from_slice(&env, &pd_bytes);
    let r = client.try_create_range_proof(
        &prover,
        &BytesN::from_array(&env, &[0x04u8; 32]),
        &enc,
        &1,
        &100,
        &bad_pd,
        &vk,
        &1_000,
    );
    assert_eq!(r, Err(Ok(Error::VersionMismatch)));
}

#[test]
fn test_range_proof_tampered_commitment_returns_invalid_range_proof() {
    let env = Env::default();
    env.mock_all_auths();
    let client = init_rng(&env);
    let prover = Address::generate(&env);
    let enc = Bytes::from_slice(&env, b"secret");
    let vk = BytesN::from_array(&env, &[1u8; 32]);
    let rp = RangeProof {
        prover: prover.clone(),
        encrypted_value: enc.clone(),
        min_value: 10,
        max_value: 200,
        proof_data: Bytes::new(&env),
        vk_hash: vk.clone(),
        verification_gas: 1_000,
        created_at: 0,
    };
    let mut pd_bytes = [0u8; 36];
    make_range_proof_data(&env, &rp).copy_into_slice(&mut pd_bytes);
    pd_bytes[35] ^= 0x01;
    let bad_pd = Bytes::from_slice(&env, &pd_bytes);
    let r = client.try_create_range_proof(
        &prover,
        &BytesN::from_array(&env, &[0x05u8; 32]),
        &enc,
        &10,
        &200,
        &bad_pd,
        &vk,
        &1_000,
    );
    assert_eq!(r, Err(Ok(Error::InvalidRangeProof)));
}

#[test]
fn test_range_proof_wrong_vk_hash_rejected() {
    let env = Env::default();
    env.mock_all_auths();
    let client = init_rng(&env);
    let prover = Address::generate(&env);
    let enc = Bytes::from_slice(&env, b"secret");
    let correct_vk = BytesN::from_array(&env, &[0xaau8; 32]);
    let wrong_vk = BytesN::from_array(&env, &[0xbbu8; 32]);
    let rp = RangeProof {
        prover: prover.clone(),
        encrypted_value: enc.clone(),
        min_value: 1,
        max_value: 50,
        proof_data: Bytes::new(&env),
        vk_hash: correct_vk.clone(),
        verification_gas: 1_000,
        created_at: 0,
    };
    let proof_data = make_range_proof_data(&env, &rp);
    // Submit with wrong_vk → commitment mismatch
    let r = client.try_create_range_proof(
        &prover,
        &BytesN::from_array(&env, &[0x06u8; 32]),
        &enc,
        &1,
        &50,
        &proof_data,
        &wrong_vk,
        &1_000,
    );
    assert_eq!(r, Err(Ok(Error::InvalidRangeProof)));
}

#[test]
fn test_range_proof_wrong_min_rejected() {
    let env = Env::default();
    env.mock_all_auths();
    let client = init_rng(&env);
    let prover = Address::generate(&env);
    let enc = Bytes::from_slice(&env, b"v");
    let vk = BytesN::from_array(&env, &[1u8; 32]);
    let rp = RangeProof {
        prover: prover.clone(),
        encrypted_value: enc.clone(),
        min_value: 1,
        max_value: 100,
        proof_data: Bytes::new(&env),
        vk_hash: vk.clone(),
        verification_gas: 1_000,
        created_at: 0,
    };
    let proof_data = make_range_proof_data(&env, &rp);
    // Submit with different min_value
    let r = client.try_create_range_proof(
        &prover,
        &BytesN::from_array(&env, &[0x07u8; 32]),
        &enc,
        &5,
        &100,
        &proof_data,
        &vk,
        &1_000,
    );
    assert_eq!(r, Err(Ok(Error::InvalidRangeProof)));
}

#[test]
fn test_range_proof_wrong_max_rejected() {
    let env = Env::default();
    env.mock_all_auths();
    let client = init_rng(&env);
    let prover = Address::generate(&env);
    let enc = Bytes::from_slice(&env, b"v");
    let vk = BytesN::from_array(&env, &[1u8; 32]);
    let rp = RangeProof {
        prover: prover.clone(),
        encrypted_value: enc.clone(),
        min_value: 1,
        max_value: 100,
        proof_data: Bytes::new(&env),
        vk_hash: vk.clone(),
        verification_gas: 1_000,
        created_at: 0,
    };
    let proof_data = make_range_proof_data(&env, &rp);
    // Submit with different max_value → commitment mismatch
    let r = client.try_create_range_proof(
        &prover,
        &BytesN::from_array(&env, &[0x08u8; 32]),
        &enc,
        &1,
        &1_000_000,
        &proof_data,
        &vk,
        &1_000,
    );
    assert_eq!(r, Err(Ok(Error::InvalidRangeProof)));
}

#[test]
fn test_range_proof_wrong_encrypted_value_rejected() {
    let env = Env::default();
    env.mock_all_auths();
    let client = init_rng(&env);
    let prover = Address::generate(&env);
    let enc = Bytes::from_slice(&env, b"original_value");
    let forged_enc = Bytes::from_slice(&env, b"forged_value");
    let vk = BytesN::from_array(&env, &[1u8; 32]);
    let rp = RangeProof {
        prover: prover.clone(),
        encrypted_value: enc.clone(),
        min_value: 1,
        max_value: 100,
        proof_data: Bytes::new(&env),
        vk_hash: vk.clone(),
        verification_gas: 1_000,
        created_at: 0,
    };
    let proof_data = make_range_proof_data(&env, &rp);
    // Submit with different encrypted_value
    let r = client.try_create_range_proof(
        &prover,
        &BytesN::from_array(&env, &[0x09u8; 32]),
        &forged_enc,
        &1,
        &100,
        &proof_data,
        &vk,
        &1_000,
    );
    assert_eq!(r, Err(Ok(Error::InvalidRangeProof)));
}

#[test]
fn test_range_proof_min_equals_max_rejected() {
    let env = Env::default();
    env.mock_all_auths();
    let client = init_rng(&env);
    let prover = Address::generate(&env);
    let enc = Bytes::from_slice(&env, b"v");
    let vk = BytesN::from_array(&env, &[1u8; 32]);
    let pd = Bytes::from_slice(&env, &[0u8; 36]);
    let r = client.try_create_range_proof(
        &prover,
        &BytesN::from_array(&env, &[0x0au8; 32]),
        &enc,
        &50,
        &50,
        &pd,
        &vk,
        &1_000,
    );
    assert_eq!(r, Err(Ok(Error::InvalidRange)));
}

#[test]
fn test_range_proof_prop_any_byte_flip_in_commitment_fails() {
    let env = Env::default();
    env.mock_all_auths();
    let client = init_rng(&env);
    let enc = Bytes::from_slice(&env, b"prop_val");
    let vk = BytesN::from_array(&env, &[0x55u8; 32]);
    let rp = RangeProof {
        prover: Address::generate(&env),
        encrypted_value: enc.clone(),
        min_value: 0,
        max_value: u64::MAX,
        proof_data: Bytes::new(&env),
        vk_hash: vk.clone(),
        verification_gas: 1_000,
        created_at: 0,
    };
    let mut pd_bytes = [0u8; 36];
    make_range_proof_data(&env, &rp).copy_into_slice(&mut pd_bytes);

    for i in 4..36usize {
        let mut tampered = pd_bytes;
        tampered[i] ^= 0x80;
        let prover = Address::generate(&env);
        let pid = BytesN::from_array(&env, &[i as u8; 32]);
        let r = client.try_create_range_proof(
            &prover,
            &pid,
            &enc,
            &0,
            &u64::MAX,
            &Bytes::from_slice(&env, &tampered),
            &vk,
            &1_000,
        );
        assert_eq!(
            r,
            Err(Ok(Error::InvalidRangeProof)),
            "flip at byte {i} should fail"
        );
    }
}

#[test]
fn test_range_proof_prop_different_bounds_different_commitments() {
    let env = Env::default();
    let enc = Bytes::from_slice(&env, b"enc");
    let vk = BytesN::from_array(&env, &[1u8; 32]);
    let mk = |min: u64, max: u64| {
        let rp = RangeProof {
            prover: Address::generate(&env),
            encrypted_value: enc.clone(),
            min_value: min,
            max_value: max,
            proof_data: Bytes::new(&env),
            vk_hash: vk.clone(),
            verification_gas: 1_000,
            created_at: 0,
        };
        let comm = range_commitment(&env, &rp);
        comm.to_array()
    };
    assert_ne!(mk(0, 100), mk(0, 1_000_000));
    assert_ne!(mk(0, 100), mk(50, 100));
    assert_ne!(mk(1, 99), mk(0, 100));
}

#[test]
fn test_range_proof_prop_commitment_is_deterministic() {
    let env = Env::default();
    let enc = Bytes::from_slice(&env, b"det");
    let vk = BytesN::from_array(&env, &[2u8; 32]);
    let rp = RangeProof {
        prover: Address::generate(&env),
        encrypted_value: enc.clone(),
        min_value: 10,
        max_value: 20,
        proof_data: Bytes::new(&env),
        vk_hash: vk.clone(),
        verification_gas: 1_000,
        created_at: 0,
    };
    let c1 = range_commitment(&env, &rp).to_array();
    let c2 = range_commitment(&env, &rp).to_array();
    assert_eq!(c1, c2);
}

// ==================== verify_zkp_internal unit tests ====================

/// Helper: build a ZKProof with correct commitment (passes verification).
fn valid_zkp(env: &Env, vk_hash: [u8; 32], input: &[u8]) -> ZKProof {
    let vk = BytesN::from_array(env, &vk_hash);
    let inputs = vec![env, Bytes::from_slice(env, input)];
    let proof_data = make_proof_data(env, &vk, &inputs);
    ZKProof {
        proof_type: ZKPType::SNARK,
        hash_function: ZKPHashFunction::Poseidon,
        circuit_id: String::from_str(env, "c1"),
        public_inputs: inputs,
        proof_data,
        vk_hash: vk,
        verification_gas: 1_000,
        created_at: 0,
    }
}

fn env_with_contract() -> (Env, Address) {
    let env = Env::default();
    env.mock_all_auths();
    env.ledger().set_timestamp(1_000_000);
    let id = env.register_contract(None, ZKPRegistry);
    let client = ZKPRegistryClient::new(&env, &id);
    let admin = Address::generate(&env);
    client.initialize(&admin);
    (env, id)
}

#[test]
fn test_zkp_valid_commitment_passes() {
    let (env, id) = env_with_contract();
    let client = ZKPRegistryClient::new(&env, &id);
    let admin = Address::generate(&env);
    let circuit_id = String::from_str(&env, "c1");
    let vk = BytesN::from_array(&env, &[7u8; 32]);
    let pk = BytesN::from_array(&env, &[8u8; 32]);
    client.register_circuit(
        &admin,
        &circuit_id,
        &ZKPType::SNARK,
        &1,
        &1,
        &10,
        &128,
        &vk,
        &pk,
        &false,
    );

    let inputs = vec![&env, Bytes::from_slice(&env, b"pub_input")];
    let proof_data = make_proof_data(&env, &vk, &inputs);
    let proof_id = BytesN::from_array(&env, &[9u8; 32]);
    let submitter = Address::generate(&env);

    client.submit_zkp(
        &submitter,
        &proof_id,
        &ZKPType::SNARK,
        &ZKPHashFunction::Poseidon,
        &circuit_id,
        &inputs,
        &proof_data,
        &vk,
        &10_000,
    );
    assert!(client.get_verification_result(&proof_id).is_valid);
}

#[test]
fn test_zkp_empty_proof_data_rejected() {
    let env = Env::default();
    env.mock_all_auths();
    let (client, _) = init_contract(&env);
    let holder = Address::generate(&env);
    let issuer = Address::generate(&env);
    let ctype = String::from_str(&env, "t1");
    let bad_proof = ZKProof {
        proof_type: ZKPType::SNARK,
        hash_function: ZKPHashFunction::Poseidon,
        circuit_id: String::from_str(&env, "c1"),
        public_inputs: vec![&env, Bytes::from_slice(&env, b"x")],
        proof_data: Bytes::new(&env),
        vk_hash: BytesN::from_array(&env, &[1u8; 32]),
        verification_gas: 1_000,
        created_at: 0,
    };
    let good_proof = make_proof(&env, b"attr", ZKPType::SNARK, ZKPHashFunction::Poseidon);
    let exp = make_expiration_payload(&env, 2_000_000);
    let r =
        client.try_create_credential_proof(&holder, &ctype, &issuer, &bad_proof, &good_proof, &exp);
    assert_eq!(r, Err(Ok(Error::InvalidProof)));
}

#[test]
fn test_zkp_short_proof_data_rejected() {
    let env = Env::default();
    env.mock_all_auths();
    let (client, _) = init_contract(&env);
    let holder = Address::generate(&env);
    let issuer = Address::generate(&env);
    let ctype = String::from_str(&env, "t2");
    let bad_proof = ZKProof {
        proof_type: ZKPType::SNARK,
        hash_function: ZKPHashFunction::Poseidon,
        circuit_id: String::from_str(&env, "c1"),
        public_inputs: vec![&env, Bytes::from_slice(&env, b"x")],
        proof_data: Bytes::from_slice(&env, b"tooshort"),
        vk_hash: BytesN::from_array(&env, &[1u8; 32]),
        verification_gas: 1_000,
        created_at: 0,
    };
    let good_proof = make_proof(&env, b"attr", ZKPType::SNARK, ZKPHashFunction::Poseidon);
    let exp = make_expiration_payload(&env, 2_000_000);
    let r =
        client.try_create_credential_proof(&holder, &ctype, &issuer, &bad_proof, &good_proof, &exp);
    assert_eq!(r, Err(Ok(Error::InvalidProof)));
}

#[test]
fn test_zkp_wrong_vk_hash_rejected() {
    let env = Env::default();
    env.mock_all_auths();
    let (client, _) = init_contract(&env);
    let holder = Address::generate(&env);
    let issuer = Address::generate(&env);
    let ctype = String::from_str(&env, "t3");

    let correct_vk = BytesN::from_array(&env, &[1u8; 32]);
    let wrong_vk = BytesN::from_array(&env, &[2u8; 32]);
    let inputs = vec![&env, Bytes::from_slice(&env, b"validity")];
    // proof_data is computed with correct_vk but submitted with wrong_vk
    let proof_data = make_proof_data(&env, &correct_vk, &inputs);
    let bad_proof = ZKProof {
        proof_type: ZKPType::SNARK,
        hash_function: ZKPHashFunction::Poseidon,
        circuit_id: String::from_str(&env, "c1"),
        public_inputs: inputs,
        proof_data,
        vk_hash: wrong_vk,
        verification_gas: 1_000,
        created_at: 0,
    };
    let good_proof = make_proof(&env, b"attr", ZKPType::SNARK, ZKPHashFunction::Poseidon);
    let exp = make_expiration_payload(&env, 2_000_000);
    let r =
        client.try_create_credential_proof(&holder, &ctype, &issuer, &bad_proof, &good_proof, &exp);
    assert_eq!(r, Err(Ok(Error::InvalidProof)));
}

#[test]
fn test_zkp_wrong_inputs_rejected() {
    let env = Env::default();
    env.mock_all_auths();
    let (client, _) = init_contract(&env);
    let holder = Address::generate(&env);
    let issuer = Address::generate(&env);
    let ctype = String::from_str(&env, "t4");

    let vk = BytesN::from_array(&env, &[1u8; 32]);
    let original_inputs = vec![&env, Bytes::from_slice(&env, b"real_input")];
    let tampered_inputs = vec![&env, Bytes::from_slice(&env, b"forged_input")];
    // proof_data matches original_inputs, but proof carries tampered_inputs
    let proof_data = make_proof_data(&env, &vk, &original_inputs);
    let bad_proof = ZKProof {
        proof_type: ZKPType::SNARK,
        hash_function: ZKPHashFunction::Poseidon,
        circuit_id: String::from_str(&env, "c1"),
        public_inputs: tampered_inputs,
        proof_data,
        vk_hash: vk,
        verification_gas: 1_000,
        created_at: 0,
    };
    let good_proof = make_proof(&env, b"attr", ZKPType::SNARK, ZKPHashFunction::Poseidon);
    let exp = make_expiration_payload(&env, 2_000_000);
    let r =
        client.try_create_credential_proof(&holder, &ctype, &issuer, &bad_proof, &good_proof, &exp);
    assert_eq!(r, Err(Ok(Error::InvalidProof)));
}

#[test]
fn test_zkp_empty_public_inputs_rejected() {
    let env = Env::default();
    env.mock_all_auths();
    let (client, _) = init_contract(&env);
    let holder = Address::generate(&env);
    let issuer = Address::generate(&env);
    let ctype = String::from_str(&env, "t5");
    let vk = BytesN::from_array(&env, &[1u8; 32]);
    let bad_proof = ZKProof {
        proof_type: ZKPType::SNARK,
        hash_function: ZKPHashFunction::Poseidon,
        circuit_id: String::from_str(&env, "c1"),
        public_inputs: soroban_sdk::Vec::new(&env),
        proof_data: Bytes::from_slice(&env, &[0u8; 32]),
        vk_hash: vk,
        verification_gas: 1_000,
        created_at: 0,
    };
    let good_proof = make_proof(&env, b"attr", ZKPType::SNARK, ZKPHashFunction::Poseidon);
    let exp = make_expiration_payload(&env, 2_000_000);
    let r =
        client.try_create_credential_proof(&holder, &ctype, &issuer, &bad_proof, &good_proof, &exp);
    assert_eq!(r, Err(Ok(Error::InvalidProof)));
}

#[test]
fn test_zkp_empty_input_element_rejected() {
    let env = Env::default();
    env.mock_all_auths();
    let (client, _) = init_contract(&env);
    let holder = Address::generate(&env);
    let issuer = Address::generate(&env);
    let ctype = String::from_str(&env, "t6");
    let vk = BytesN::from_array(&env, &[1u8; 32]);
    let bad_proof = ZKProof {
        proof_type: ZKPType::SNARK,
        hash_function: ZKPHashFunction::Poseidon,
        circuit_id: String::from_str(&env, "c1"),
        public_inputs: vec![&env, Bytes::new(&env)], // empty input element
        proof_data: Bytes::from_slice(&env, &[0u8; 32]),
        vk_hash: vk,
        verification_gas: 1_000,
        created_at: 0,
    };
    let good_proof = make_proof(&env, b"attr", ZKPType::SNARK, ZKPHashFunction::Poseidon);
    let exp = make_expiration_payload(&env, 2_000_000);
    let r =
        client.try_create_credential_proof(&holder, &ctype, &issuer, &bad_proof, &good_proof, &exp);
    assert_eq!(r, Err(Ok(Error::InvalidProof)));
}

#[test]
fn test_zkp_tampered_first_commitment_byte_rejected() {
    let env = Env::default();
    env.mock_all_auths();
    let (client, _) = init_contract(&env);
    let holder = Address::generate(&env);
    let issuer = Address::generate(&env);
    let ctype = String::from_str(&env, "t7");
    let vk = BytesN::from_array(&env, &[1u8; 32]);
    let inputs = vec![&env, Bytes::from_slice(&env, b"validity")];
    let mut proof_bytes = [0u8; 32];
    make_proof_data(&env, &vk, &inputs).copy_into_slice(&mut proof_bytes);
    proof_bytes[0] ^= 0x01; // flip first byte
    let bad_proof = ZKProof {
        proof_type: ZKPType::SNARK,
        hash_function: ZKPHashFunction::Poseidon,
        circuit_id: String::from_str(&env, "c1"),
        public_inputs: inputs,
        proof_data: Bytes::from_slice(&env, &proof_bytes),
        vk_hash: vk,
        verification_gas: 1_000,
        created_at: 0,
    };
    let good_proof = make_proof(&env, b"attr", ZKPType::SNARK, ZKPHashFunction::Poseidon);
    let exp = make_expiration_payload(&env, 2_000_000);
    let r =
        client.try_create_credential_proof(&holder, &ctype, &issuer, &bad_proof, &good_proof, &exp);
    assert_eq!(r, Err(Ok(Error::InvalidProof)));
}

#[test]
fn test_zkp_tampered_last_commitment_byte_rejected() {
    let env = Env::default();
    env.mock_all_auths();
    let (client, _) = init_contract(&env);
    let holder = Address::generate(&env);
    let issuer = Address::generate(&env);
    let ctype = String::from_str(&env, "t8");
    let vk = BytesN::from_array(&env, &[1u8; 32]);
    let inputs = vec![&env, Bytes::from_slice(&env, b"validity")];
    let mut proof_bytes = [0u8; 32];
    make_proof_data(&env, &vk, &inputs).copy_into_slice(&mut proof_bytes);
    proof_bytes[31] ^= 0xff;
    let bad_proof = ZKProof {
        proof_type: ZKPType::SNARK,
        hash_function: ZKPHashFunction::Poseidon,
        circuit_id: String::from_str(&env, "c1"),
        public_inputs: inputs,
        proof_data: Bytes::from_slice(&env, &proof_bytes),
        vk_hash: vk,
        verification_gas: 1_000,
        created_at: 0,
    };
    let good_proof = make_proof(&env, b"attr", ZKPType::SNARK, ZKPHashFunction::Poseidon);
    let exp = make_expiration_payload(&env, 2_000_000);
    let r =
        client.try_create_credential_proof(&holder, &ctype, &issuer, &bad_proof, &good_proof, &exp);
    assert_eq!(r, Err(Ok(Error::InvalidProof)));
}

#[test]
fn test_zkp_all_zero_commitment_rejected() {
    let env = Env::default();
    env.mock_all_auths();
    let (client, _) = init_contract(&env);
    let holder = Address::generate(&env);
    let issuer = Address::generate(&env);
    let ctype = String::from_str(&env, "t9");
    let vk = BytesN::from_array(&env, &[1u8; 32]);
    let inputs = vec![&env, Bytes::from_slice(&env, b"x")];
    let bad_proof = ZKProof {
        proof_type: ZKPType::SNARK,
        hash_function: ZKPHashFunction::Poseidon,
        circuit_id: String::from_str(&env, "c1"),
        public_inputs: inputs,
        proof_data: Bytes::from_slice(&env, &[0u8; 32]),
        vk_hash: vk,
        verification_gas: 1_000,
        created_at: 0,
    };
    let good_proof = make_proof(&env, b"attr", ZKPType::SNARK, ZKPHashFunction::Poseidon);
    let exp = make_expiration_payload(&env, 2_000_000);
    let r =
        client.try_create_credential_proof(&holder, &ctype, &issuer, &bad_proof, &good_proof, &exp);
    assert_eq!(r, Err(Ok(Error::InvalidProof)));
}

#[test]
fn test_zkp_snark_type_passes() {
    let env = Env::default();
    env.mock_all_auths();
    let (client, _) = init_contract(&env);
    let holder = Address::generate(&env);
    let issuer = Address::generate(&env);
    let ctype = String::from_str(&env, "snark_type");
    let p1 = make_proof(&env, b"validity", ZKPType::SNARK, ZKPHashFunction::Poseidon);
    let p2 = make_proof(&env, b"attr", ZKPType::SNARK, ZKPHashFunction::MiMC);
    let exp = make_expiration_payload(&env, 2_000_000);
    client.create_credential_proof(&holder, &ctype, &issuer, &p1, &p2, &exp);
    assert!(client.get_credential_proof(&holder, &ctype).is_verified);
}

#[test]
fn test_zkp_stark_type_passes() {
    let env = Env::default();
    env.mock_all_auths();
    let (client, _) = init_contract(&env);
    let holder = Address::generate(&env);
    let issuer = Address::generate(&env);
    let ctype = String::from_str(&env, "stark_type");
    let p1 = make_proof(&env, b"validity", ZKPType::STARK, ZKPHashFunction::SHA256);
    let p2 = make_proof(&env, b"attr", ZKPType::STARK, ZKPHashFunction::Rescue);
    let exp = make_expiration_payload(&env, 2_000_000);
    client.create_credential_proof(&holder, &ctype, &issuer, &p1, &p2, &exp);
    assert!(client.get_credential_proof(&holder, &ctype).is_verified);
}

#[test]
fn test_zkp_bulletproof_type_passes() {
    let env = Env::default();
    env.mock_all_auths();
    let (client, _) = init_contract(&env);
    let holder = Address::generate(&env);
    let issuer = Address::generate(&env);
    let ctype = String::from_str(&env, "bp_type");
    let p1 = make_proof(
        &env,
        b"validity",
        ZKPType::Bulletproof,
        ZKPHashFunction::Poseidon,
    );
    let p2 = make_proof(
        &env,
        b"attr",
        ZKPType::Bulletproof,
        ZKPHashFunction::Poseidon,
    );
    let exp = make_expiration_payload(&env, 2_000_000);
    client.create_credential_proof(&holder, &ctype, &issuer, &p1, &p2, &exp);
    assert!(client.get_credential_proof(&holder, &ctype).is_verified);
}

#[test]
fn test_zkp_pedersen_type_passes() {
    let env = Env::default();
    env.mock_all_auths();
    let (client, _) = init_contract(&env);
    let holder = Address::generate(&env);
    let issuer = Address::generate(&env);
    let ctype = String::from_str(&env, "ped_type");
    let p1 = make_proof(
        &env,
        b"validity",
        ZKPType::PedersenCommitment,
        ZKPHashFunction::MiMC,
    );
    let p2 = make_proof(
        &env,
        b"attr",
        ZKPType::PedersenCommitment,
        ZKPHashFunction::MiMC,
    );
    let exp = make_expiration_payload(&env, 2_000_000);
    client.create_credential_proof(&holder, &ctype, &issuer, &p1, &p2, &exp);
    assert!(client.get_credential_proof(&holder, &ctype).is_verified);
}

#[test]
fn test_zkp_recursive_type_passes() {
    let env = Env::default();
    env.mock_all_auths();
    let (client, _) = init_contract(&env);
    let holder = Address::generate(&env);
    let issuer = Address::generate(&env);
    let ctype = String::from_str(&env, "rec_type");
    let p1 = make_proof(
        &env,
        b"validity",
        ZKPType::Recursive,
        ZKPHashFunction::Rescue,
    );
    let p2 = make_proof(&env, b"attr", ZKPType::Recursive, ZKPHashFunction::Rescue);
    let exp = make_expiration_payload(&env, 2_000_000);
    client.create_credential_proof(&holder, &ctype, &issuer, &p1, &p2, &exp);
    assert!(client.get_credential_proof(&holder, &ctype).is_verified);
}

#[test]
fn test_zkp_multiple_public_inputs_passes() {
    let env = Env::default();
    env.mock_all_auths();
    let (client, _) = init_contract(&env);
    let holder = Address::generate(&env);
    let issuer = Address::generate(&env);
    let ctype = String::from_str(&env, "multi_in");
    let vk = BytesN::from_array(&env, &[1u8; 32]);
    let inputs = vec![
        &env,
        Bytes::from_slice(&env, b"in0"),
        Bytes::from_slice(&env, b"in1"),
        Bytes::from_slice(&env, b"in2"),
    ];
    let proof_data = make_proof_data(&env, &vk, &inputs);
    let p1 = ZKProof {
        proof_type: ZKPType::SNARK,
        hash_function: ZKPHashFunction::Poseidon,
        circuit_id: String::from_str(&env, "circuit"),
        public_inputs: inputs,
        proof_data,
        vk_hash: vk,
        verification_gas: 1_000,
        created_at: 0,
    };
    let p2 = make_proof(&env, b"attr", ZKPType::SNARK, ZKPHashFunction::Poseidon);
    let exp = make_expiration_payload(&env, 2_000_000);
    client.create_credential_proof(&holder, &ctype, &issuer, &p1, &p2, &exp);
    assert!(client.get_credential_proof(&holder, &ctype).is_verified);
}

#[test]
fn test_zkp_extra_proof_bytes_after_commitment_valid() {
    let env = Env::default();
    env.mock_all_auths();
    let (client, _) = init_contract(&env);
    let holder = Address::generate(&env);
    let issuer = Address::generate(&env);
    let ctype = String::from_str(&env, "extra_bytes");
    let vk = BytesN::from_array(&env, &[1u8; 32]);
    let inputs = vec![&env, Bytes::from_slice(&env, b"validity")];
    let commitment = make_proof_data(&env, &vk, &inputs);
    // Append extra non-commitment bytes after the 32-byte prefix
    let mut proof_data = commitment;
    proof_data.append(&Bytes::from_slice(&env, b"extra_arbitrary_data_for_proof"));
    let p1 = ZKProof {
        proof_type: ZKPType::SNARK,
        hash_function: ZKPHashFunction::Poseidon,
        circuit_id: String::from_str(&env, "circuit"),
        public_inputs: inputs,
        proof_data,
        vk_hash: vk,
        verification_gas: 1_000,
        created_at: 0,
    };
    let p2 = make_proof(&env, b"attr", ZKPType::SNARK, ZKPHashFunction::Poseidon);
    let exp = make_expiration_payload(&env, 2_000_000);
    client.create_credential_proof(&holder, &ctype, &issuer, &p1, &p2, &exp);
    assert!(client.get_credential_proof(&holder, &ctype).is_verified);
}

#[test]
fn test_zkp_different_vk_hashes_produce_different_commitments() {
    let env = Env::default();
    let vk_a = BytesN::from_array(&env, &[0xaau8; 32]);
    let vk_b = BytesN::from_array(&env, &[0xbbu8; 32]);
    let inputs = vec![&env, Bytes::from_slice(&env, b"same_input")];
    let comm_a = proof_commitment(&env, &vk_a, &inputs);
    let comm_b = proof_commitment(&env, &vk_b, &inputs);
    assert_ne!(comm_a.to_array(), comm_b.to_array());
}

#[test]
fn test_zkp_different_inputs_produce_different_commitments() {
    let env = Env::default();
    let vk = BytesN::from_array(&env, &[1u8; 32]);
    let inputs_a = vec![&env, Bytes::from_slice(&env, b"input_a")];
    let inputs_b = vec![&env, Bytes::from_slice(&env, b"input_b")];
    let comm_a = proof_commitment(&env, &vk, &inputs_a);
    let comm_b = proof_commitment(&env, &vk, &inputs_b);
    assert_ne!(comm_a.to_array(), comm_b.to_array());
}

#[test]
fn test_zkp_same_inputs_same_vk_deterministic() {
    let env = Env::default();
    let vk = BytesN::from_array(&env, &[1u8; 32]);
    let inputs = vec![&env, Bytes::from_slice(&env, b"deterministic")];
    let c1 = proof_commitment(&env, &vk, &inputs);
    let c2 = proof_commitment(&env, &vk, &inputs);
    assert_eq!(c1.to_array(), c2.to_array());
}

#[test]
fn test_zkp_property_any_single_input_byte_flip_rejected() {
    // Flip every byte in the 32-byte proof_data commitment and confirm each flip fails.
    let env = Env::default();
    env.mock_all_auths();
    let (client, _) = init_contract(&env);
    let vk = BytesN::from_array(&env, &[1u8; 32]);
    let inputs = vec![&env, Bytes::from_slice(&env, b"prop_test")];
    let mut proof_bytes = [0u8; 32];
    make_proof_data(&env, &vk, &inputs).copy_into_slice(&mut proof_bytes);

    for i in 0..32usize {
        let mut tampered = proof_bytes;
        tampered[i] ^= 0x80;
        let holder = Address::generate(&env);
        let issuer = Address::generate(&env);
        let ctype = String::from_str(&env, "prop");
        let bad_proof = ZKProof {
            proof_type: ZKPType::SNARK,
            hash_function: ZKPHashFunction::Poseidon,
            circuit_id: String::from_str(&env, "c1"),
            public_inputs: inputs.clone(),
            proof_data: Bytes::from_slice(&env, &tampered),
            vk_hash: vk.clone(),
            verification_gas: 1_000,
            created_at: 0,
        };
        let good_proof = make_proof(&env, b"attr", ZKPType::SNARK, ZKPHashFunction::Poseidon);
        let exp = make_expiration_payload(&env, 2_000_000);
        let r = client.try_create_credential_proof(
            &holder,
            &ctype,
            &issuer,
            &bad_proof,
            &good_proof,
            &exp,
        );
        assert_eq!(
            r,
            Err(Ok(Error::InvalidProof)),
            "bit flip at byte {i} should fail"
        );
    }
}
// ==================== verify_recursive_proof_internal tests ====================

use super::recursive_proof::{make_aggregated_vk, recursive_commitment};

/// Registers a base ZKP so create_recursive_proof can look it up.
fn register_base_proof(
    env: &Env,
    client: &ZKPRegistryClient<'_>,
    proof_id: &BytesN<32>,
    vk: &BytesN<32>,
    pk: &BytesN<32>,
) {
    let admin = Address::generate(env);
    let circuit_id = String::from_str(env, "base_circuit");
    client.register_circuit(
        &admin,
        &circuit_id,
        &ZKPType::SNARK,
        &1,
        &1,
        &10,
        &128,
        vk,
        pk,
        &false,
    );
    let submitter = Address::generate(env);
    let inputs = vec![env, Bytes::from_slice(env, b"base_input")];
    let pd = Bytes::from_slice(env, b"0123456789abcdef0123456789abcdef");
    // Use old-style (non-verifier) path: just store the raw proof directly
    env.as_contract(&client.address, || {
        let proof_struct = ZKProof {
            proof_type: ZKPType::SNARK,
            hash_function: ZKPHashFunction::Poseidon,
            circuit_id: circuit_id.clone(),
            public_inputs: inputs.clone(),
            proof_data: pd.clone(),
            vk_hash: vk.clone(),
            verification_gas: 1_000,
            created_at: 0,
        };
        env.storage()
            .persistent()
            .set(&DataKey::ZKProof(proof_id.clone()), &proof_struct);
    });
}

fn make_recursive_proof_struct(
    env: &Env,
    base_id: &BytesN<32>,
    vk: &BytesN<32>,
    depth: u32,
) -> (RecursiveProof, Bytes) {
    let inner = ZKProof {
        proof_type: ZKPType::Recursive,
        hash_function: ZKPHashFunction::Poseidon,
        circuit_id: String::from_str(env, "rec_circuit"),
        public_inputs: vec![env, Bytes::from_slice(env, b"rec_in")],
        proof_data: Bytes::from_slice(env, b"0123456789abcdef0123456789abcdef"),
        vk_hash: vk.clone(),
        verification_gas: 1_000,
        created_at: 0,
    };
    let rp = RecursiveProof {
        base_proof_id: base_id.clone(),
        recursive_proof: inner,
        aggregated_vk: Bytes::new(env),
        composition_depth: depth,
        total_gas: 5_000,
        composed_at: 0,
    };
    let agg_vk = make_aggregated_vk(env, &rp);
    (rp, agg_vk)
}

fn init_rec(env: &Env) -> (ZKPRegistryClient<'_>, BytesN<32>, BytesN<32>) {
    let id = env.register_contract(None, ZKPRegistry);
    let client = ZKPRegistryClient::new(env, &id);
    let admin = Address::generate(env);
    client.initialize(&admin);
    let vk = BytesN::from_array(env, &[0x22u8; 32]);
    let pk = BytesN::from_array(env, &[0x33u8; 32]);
    let base_id = BytesN::from_array(env, &[0x11u8; 32]);
    register_base_proof(env, &client, &base_id, &vk, &pk);
    (client, base_id, vk)
}

#[test]
fn test_recursive_proof_depth_1_passes() {
    let env = Env::default();
    env.mock_all_auths();
    env.ledger().set_timestamp(1_000_000);
    let (client, base_id, vk) = init_rec(&env);

    let (rp, agg_vk) = make_recursive_proof_struct(&env, &base_id, &vk, 1);
    let composer = Address::generate(&env);
    client.create_recursive_proof(
        &composer,
        &base_id,
        &rp.recursive_proof,
        &agg_vk,
        &1,
        &5_000,
    );
}

#[test]
fn test_recursive_proof_depth_3_passes() {
    let env = Env::default();
    env.mock_all_auths();
    env.ledger().set_timestamp(1_000_000);
    let (client, base_id, vk) = init_rec(&env);

    let (rp, agg_vk) = make_recursive_proof_struct(&env, &base_id, &vk, 3);
    let composer = Address::generate(&env);
    client.create_recursive_proof(
        &composer,
        &base_id,
        &rp.recursive_proof,
        &agg_vk,
        &3,
        &5_000,
    );
}

#[test]
fn test_recursive_proof_max_depth_10_passes() {
    let env = Env::default();
    env.mock_all_auths();
    env.ledger().set_timestamp(1_000_000);
    let (client, base_id, vk) = init_rec(&env);

    let (rp, agg_vk) = make_recursive_proof_struct(&env, &base_id, &vk, 10);
    let composer = Address::generate(&env);
    client.create_recursive_proof(
        &composer,
        &base_id,
        &rp.recursive_proof,
        &agg_vk,
        &10,
        &5_000,
    );
}

#[test]
fn test_recursive_proof_depth_0_rejected() {
    let env = Env::default();
    env.mock_all_auths();
    let (client, base_id, vk) = init_rec(&env);

    let (rp, agg_vk) = make_recursive_proof_struct(&env, &base_id, &vk, 1);
    let composer = Address::generate(&env);
    let r = client.try_create_recursive_proof(
        &composer,
        &base_id,
        &rp.recursive_proof,
        &agg_vk,
        &0,
        &5_000,
    );
    assert_eq!(r, Err(Ok(Error::RecursiveDepthExceeded)));
}

#[test]
fn test_recursive_proof_depth_11_rejected() {
    let env = Env::default();
    env.mock_all_auths();
    let (client, base_id, vk) = init_rec(&env);
    let (rp, _) = make_recursive_proof_struct(&env, &base_id, &vk, 11);
    let agg_vk = Bytes::from_slice(&env, &[0u8; 32]);
    let composer = Address::generate(&env);
    let r = client.try_create_recursive_proof(
        &composer,
        &base_id,
        &rp.recursive_proof,
        &agg_vk,
        &11,
        &5_000,
    );
    assert_eq!(r, Err(Ok(Error::RecursiveDepthExceeded)));
}

#[test]
fn test_recursive_proof_empty_aggregated_vk_rejected() {
    let env = Env::default();
    env.mock_all_auths();
    let (client, base_id, vk) = init_rec(&env);
    let (rp, _) = make_recursive_proof_struct(&env, &base_id, &vk, 1);
    let composer = Address::generate(&env);
    let r = client.try_create_recursive_proof(
        &composer,
        &base_id,
        &rp.recursive_proof,
        &Bytes::new(&env),
        &1,
        &5_000,
    );
    assert_eq!(r, Err(Ok(Error::InvalidProof)));
}

#[test]
fn test_recursive_proof_short_aggregated_vk_rejected() {
    let env = Env::default();
    env.mock_all_auths();
    let (client, base_id, vk) = init_rec(&env);
    let (rp, _) = make_recursive_proof_struct(&env, &base_id, &vk, 1);
    let composer = Address::generate(&env);
    let r = client.try_create_recursive_proof(
        &composer,
        &base_id,
        &rp.recursive_proof,
        &Bytes::from_slice(&env, b"tooshort"),
        &1,
        &5_000,
    );
    assert_eq!(r, Err(Ok(Error::InvalidProof)));
}

#[test]
fn test_recursive_proof_wrong_base_proof_id_rejected() {
    let env = Env::default();
    env.mock_all_auths();
    let (client, base_id, vk) = init_rec(&env);
    let wrong_base_id = BytesN::from_array(&env, &[0xffu8; 32]);
    let (rp, agg_vk) = make_recursive_proof_struct(&env, &base_id, &vk, 1);
    let composer = Address::generate(&env);
    // agg_vk was built for base_id, but we pass wrong_base_id in both slots
    // The contract checks ZKProof(wrong_base_id) exists first, which it doesn't
    let r = client.try_create_recursive_proof(
        &composer,
        &wrong_base_id,
        &rp.recursive_proof,
        &agg_vk,
        &1,
        &5_000,
    );
    assert_eq!(r, Err(Ok(Error::ProofNotFound)));
}

#[test]
fn test_recursive_proof_wrong_vk_hash_in_commitment_rejected() {
    let env = Env::default();
    env.mock_all_auths();
    let (client, base_id, vk) = init_rec(&env);
    let (rp, agg_vk) = make_recursive_proof_struct(&env, &base_id, &vk, 2);
    // Swap to a different recursive_proof with a different vk_hash (still depth=2)
    let wrong_vk = BytesN::from_array(&env, &[0xeeu8; 32]);
    let bad_inner = ZKProof {
        vk_hash: wrong_vk,
        ..rp.recursive_proof.clone()
    };
    let composer = Address::generate(&env);
    let r = client.try_create_recursive_proof(&composer, &base_id, &bad_inner, &agg_vk, &2, &5_000);
    assert_eq!(r, Err(Ok(Error::InvalidProof)));
}

#[test]
fn test_recursive_proof_wrong_depth_in_commitment_rejected() {
    let env = Env::default();
    env.mock_all_auths();
    let (client, base_id, vk) = init_rec(&env);
    // Build valid aggregated_vk for depth=1, but submit as depth=2
    let (rp, agg_vk) = make_recursive_proof_struct(&env, &base_id, &vk, 1);
    let composer = Address::generate(&env);
    let r = client.try_create_recursive_proof(
        &composer,
        &base_id,
        &rp.recursive_proof,
        &agg_vk,
        &2,
        &5_000,
    );
    assert_eq!(r, Err(Ok(Error::InvalidProof)));
}

#[test]
fn test_recursive_proof_tampered_first_byte_rejected() {
    let env = Env::default();
    env.mock_all_auths();
    let (client, base_id, vk) = init_rec(&env);
    let (rp, agg_vk) = make_recursive_proof_struct(&env, &base_id, &vk, 1);
    let mut agg_bytes = [0u8; 32];
    agg_vk.copy_into_slice(&mut agg_bytes);
    agg_bytes[0] ^= 0x01;
    let composer = Address::generate(&env);
    let r = client.try_create_recursive_proof(
        &composer,
        &base_id,
        &rp.recursive_proof,
        &Bytes::from_slice(&env, &agg_bytes),
        &1,
        &5_000,
    );
    assert_eq!(r, Err(Ok(Error::InvalidProof)));
}

#[test]
fn test_recursive_proof_tampered_last_byte_rejected() {
    let env = Env::default();
    env.mock_all_auths();
    let (client, base_id, vk) = init_rec(&env);
    let (rp, agg_vk) = make_recursive_proof_struct(&env, &base_id, &vk, 1);
    let mut agg_bytes = [0u8; 32];
    agg_vk.copy_into_slice(&mut agg_bytes);
    agg_bytes[31] ^= 0xff;
    let composer = Address::generate(&env);
    let r = client.try_create_recursive_proof(
        &composer,
        &base_id,
        &rp.recursive_proof,
        &Bytes::from_slice(&env, &agg_bytes),
        &1,
        &5_000,
    );
    assert_eq!(r, Err(Ok(Error::InvalidProof)));
}

#[test]
fn test_recursive_proof_extra_vk_bytes_still_passes() {
    let env = Env::default();
    env.mock_all_auths();
    env.ledger().set_timestamp(1_000_000);
    let (client, base_id, vk) = init_rec(&env);
    let (rp, agg_vk) = make_recursive_proof_struct(&env, &base_id, &vk, 1);
    // Append extra bytes after the 32-byte commitment prefix
    let mut extended = agg_vk;
    extended.append(&Bytes::from_slice(&env, b"extra_data"));
    let composer = Address::generate(&env);
    client.create_recursive_proof(
        &composer,
        &base_id,
        &rp.recursive_proof,
        &extended,
        &1,
        &5_000,
    );
}

#[test]
fn test_recursive_commitment_different_depths_produce_different_hashes() {
    let env = Env::default();
    let base_id = BytesN::from_array(&env, &[1u8; 32]);
    let vk = BytesN::from_array(&env, &[2u8; 32]);
    let inner = ZKProof {
        proof_type: ZKPType::Recursive,
        hash_function: ZKPHashFunction::Poseidon,
        circuit_id: String::from_str(&env, "c"),
        public_inputs: vec![&env, Bytes::from_slice(&env, b"x")],
        proof_data: Bytes::from_slice(&env, b"0123456789abcdef0123456789abcdef"),
        vk_hash: vk.clone(),
        verification_gas: 1_000,
        created_at: 0,
    };
    let rp1 = RecursiveProof {
        base_proof_id: base_id.clone(),
        recursive_proof: inner.clone(),
        aggregated_vk: Bytes::new(&env),
        composition_depth: 1,
        total_gas: 0,
        composed_at: 0,
    };
    let rp3 = RecursiveProof {
        base_proof_id: base_id.clone(),
        recursive_proof: inner.clone(),
        aggregated_vk: Bytes::new(&env),
        composition_depth: 3,
        total_gas: 0,
        composed_at: 0,
    };
    assert_ne!(
        recursive_commitment(&env, &rp1).to_array(),
        recursive_commitment(&env, &rp3).to_array()
    );
}

#[test]
fn test_recursive_commitment_different_base_ids_produce_different_hashes() {
    let env = Env::default();
    let base_a = BytesN::from_array(&env, &[0xaau8; 32]);
    let base_b = BytesN::from_array(&env, &[0xbbu8; 32]);
    let vk = BytesN::from_array(&env, &[2u8; 32]);
    let inner = ZKProof {
        proof_type: ZKPType::Recursive,
        hash_function: ZKPHashFunction::Poseidon,
        circuit_id: String::from_str(&env, "c"),
        public_inputs: vec![&env, Bytes::from_slice(&env, b"x")],
        proof_data: Bytes::from_slice(&env, b"0123456789abcdef0123456789abcdef"),
        vk_hash: vk.clone(),
        verification_gas: 1_000,
        created_at: 0,
    };
    let rp_a = RecursiveProof {
        base_proof_id: base_a,
        recursive_proof: inner.clone(),
        aggregated_vk: Bytes::new(&env),
        composition_depth: 1,
        total_gas: 0,
        composed_at: 0,
    };
    let rp_b = RecursiveProof {
        base_proof_id: base_b,
        recursive_proof: inner.clone(),
        aggregated_vk: Bytes::new(&env),
        composition_depth: 1,
        total_gas: 0,
        composed_at: 0,
    };
    assert_ne!(
        recursive_commitment(&env, &rp_a).to_array(),
        recursive_commitment(&env, &rp_b).to_array()
    );
}

#[test]
fn test_recursive_commitment_deterministic() {
    let env = Env::default();
    let base_id = BytesN::from_array(&env, &[1u8; 32]);
    let vk = BytesN::from_array(&env, &[2u8; 32]);
    let inner = ZKProof {
        proof_type: ZKPType::Recursive,
        hash_function: ZKPHashFunction::Poseidon,
        circuit_id: String::from_str(&env, "c"),
        public_inputs: vec![&env, Bytes::from_slice(&env, b"x")],
        proof_data: Bytes::from_slice(&env, b"0123456789abcdef0123456789abcdef"),
        vk_hash: vk.clone(),
        verification_gas: 1_000,
        created_at: 0,
    };
    let rp = RecursiveProof {
        base_proof_id: base_id,
        recursive_proof: inner,
        aggregated_vk: Bytes::new(&env),
        composition_depth: 2,
        total_gas: 0,
        composed_at: 0,
    };
    assert_eq!(
        recursive_commitment(&env, &rp).to_array(),
        recursive_commitment(&env, &rp).to_array()
    );
}

#[test]
fn test_recursive_proof_prop_any_byte_flip_in_commitment_fails() {
    let env = Env::default();
    env.mock_all_auths();
    env.ledger().set_timestamp(1_000_000);
    let (client, base_id, vk) = init_rec(&env);
    let (rp, agg_vk) = make_recursive_proof_struct(&env, &base_id, &vk, 1);
    let mut agg_bytes = [0u8; 32];
    agg_vk.copy_into_slice(&mut agg_bytes);

    for i in 0..32usize {
        let mut tampered = agg_bytes;
        tampered[i] ^= 0x40;
        let composer = Address::generate(&env);
        let r = client.try_create_recursive_proof(
            &composer,
            &base_id,
            &rp.recursive_proof,
            &Bytes::from_slice(&env, &tampered),
            &1,
            &5_000,
        );
        assert_eq!(
            r,
            Err(Ok(Error::InvalidProof)),
            "flip at byte {i} should fail"
        );
    }
}
