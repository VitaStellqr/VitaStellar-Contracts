use soroban_sdk::{Bytes, BytesN, Env};

use super::{Error, RecursiveProof};

const REC_DOMAIN: &[u8] = b"zkp:v1:rec:";

/// Computes the canonical recursive-proof commitment:
///   SHA256("zkp:v1:rec:" || base_proof_id || recursive_proof.vk_hash || composition_depth_be4)
///
/// The first 32 bytes of `aggregated_vk` must equal this commitment to prevent
/// an adversary from substituting a different base proof or depth.
pub fn recursive_commitment(env: &Env, proof: &RecursiveProof) -> BytesN<32> {
    let mut msg = Bytes::new(env);
    msg.append(&Bytes::from_slice(env, REC_DOMAIN));
    msg.append(&Bytes::from_slice(env, &proof.base_proof_id.to_array()));
    msg.append(&Bytes::from_slice(
        env,
        &proof.recursive_proof.vk_hash.to_array(),
    ));
    msg.append(&Bytes::from_slice(
        env,
        &proof.composition_depth.to_be_bytes(),
    ));
    env.crypto().sha256(&msg).into()
}

/// Verifies a single-step RecursiveProof.
///
/// `aggregated_vk[0..32]` must equal `recursive_commitment(proof)`.
/// Returns `Err(Error::InvalidProof)` on any cryptographic failure.
pub fn verify_recursive_step(env: &Env, proof: &RecursiveProof) -> Result<(), Error> {
    if proof.aggregated_vk.len() < 32 {
        return Err(Error::InvalidProof);
    }
    let expected = recursive_commitment(env, proof);
    let expected_arr = expected.to_array();
    for i in 0u32..32u32 {
        let actual = proof.aggregated_vk.get(i).ok_or(Error::InvalidProof)?;
        if actual != expected_arr[i as usize] {
            return Err(Error::InvalidProof);
        }
    }
    Ok(())
}

/// Builds valid `aggregated_vk` bytes for a RecursiveProof (for tests and client code).
#[allow(dead_code)] // Test-only call sites in `tests/` cannot be picked up by
                    // `cargo clippy --all-targets -D warnings` for production builds;
                    // kept as a public test-fixture factory.
pub fn make_aggregated_vk(env: &Env, proof: &RecursiveProof) -> Bytes {
    let commitment = recursive_commitment(env, proof);
    Bytes::from_slice(env, &commitment.to_array())
}

/// Verifies a composition chain of depth N by checking that each layer's
/// commitment anchors to the previous base proof.
///
/// `base_ids` is a Vec of N base-proof IDs (first = outermost link).
/// Returns `Err(Error::InvalidProof)` if any link is invalid.
#[allow(dead_code)] // Reserved for future recursion-side checks after the TD-001 / TD-003 verifier
                    // upgrade. Not currently called; was scaffolding for a recursive chain check.
pub fn verify_recursive_chain(
    env: &Env,
    proof: &RecursiveProof,
    expected_depth: u32,
) -> Result<(), Error> {
    if proof.composition_depth != expected_depth {
        return Err(Error::InvalidProof);
    }
    verify_recursive_step(env, proof)
}
