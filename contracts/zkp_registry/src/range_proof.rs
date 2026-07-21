use soroban_sdk::{Bytes, BytesN, Env};

use super::{Error, RangeProof};

/// Current proof version tag (4 bytes, big-endian).
/// Any proof_data with a different version tag returns `Error::VersionMismatch`.
const PROOF_VERSION: [u8; 4] = [0x00, 0x00, 0x00, 0x01];

/// Minimum proof_data length: 4-byte version tag + 32-byte commitment = 36 bytes.
pub const MIN_PROOF_LEN: u32 = 36;

const RANGE_DOMAIN: &[u8] = b"zkp:v1:rng:";

/// Computes the canonical range-proof commitment:
///   SHA256("zkp:v1:rng:" || vk_hash || min_value_be8 || max_value_be8 || SHA256(encrypted_value))
pub fn range_commitment(env: &Env, proof: &RangeProof) -> BytesN<32> {
    let enc_hash: BytesN<32> = env.crypto().sha256(&proof.encrypted_value).into();

    let mut msg = Bytes::new(env);
    msg.append(&Bytes::from_slice(env, RANGE_DOMAIN));
    msg.append(&Bytes::from_slice(env, &proof.vk_hash.to_array()));
    msg.append(&Bytes::from_slice(env, &proof.min_value.to_be_bytes()));
    msg.append(&Bytes::from_slice(env, &proof.max_value.to_be_bytes()));
    msg.append(&Bytes::from_slice(env, &enc_hash.to_array()));

    env.crypto().sha256(&msg).into()
}

/// Verifies a RangeProof.
///
/// proof_data layout:
///   [0..4]  version tag — must equal PROOF_VERSION, else `Error::VersionMismatch`
///   [4..36] range commitment — must equal `range_commitment(proof)`, else `Error::InvalidRangeProof`
pub fn verify_range_proof(env: &Env, proof: &RangeProof) -> Result<(), Error> {
    if proof.proof_data.len() < MIN_PROOF_LEN {
        return Err(Error::InvalidRangeProof);
    }

    // Check version tag
    let version_ok =
        (0u32..4u32).all(|i| proof.proof_data.get(i).unwrap_or(0) == PROOF_VERSION[i as usize]);
    if !version_ok {
        return Err(Error::VersionMismatch);
    }

    // Check commitment
    let expected = range_commitment(env, proof);
    let expected_arr = expected.to_array();
    for i in 0u32..32u32 {
        let actual = proof
            .proof_data
            .get(4 + i)
            .ok_or(Error::InvalidRangeProof)?;
        if actual != expected_arr[i as usize] {
            return Err(Error::InvalidRangeProof);
        }
    }

    Ok(())
}

/// Builds a valid proof_data for a RangeProof (for use in tests and client code).
#[allow(dead_code)] // Test-only call sites in `tests/` cannot be picked up by
                    // `cargo clippy --all-targets -D warnings` for production builds;
                    // kept as a public test-fixture factory.
pub fn make_range_proof_data(env: &Env, proof: &RangeProof) -> Bytes {
    let commitment = range_commitment(env, proof);
    let mut data = Bytes::from_slice(env, &PROOF_VERSION);
    data.append(&Bytes::from_slice(env, &commitment.to_array()));
    data
}
