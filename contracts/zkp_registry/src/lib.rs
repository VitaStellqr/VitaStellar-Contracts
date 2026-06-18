#![no_std]
#![allow(clippy::too_many_arguments)]

#[cfg(test)]
mod test;

use soroban_sdk::{
    contract, contracterror, contractimpl, contracttype, symbol_short, Address, Bytes, BytesN, Env,
    String, Symbol, Vec,
};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[contracttype]
pub enum ZKPType {
    SNARK,
    STARK,
    Bulletproof,
    PedersenCommitment,
    Recursive,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[contracttype]
pub enum ZKPHashFunction {
    Poseidon,
    MiMC,
    SHA256,
    Rescue,
}

#[derive(Clone)]
#[contracttype]
pub struct ZKProof {
    pub proof_type: ZKPType,
    pub hash_function: ZKPHashFunction,
    pub circuit_id: String,
    pub public_inputs: Vec<Bytes>,
    pub proof_data: Bytes,
    pub vk_hash: BytesN<32>,
    pub verification_gas: u64,
    pub created_at: u64,
}

#[derive(Clone)]
#[contracttype]
pub struct MedicalRecordProof {
    pub patient_id: Address,
    pub record_id: u64,
    pub authenticity_proof: ZKProof,
    pub access_proof: ZKProof,
    pub metadata_hash: BytesN<32>,
    pub is_verified: bool,
    pub verified_at: u64,
}

#[derive(Clone)]
#[contracttype]
pub struct RangeProof {
    pub prover: Address,
    pub encrypted_value: Bytes,
    pub min_value: u64,
    pub max_value: u64,
    pub proof_data: Bytes,
    pub vk_hash: BytesN<32>,
    pub verification_gas: u64,
    pub created_at: u64,
}

#[derive(Clone)]
#[contracttype]
pub struct CredentialProof {
    pub holder: Address,
    pub credential_type: String,
    pub issuer: Address,
    pub validity_proof: ZKProof,
    pub attribute_proof: ZKProof,
    pub encrypted_expiration: Bytes,
    pub is_verified: bool,
    pub verified_at: u64,
}

#[derive(Clone)]
#[contracttype]
pub struct RecursiveProof {
    pub base_proof_id: BytesN<32>,
    pub recursive_proof: ZKProof,
    pub aggregated_vk: Bytes,
    pub composition_depth: u32,
    pub total_gas: u64,
    pub composed_at: u64,
}

#[derive(Clone)]
#[contracttype]
pub struct ZKPCircuitParams {
    pub circuit_id: String,
    pub circuit_type: ZKPType,
    pub num_public_inputs: u32,
    pub num_private_inputs: u32,
    pub num_constraints: u32,
    pub security_param: u32,
    pub vk_hash: BytesN<32>,
    pub pk_hash: BytesN<32>,
    pub setup_at: u64,
    pub trusted_setup: bool,
}

#[derive(Clone)]
#[contracttype]
pub struct ZKPVerificationResult {
    pub proof_id: BytesN<32>,
    pub is_valid: bool,
    pub gas_used: u64,
    pub verified_at: u64,
    pub verifier: Address,
    pub metadata: Bytes,
}

#[contracttype]
pub enum DataKey {
    Initialized,
    Admin,
    ZKProof(BytesN<32>),
    MedicalRecordProof(Address, u64),
    RangeProof(BytesN<32>),
    CredentialProof(Address, String),
    RecursiveProof(BytesN<32>),
    ZKPCircuitParams(String),
    VerificationResult(BytesN<32>),
    ProofCounter,
    GasTracker(Address),
}

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum Error {
    AlreadyInitialized = 1,
    NotInitialized = 2,
    NotAuthorized = 3,
    InvalidProof = 4,
    ProofNotFound = 5,
    CircuitNotFound = 6,
    VerificationFailed = 7,
    GasLimitExceeded = 8,
    InvalidInput = 9,
    InvalidRange = 10,
    CredentialExpired = 11,
    InvalidCircuit = 12,
    ProofTooLarge = 13,
    RecursiveDepthExceeded = 14,
    InvalidHashFunction = 15,
    CommitmentMismatch = 16,
}

const ADMIN: Symbol = symbol_short!("ADMIN");
const EXPIRATION_DOMAIN: &[u8] = b"zkp_registry:cred_exp";
const EXPIRATION_PAYLOAD_LEN: usize = 40;

#[contract]
pub struct ZKPRegistry;

#[contractimpl]
impl ZKPRegistry {
    pub fn initialize(env: Env, admin: Address) -> Result<(), Error> {
        admin.require_auth();
        if env.storage().instance().has(&DataKey::Initialized) {
            return Err(Error::AlreadyInitialized);
        }
        env.storage().instance().set(&DataKey::Initialized, &true);
        env.storage().instance().set(&DataKey::Admin, &admin);
        env.storage().persistent().set(&ADMIN, &admin);
        env.events()
            .publish((symbol_short!("zkp"), symbol_short!("init")), admin);
        Ok(())
    }

    #[allow(clippy::too_many_arguments)]
    pub fn register_circuit(
        env: Env,
        admin: Address,
        circuit_id: String,
        circuit_type: ZKPType,
        num_public_inputs: u32,
        num_private_inputs: u32,
        num_constraints: u32,
        security_param: u32,
        vk_hash: BytesN<32>,
        pk_hash: BytesN<32>,
        trusted_setup: bool,
    ) -> Result<(), Error> {
        admin.require_auth();
        Self::require_initialized(&env)?;

        if num_public_inputs > 50 || num_private_inputs > 100 || num_constraints > 10_000 {
            return Err(Error::InvalidCircuit);
        }

        let params = ZKPCircuitParams {
            circuit_id: circuit_id.clone(),
            circuit_type,
            num_public_inputs,
            num_private_inputs,
            num_constraints,
            security_param,
            vk_hash,
            pk_hash,
            setup_at: env.ledger().timestamp(),
            trusted_setup,
        };

        env.storage()
            .persistent()
            .set(&DataKey::ZKPCircuitParams(circuit_id.clone()), &params);
        env.events().publish(
            (symbol_short!("zkp"), symbol_short!("circ_reg")),
            circuit_id,
        );
        Ok(())
    }

    #[allow(clippy::too_many_arguments)]
    pub fn submit_zkp(
        env: Env,
        submitter: Address,
        proof_id: BytesN<32>,
        proof_type: ZKPType,
        hash_function: ZKPHashFunction,
        circuit_id: String,
        public_inputs: Vec<Bytes>,
        proof_data: Bytes,
        vk_hash: BytesN<32>,
        verification_gas: u64,
    ) -> Result<(), Error> {
        submitter.require_auth();
        Self::require_initialized(&env)?;

        if verification_gas > 100_000 {
            return Err(Error::GasLimitExceeded);
        }
        if proof_data.len() > 10_000 {
            return Err(Error::ProofTooLarge);
        }
        if !env
            .storage()
            .persistent()
            .has(&DataKey::ZKPCircuitParams(circuit_id.clone()))
        {
            return Err(Error::CircuitNotFound);
        }

        let proof = ZKProof {
            proof_type,
            hash_function,
            circuit_id,
            public_inputs,
            proof_data,
            vk_hash,
            verification_gas,
            created_at: env.ledger().timestamp(),
        };

        let is_valid = Self::verify_zkp_internal(&env, &proof)?;

        env.storage()
            .persistent()
            .set(&DataKey::ZKProof(proof_id.clone()), &proof);

        let result = ZKPVerificationResult {
            proof_id: proof_id.clone(),
            is_valid,
            gas_used: verification_gas,
            verified_at: env.ledger().timestamp(),
            verifier: submitter.clone(),
            metadata: Bytes::from_slice(&env, b"standard_verification"),
        };

        env.storage()
            .persistent()
            .set(&DataKey::VerificationResult(proof_id.clone()), &result);

        Self::track_gas_usage(&env, &submitter, verification_gas);

        env.events().publish(
            (symbol_short!("zkp"), symbol_short!("proof_sub")),
            (submitter, proof_id, is_valid),
        );

        if is_valid {
            Ok(())
        } else {
            Err(Error::VerificationFailed)
        }
    }

    #[allow(clippy::too_many_arguments)]
    pub fn create_medical_record_proof(
        env: Env,
        patient: Address,
        record_id: u64,
        authenticity_proof: ZKProof,
        access_proof: ZKProof,
        metadata_hash: BytesN<32>,
    ) -> Result<(), Error> {
        patient.require_auth();
        Self::require_initialized(&env)?;

        let auth_valid = Self::verify_zkp_internal(&env, &authenticity_proof)?;
        let access_valid = Self::verify_zkp_internal(&env, &access_proof)?;
        if !auth_valid || !access_valid {
            return Err(Error::VerificationFailed);
        }

        let proof = MedicalRecordProof {
            patient_id: patient.clone(),
            record_id,
            authenticity_proof,
            access_proof,
            metadata_hash,
            is_verified: true,
            verified_at: env.ledger().timestamp(),
        };

        env.storage().persistent().set(
            &DataKey::MedicalRecordProof(patient.clone(), record_id),
            &proof,
        );

        env.events().publish(
            (symbol_short!("zkp"), symbol_short!("med_proof")),
            (patient, record_id),
        );

        Ok(())
    }

    #[allow(clippy::too_many_arguments)]
    pub fn create_range_proof(
        env: Env,
        prover: Address,
        proof_id: BytesN<32>,
        encrypted_value: Bytes,
        min_value: u64,
        max_value: u64,
        proof_data: Bytes,
        vk_hash: BytesN<32>,
        verification_gas: u64,
    ) -> Result<(), Error> {
        prover.require_auth();
        Self::require_initialized(&env)?;

        if min_value >= max_value {
            return Err(Error::InvalidRange);
        }
        if verification_gas > 100_000 {
            return Err(Error::GasLimitExceeded);
        }

        let range_proof = RangeProof {
            prover: prover.clone(),
            encrypted_value,
            min_value,
            max_value,
            proof_data,
            vk_hash,
            verification_gas,
            created_at: env.ledger().timestamp(),
        };

        if !Self::verify_range_proof_internal(&env, &range_proof)? {
            return Err(Error::VerificationFailed);
        }

        env.storage()
            .persistent()
            .set(&DataKey::RangeProof(proof_id.clone()), &range_proof);
        Self::track_gas_usage(&env, &prover, verification_gas);

        env.events().publish(
            (symbol_short!("zkp"), symbol_short!("rng_proof")),
            (prover, proof_id, min_value, max_value),
        );

        Ok(())
    }

    #[allow(clippy::too_many_arguments)]
    pub fn create_credential_proof(
        env: Env,
        holder: Address,
        credential_type: String,
        issuer: Address,
        validity_proof: ZKProof,
        attribute_proof: ZKProof,
        encrypted_expiration: Bytes,
    ) -> Result<(), Error> {
        holder.require_auth();
        Self::require_initialized(&env)?;

        let valid_valid = Self::verify_zkp_internal(&env, &validity_proof)?;
        let attr_valid = Self::verify_zkp_internal(&env, &attribute_proof)?;
        if !valid_valid || !attr_valid {
            return Err(Error::VerificationFailed);
        }

        let valid_until = Self::decode_expiration(&env, &encrypted_expiration)?;
        if valid_until < env.ledger().timestamp() {
            return Err(Error::CredentialExpired);
        }

        let proof = CredentialProof {
            holder: holder.clone(),
            credential_type: credential_type.clone(),
            issuer,
            validity_proof,
            attribute_proof,
            encrypted_expiration,
            is_verified: true,
            verified_at: env.ledger().timestamp(),
        };

        env.storage().persistent().set(
            &DataKey::CredentialProof(holder.clone(), credential_type.clone()),
            &proof,
        );

        env.events().publish(
            (symbol_short!("zkp"), symbol_short!("cred_prf")),
            (holder, credential_type),
        );

        Ok(())
    }

    #[allow(clippy::too_many_arguments)]
    pub fn create_recursive_proof(
        env: Env,
        composer: Address,
        base_proof_id: BytesN<32>,
        recursive_proof: ZKProof,
        aggregated_vk: Bytes,
        composition_depth: u32,
        total_gas: u64,
    ) -> Result<(), Error> {
        composer.require_auth();
        Self::require_initialized(&env)?;

        if composition_depth > 10 {
            return Err(Error::RecursiveDepthExceeded);
        }
        if total_gas > 100_000 {
            return Err(Error::GasLimitExceeded);
        }
        if !env
            .storage()
            .persistent()
            .has(&DataKey::ZKProof(base_proof_id.clone()))
        {
            return Err(Error::ProofNotFound);
        }

        let recursive = RecursiveProof {
            base_proof_id,
            recursive_proof: recursive_proof.clone(),
            aggregated_vk,
            composition_depth,
            total_gas,
            composed_at: env.ledger().timestamp(),
        };

        if !Self::verify_recursive_proof_internal(&env, &recursive)? {
            return Err(Error::VerificationFailed);
        }

        let proof_id: BytesN<32> = env
            .crypto()
            .sha256(&recursive.recursive_proof.proof_data)
            .into();
        env.storage()
            .persistent()
            .set(&DataKey::RecursiveProof(proof_id.clone()), &recursive);

        Self::track_gas_usage(&env, &composer, total_gas);

        env.events().publish(
            (symbol_short!("zkp"), symbol_short!("rec_proof")),
            (composer, proof_id, composition_depth),
        );

        Ok(())
    }

    pub fn get_verification_result(
        env: Env,
        proof_id: BytesN<32>,
    ) -> Result<ZKPVerificationResult, Error> {
        Self::require_initialized(&env)?;
        env.storage()
            .persistent()
            .get(&DataKey::VerificationResult(proof_id))
            .ok_or(Error::ProofNotFound)
    }

    pub fn get_medical_record_proof(
        env: Env,
        patient: Address,
        record_id: u64,
    ) -> Result<MedicalRecordProof, Error> {
        Self::require_initialized(&env)?;
        env.storage()
            .persistent()
            .get(&DataKey::MedicalRecordProof(patient, record_id))
            .ok_or(Error::ProofNotFound)
    }

    pub fn get_range_proof(env: Env, proof_id: BytesN<32>) -> Result<RangeProof, Error> {
        Self::require_initialized(&env)?;
        env.storage()
            .persistent()
            .get(&DataKey::RangeProof(proof_id))
            .ok_or(Error::ProofNotFound)
    }

    pub fn get_credential_proof(
        env: Env,
        holder: Address,
        credential_type: String,
    ) -> Result<CredentialProof, Error> {
        Self::require_initialized(&env)?;
        env.storage()
            .persistent()
            .get(&DataKey::CredentialProof(holder, credential_type))
            .ok_or(Error::ProofNotFound)
    }

    pub fn get_circuit_params(env: Env, circuit_id: String) -> Result<ZKPCircuitParams, Error> {
        Self::require_initialized(&env)?;
        env.storage()
            .persistent()
            .get(&DataKey::ZKPCircuitParams(circuit_id))
            .ok_or(Error::CircuitNotFound)
    }

    pub fn get_gas_stats(env: Env, user: Address) -> Result<u64, Error> {
        Self::require_initialized(&env)?;
        Ok(env
            .storage()
            .persistent()
            .get(&DataKey::GasTracker(user))
            .unwrap_or(0))
    }

    fn require_initialized(env: &Env) -> Result<(), Error> {
        if env.storage().instance().has(&DataKey::Initialized) {
            Ok(())
        } else {
            Err(Error::NotInitialized)
        }
    }

    fn verify_zkp_internal(_env: &Env, proof: &ZKProof) -> Result<bool, Error> {
        if proof.proof_data.is_empty() || proof.proof_data.len() < 32 {
            return Err(Error::InvalidProof);
        }
        if proof.public_inputs.is_empty() || proof.public_inputs.len() > 50 {
            return Err(Error::InvalidProof);
        }
        for input in proof.public_inputs.iter() {
            if input.is_empty() {
                return Err(Error::InvalidProof);
            }
        }

        let verification_cost = match proof.proof_type {
            ZKPType::SNARK => match proof.hash_function {
                ZKPHashFunction::Poseidon => 50_000,
                ZKPHashFunction::MiMC => 45_000,
                ZKPHashFunction::SHA256 => 80_000,
                ZKPHashFunction::Rescue => 55_000,
            },
            ZKPType::STARK => 90_000,
            ZKPType::Bulletproof => 30_000,
            ZKPType::PedersenCommitment => 20_000,
            ZKPType::Recursive => 95_000,
        };

        Ok(verification_cost <= 100_000)
    }

    fn verify_range_proof_internal(_env: &Env, proof: &RangeProof) -> Result<bool, Error> {
        if proof.proof_data.is_empty() {
            return Ok(false);
        }
        if proof.min_value >= proof.max_value {
            return Ok(false);
        }
        Ok(true)
    }

    fn verify_recursive_proof_internal(_env: &Env, proof: &RecursiveProof) -> Result<bool, Error> {
        if proof.recursive_proof.proof_data.is_empty() {
            return Ok(false);
        }
        if proof.composition_depth > 10 {
            return Ok(false);
        }
        Ok(true)
    }

    fn track_gas_usage(env: &Env, user: &Address, gas_used: u64) {
        let gas_key = DataKey::GasTracker(user.clone());
        let current_gas: u64 = env.storage().persistent().get(&gas_key).unwrap_or(0);
        let total_gas = current_gas.saturating_add(gas_used);
        env.storage().persistent().set(&gas_key, &total_gas);
    }

    fn expected_expiration_commitment(env: &Env, valid_until: u64) -> BytesN<32> {
        let mut payload = Bytes::new(env);
        payload.append(&Bytes::from_slice(env, EXPIRATION_DOMAIN));
        payload.append(&Bytes::from_slice(env, &valid_until.to_be_bytes()));
        env.crypto().sha256(&payload).into()
    }

    fn decode_expiration(env: &Env, encrypted_expiration: &Bytes) -> Result<u64, Error> {
        if encrypted_expiration.len() as usize != EXPIRATION_PAYLOAD_LEN {
            return Err(Error::InvalidInput);
        }

        let mut raw = [0u8; EXPIRATION_PAYLOAD_LEN];
        encrypted_expiration.copy_into_slice(&mut raw);

        let mut ts_bytes = [0u8; 8];
        ts_bytes.copy_from_slice(&raw[..8]);
        let valid_until = u64::from_be_bytes(ts_bytes);

        let mut expected_bytes = [0u8; 32];
        expected_bytes.copy_from_slice(&raw[8..]);
        let expected = BytesN::from_array(env, &expected_bytes);
        let actual = Self::expected_expiration_commitment(env, valid_until);
        if actual != expected {
            return Err(Error::CommitmentMismatch);
        }

        Ok(valid_until)
    }
}
