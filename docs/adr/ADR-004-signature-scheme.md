# ADR-004: Cross-chain identity signature scheme

**Status:** Accepted
**Date:** 2026-06-19

## Context

`cross_chain_identity` accepts external-chain proofs (the `proof:
BytesN<64>` field of `VerificationRequest`) and validator attestations
(`signature: BytesN<64>` on `Attestation`). We need to declare one
signature scheme end-to-end so off-chain relayers, validators, and
future SDKs interoperate.

Options considered:

1. **Ed25519** (`BytesN<64>` signatures, `BytesN<32>` public keys) —
   native to Soroban via `env.crypto().ed25519_verify`.
2. **Secp256k1 / ECDSA** — matches the dominant signature scheme on
   Ethereum and other EVM chains, but requires a host-function or
   in-contract implementation that increases WASM size and gas.
3. **Multi-scheme / per-chain** — accept whatever the foreign chain
   uses natively. Simpler for relayers, but pushes verification
   complexity into the contract for every supported chain.

## Decision

Standardise on **Ed25519** for both:

- The `proof: BytesN<64>` field submitted by the user to
  `request_verification`.
- The `signature: BytesN<64>` field stored on each `Attestation`.
- `public_key: BytesN<32>` on each `IdentityValidator`.

## Rationale

- **Native verification** — Soroban exposes `env.crypto().ed25519_verify`
  as a host function, so verification does not allocate WASM pages for a
  big-number arithmetic library.
- **Single primitive** — relayers, validators, and the on-chain
  verifier all use the same key encoding; no per-chain branching.
- **Forward-compatible with relayer diversity** — relayers can hold
  their own Ed25519 keypair and rotate independently of the validator
  set.

## Consequences

- All off-chain relayers and validators **must** sign and verify with
  Ed25519. EVM-side signatures (ECDSA) cannot be used directly without
  wrapping in an off-chain adapter.
- **Known implementation gap (as of this ADR):** `cross_chain_identity`
  stores the 64-byte signature blob and the validator's 32-byte public
  key but does not yet call `env.crypto().ed25519_verify()` against
  them. Follow-up issue: enforce cryptographic verification on the
  `attest_verification` and `request_verification` paths before any
  mainnet promotion. Until that lands, the contract must be treated
  as storing **attestation receipts**, not cryptographically-verified
  attestations.
- Future signature-scheme work must either supersede this ADR or
  extend the `Attestation` shape in a versioned way; do not
  reinterpret the existing `BytesN<64>` field.
