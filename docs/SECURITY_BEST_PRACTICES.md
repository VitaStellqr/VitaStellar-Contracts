# Security Best Practices Guide

This guide covers secure smart contract development patterns for VitaStellar Contracts on Soroban/Stellar. Follow these practices when writing, reviewing, or auditing contracts.

---

## 1. Access Control

### Always authenticate before authorizing

Every state-mutating function must call `require_auth()` on the caller before checking roles:

```rust
pub fn admin_action(env: Env, caller: Address) -> Result<(), Error> {
    caller.require_auth();           // 1. Authenticate (Soroban verifies signature)
    Self::require_admin(&env, &caller)?; // 2. Authorize (check role)
    // ... perform action
    Ok(())
}
```

Never skip `require_auth()` — without it, any address can impersonate the caller.

### Enforce single initialization

Use a guard that returns an error (not a panic) on re-initialization:

```rust
pub fn initialize(env: Env, admin: Address) -> Result<(), Error> {
    if env.storage().instance().has(&DataKey::Admin) {
        return Err(Error::AlreadyInitialized);
    }
    admin.require_auth();
    env.storage().instance().set(&DataKey::Admin, &admin);
    env.events().publish(("initialized",), admin);
    Ok(())
}
```

### Principle of least privilege

- Admin functions should only be callable by the stored admin address.
- Validator/operator functions should only be callable by registered validators.
- Read-only functions need no authentication.

---

## 2. Common Vulnerabilities and Defenses

### Integer Overflow / Underflow

Soroban runs in `no_std` with Rust's overflow checks disabled in release mode. Always use safe arithmetic:

```rust
// ❌ Unsafe
let new_count = count + 1;

// ✅ Safe — returns Err on overflow
let new_count = count.checked_add(1).ok_or(Error::Overflow)?;

// ✅ Safe — saturates at u64::MAX (for counters where overflow is impossible in practice)
let new_count = count.saturating_add(1);
```

### Reentrancy

Soroban contracts are single-threaded and do not support mid-execution callbacks in the same way EVM does. However, cross-contract calls can still cause unexpected state if not handled carefully:

- Write state **before** making cross-contract calls (checks-effects-interactions pattern).
- Validate return values from cross-contract calls.

### Replay Attacks

For cross-chain messages and meta-transactions, always enforce nonces:

```rust
fn verify_nonce(env: &Env, sender: &String, nonce: u64) -> Result<(), Error> {
    let stored: u64 = env.storage().persistent()
        .get(&DataKey::Nonce(sender.clone()))
        .unwrap_or(0);
    if nonce != stored.saturating_add(1) {
        return Err(Error::InvalidNonce);
    }
    Ok(())
}
```

### Denial of Service via Unbounded Loops

Avoid iterating over unbounded collections in a single transaction. Use pagination or batch limits:

```rust
// ❌ Unbounded — can exceed gas/instruction limits
for i in 0..total_alerts { ... }

// ✅ Bounded — caller controls batch size
let limit = count.min(MAX_BATCH_SIZE);
for i in 0..limit { ... }
```

---

## 3. Secure Coding Practices

### Validate all inputs

Check ranges, lengths, and invariants at function entry:

```rust
if threshold_bps == 0 || threshold_bps >= 10_000 {
    return Err(Error::InvalidThreshold);
}
if feature_count == 0 || feature_count > MAX_FEATURES {
    return Err(Error::InvalidFeatureCount);
}
```

### Use typed storage keys

Avoid string-based storage keys that can collide. Use a typed `DataKey` enum:

```rust
#[contracttype]
pub enum DataKey {
    Admin,
    Message(BytesN<32>),
    Nonce(String),
}
```

### Emit events for all state changes

Every state mutation should emit an event for off-chain auditability:

```rust
env.events().publish(
    (Symbol::new(&env, "RecordCreated"),),
    (record_id, actor, timestamp),
);
```

### Avoid `panic!` in production code

Use `Result<T, Error>` instead of `panic!`. Panics abort the transaction with an opaque error; typed errors give callers actionable information:

```rust
// ❌
if already_initialized { panic!("Already initialized"); }

// ✅
if already_initialized { return Err(Error::AlreadyInitialized); }
```

---

## 4. Cryptographic Guidance

- **Signatures**: Use `BytesN<64>` for Ed25519 signatures. Verify via Soroban's host functions, not custom implementations.
- **Hashes**: Use `BytesN<32>` for SHA-256 / Keccak-256 outputs. Do not truncate hashes.
- **Keys**: Never store private keys on-chain. Store only public keys or addresses.
- **Randomness**: Soroban does not provide on-chain randomness. Use commit-reveal schemes or oracle-provided randomness for any randomness requirement.
- **Encryption**: Encrypt sensitive data off-chain before storing on-chain. Store only ciphertext and the encryption key reference (not the key itself).

---

## 5. State Management Security

### Persistent vs. Instance vs. Temporary storage

| Type | Survives ledger close | Use for |
|---|---|---|
| `persistent` | Yes (with TTL) | Records, balances, long-lived state |
| `instance` | Yes (contract lifetime) | Admin, config, flags |
| `temporary` | No | Confirmations, short-lived locks |

Use `temporary` storage for data that should not persist (e.g., in-flight confirmations) to avoid state bloat.

### TTL management

Persistent storage entries expire. Extend TTLs for critical data:

```rust
env.storage().persistent().extend_ttl(&key, MIN_TTL, MAX_TTL);
```

---

## 6. Security Review Checklist

Before submitting a PR, verify:

- [ ] All admin/privileged functions call `require_auth()` and check roles
- [ ] No `panic!` or `unwrap()` in non-test code
- [ ] All arithmetic uses `checked_*` or `saturating_*`
- [ ] Nonces enforced for replay-sensitive operations
- [ ] Events emitted for all state changes
- [ ] Input validation at function entry
- [ ] No unbounded loops over storage
- [ ] Typed `DataKey` enum used (no raw string keys)
- [ ] Sensitive data encrypted before storage

---

## 7. Fuzz / Property-based Testing for Public Mutating Endpoints

Public mutating endpoints — anything that accepts arbitrary input from a
caller outside the trust boundary — must have at least one property-based
or fuzz harness. **Panics abort the transaction with an opaque error**,
and a panic at the project perimeter (an `analytics` ingest, a public
metadata setter, etc.) typically signals allocator abuse or DoS.

### Quick reference for adding a new harness

1. **Place the harness file** at
   `contracts/<your_contract>/fuzz/<harness_name>.rs`, then register it
   in the contract `Cargo.toml`:
   ```toml
   [dev-dependencies]
   proptest = "1.5"

   [[test]]
   name = "<harness_name>"
   path = "fuzz/<harness_name>.rs"
   ```
2. **Wire it into** `scripts/run_contract_fuzz.sh` so it runs in CI:
   ```bash
   cargo test -p <your_contract> --test <harness_name>
   ```
3. **Assert non-panicking invariants.** For an ingest like
   `record_call`, assert at minimum:
   - `try_record_call(...).is_ok()` for any valid-shaped input.
   - `get_function_metrics(name)` is populated after the call.
   - `call_count` is monotonically non-decreasing on repeat calls.

### Why `proptest` and not `cargo-fuzz`

`cargo-fuzz` requires **nightly** Rust and the `arbitrary` crate. This
workspace pins **stable** Rust (`rust-toolchain.toml` `channel = "1.92.0"`)
and explicitly notes that `arbitrary` was removed to fix workspace
inheritance (`Cargo.toml`). `proptest` runs on stable and matches the
convention already used in `scripts/run_contract_fuzz.sh` and
`docs/testing/CONTRACT_BEHAVIOR_FUZZING.md`.

### Reference harness

- `contracts/contract_usage_analytics/fuzz/fuzz_record_event.rs` —
  added by GWorld57 in response to issue #82. Targets the public
  `record_call` ingest endpoint, asserts non-panic on arbitrary
  inputs, and includes a regression case (`record_call_does_not_duplicate_function_names`)
  that catches a real unbounded-growth bug class.

---

## 8. Incident Response

If a vulnerability is discovered:

1. **Do not disclose publicly** until a fix is deployed.
2. Pause the affected contract immediately: `pause(env, admin)`.
3. Follow the incident postmortem process: `docs/INCIDENT_POSTMORTEM_GUIDELINES.md`.
4. Deploy a fix and verify via `scripts/verify_deployment.sh`.
5. Unpause and notify stakeholders.

For critical vulnerabilities, contact the security team directly before any public disclosure.
