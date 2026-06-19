extern crate std;

use super::*;
use soroban_sdk::testutils::{Address as _, Ledger};
use soroban_sdk::{vec, Address, Bytes, BytesN, Env, String};

fn setup(env: &Env) -> (ZKPRegistryClient<'_>, Address) {
    let contract_id = env.register_contract(None, ZKPRegistry);
    let client = ZKPRegistryClient::new(env, &contract_id);
    (client, contract_id)
}

fn make_proof(
    env: &Env,
    label: &'static [u8],
    proof_type: ZKPType,
    hash: ZKPHashFunction,
) -> ZKProof {
    ZKProof {
        proof_type,
        hash_function: hash,
        circuit_id: String::from_str(env, "circuit"),
        public_inputs: vec![env, Bytes::from_slice(env, label)],
        proof_data: Bytes::from_slice(env, b"0123456789abcdef0123456789abcdef"),
        vk_hash: BytesN::from_array(env, &[1u8; 32]),
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
    let proof = Bytes::from_slice(&env, b"0123456789abcdef0123456789abcdef");

    client.submit_zkp(
        &submitter,
        &proof_id,
        &ZKPType::SNARK,
        &ZKPHashFunction::Poseidon,
        &circuit_id,
        &inputs,
        &proof,
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
