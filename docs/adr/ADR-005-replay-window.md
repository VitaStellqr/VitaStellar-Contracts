# ADR-005: Cross-chain identity replay window

**Status:** Accepted
**Date:** 2026-06-19

## Context

A `VerificationRequest` is created on Stellar, then validators race to
attest before the request expires. Two failure modes have to be bounded:

- **Stale requests** — a request whose underlying foreign-chain state is
  no longer reflects reality (re-orgs, key rotations, key revocations).
- **Replay** — the same signed payload re-submitted after the original
  context has lapsed.

Soroban `env.ledger().timestamp()` is monotonic and consensus-final, so
the contract can use it as the source of truth. The previously-adopted
constant `REQUEST_EXPIRY = 86_400` (24 hours) is the only thing standing
between `Pending` and `Expired`.

## Decision

Use a **24-hour replay and state window**, defined by the
`REQUEST_EXPIRY = 86_400` constant. A `VerificationRequest` whose
`created_at + REQUEST_EXPIRY` has elapsed transitions to
`RequestStatus::Expired` and becomes non-attestable.

The longer-lived **`DEFAULT_IDENTITY_TTL = 31_536_000`** (1 year) is a
separate knob that controls the *resulting* `CrossChainIdentity`'s
`expires_at`; it does **not** interact with the request replay window.

## Rationale

- **Finality padding** — 24 h comfortably covers finality for slow L1s
  (Ethereum ~12 min probabilistic, ~15 min economic; Polygon checkpoints
  ~30 min) plus relayer downtime.
- **Storage hygiene** — every request consumed inside the window
  eliminates dangling `Pending` entries that would otherwise sit
  in `persistent` storage forever.
- **No extra dependency** — Soroban-ledger timestamps are
  consensus-final, so we don't need an oracle or block header.

## Consequences

- Relayers must produce and submit attestations within **24 hours** of
  `request_verification`. Miss the window and the request is
  permanently `Expired`; resubmission is required.
- A user that loses liveness on two validators simultaneously will time
  out and have to restart the flow. This is intentional — the contract
  prefers a clean restart over a stale attestation.
- Adjusting the window requires a contract upgrade (the constant is
  baked into the WASM); if we ever shorten it, document the audit
  rationale in a successor ADR.
