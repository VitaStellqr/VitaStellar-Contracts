# ADR-003: Cross-chain identity relayer trust model

**Status:** Accepted
**Date:** 2026-06-19

## Context

`cross_chain_identity` attests that a Stellar address controls an address on
another chain (Ethereum, Polygon, Arbitrum, etc.). On a per-request basis the
contract needs to decide whose sign-off is good enough to flip a
`VerificationRequest` from `Pending` to `Approved`. Options considered:

1. **Trustless light-client verification** of the foreign chain's consensus —
   security-optimal but requires running (and gas-paying for) header sync
   inside the contract.
2. **Permissionless staking / slashing** of relayers — opens an open market
   but adds cryptoeconomic complexity that the contracts are not designed
   to host.
3. **Admin-curated validators** with a per-validator trust score and a
   minimum-attestation count.

## Decision

Use an **admin-curated, score-weighted validator set**. The admin calls
`add_validator` (and `update_trust_score` / `deactivate_validator`) to
maintain the set. Each `VerificationRequest` requires at least
`DEFAULT_MIN_ATTESTATIONS = 2` valid attestations before it is auto-promoted
to a `CrossChainIdentity`.

## Rationale

- **Simplicity** — a permissioned set avoids running a penalty/slashing
  ledger on top of an identity contract and fits the existing
  `access_utils::require_admin!` guard pattern.
- **Fast remediation** — the admin can immediately `deactivate_validator`
  on a compromised or dishonest relayer without a governance vote.
- **Bounded blast radius** — a single misbehaving validator cannot
  approve a request unilaterally because the `min_attestations` threshold
  enforces a quorum of two.
- **Trust score forward-compatibility** — `trust_score: u32` (0..=100) is
  plumbed through the storage layout so a future ADR can layer
  weight-based quorum without a key migration.

## Consequences

- The bridge functions as a **federated multisig on attestations**, not a
  trustless light-client. Auditors must trust the validator set's
  operational security.
- Liveness depends on at least two validators remaining online and
  honest. Drop below `MIN_ATTESTATIONS` and the contract reaches quorum
  failure on new requests.
- Admin key compromise lets an attacker approve or revoke identities at
  will; this is mitigated by the existing admin-rotation procedure
  (see `docs/GOVERNANCE_REFACTORING_GUIDE.md`).
