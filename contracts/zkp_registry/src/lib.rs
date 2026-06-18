#![no_std]

use soroban_sdk::{
    contract, contracterror, contractimpl, contracttype, Address, Bytes, BytesN, Env, String,
};

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum Error {
    NotInitialized = 1,
    AlreadyInitialized = 2,
    NotFound = 3,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ZKPType {
    SNARK,
    STARK,
    Bulletproofs,
    PedersenCommitment,
    Recursive,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ZKPHashFunction {
    Poseidon,
    MiMC,
    SHA256,
    Rescue,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ZKProof {
    pub proof_type: ZKPType,
    pub hash_function: ZKPHashFunction,
    pub circuit_id: String,
    pub public_inputs: Bytes,
    pub proof_data: Bytes,
    pub vk_hash: BytesN<32>,
    pub verification_gas: u64,
    pub created_at: u64,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
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

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
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

#[contracttype]
pub enum DataKey {
    Initialized,
    Admin,
    RangeProof(BytesN<32>),
    CircuitParams(String),
    GasStats(Address),
}

#[contract]
pub struct ZkpRegistryContract;

#[contractimpl]
impl ZkpRegistryContract {
    pub fn initialize(env: Env, admin: Address) -> Result<(), Error> {
        admin.require_auth();
        if env.storage().instance().has(&DataKey::Initialized) {
            return Err(Error::AlreadyInitialized);
        }

        env.storage().instance().set(&DataKey::Initialized, &true);
        env.storage().instance().set(&DataKey::Admin, &admin);
        Ok(())
    }

    pub fn get_range_proof(env: Env, proof_id: BytesN<32>) -> Result<RangeProof, Error> {
        env.storage()
            .persistent()
            .get(&DataKey::RangeProof(proof_id))
            .ok_or(Error::NotFound)
    }

    pub fn get_circuit_params(env: Env, circuit_id: String) -> Result<ZKPCircuitParams, Error> {
        env.storage()
            .persistent()
            .get(&DataKey::CircuitParams(circuit_id))
            .ok_or(Error::NotFound)
    }

    pub fn get_gas_stats(env: Env, user: Address) -> Result<u64, Error> {
        env.storage()
            .persistent()
            .get(&DataKey::GasStats(user))
            .ok_or(Error::NotFound)
    }
}
