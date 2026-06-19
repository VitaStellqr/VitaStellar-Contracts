# ADR-006: Cross-chain identity ownership representation

**Status:** Accepted
**Date:** 2026-06-19

## Context

A patient on Stellar may legitimately hold different addresses on each
external chain (a hardware-wallet Ethereum address, a hot-wallet Polygon
address, an exchange-controlled BSC address, etc.). The contract must
store each linkage independently so:

- Revoking access on Polygon does not affect the verified Ethereum link.
- An attacker cannot present an unrelated chain's address as a duplicate
  of the user's Stellar identity.
- Indexers and frontends can query a single endpoint for "is this
  Stellar address verified on chain X?".

Earlier revisions of `cross_chain_identity` accidentally stored every
identity under the same stringly-typed key (`"id_key"`) — see the in-file
`// BUG FIX:` comments. That class of bug must not be reachable again.

## Decision

Represent each cross-chain link as a single `CrossChainIdentity` struct
stored under a **composite, strongly-typed key**:

```rust
DataKey::Identity(Address /* stellar */, ChainId)
```

where `ChainId` is the closed enum
`Stellar | Ethereum | Polygon | Avalanche | BinanceSmartChain |
Arbitrum | Optimism | Custom(u32)`. A user can hold one `CrossChainIdentity`
per (stellar address, ChainId) pair; revocation is also per-pair.

## Rationale

- **One-to-many mapping without collision** — the composite key is unique
  by construction, eliminating the previous `id_key` over-write class of
  bug.
- **Type-safe storage** — `DataKey` is a `#[contracttype] enum`, so the
  Soroban host rejects any other shape; raw string keys are not used.
- **Per-chain lifecycle** — each `(stellar, chain)` pair has its own
  `VerificationStatus`, `verified_at`, `expires_at`, and `attestations`
  count, which mirrors off-chain reality.
- **Explicit extensibility** — adding a new chain family is a `Custom(u32)`
  entry in the enum plus a contract upgrade; we cannot accidentally
  collide with existing keys.

## Consequences

- Users **must** initiate a separate `request_verification` and sync for
  every external chain they want connected. There is no automatic
  fan-out.
- Frontends should query by `(stellar_address, ChainId)` rather than by
  Stellar address alone, otherwise they see only one of possibly many
  links.
- Adding a new chain requires a contract upgrade (the enum is closed),
  keeping the change auditable rather than runtime-open.
- The choice is documented in code at the `DataKey::Identity(Address,
  ChainId)` variant with a `// BUG FIX:` historical comment so future
  contributors do not regress to a single-string key.
