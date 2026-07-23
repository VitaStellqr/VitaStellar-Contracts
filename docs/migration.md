# Data Migration System

## Overview
The Medical Records contract includes a built-in data migration system. This ensures that when the contract logic is upgraded (e.g., adding new fields to a struct), the existing data in storage is automatically updated to match the new format.

## How it Works
1.  **Versioning:** The contract tracks a `ContractVersion` in storage (default is 0).
2.  **Atomicity:** The `upgrade` function performs two actions in a single transaction:
    * Updates the WASM code.
    * Runs the `migrate_data` logic.
3.  **Safety:** If the migration logic fails (panics), the entire transaction reverts. The contract code remains on the old version, preventing data corruption.

## Operational Guide (For Admins)

To upgrade the contract, the Admin must perform the following:

1.  **Deploy** the new WASM file to the network to get its `wasm_hash`.
2.  **Call** the `upgrade` function on the *existing* contract:
    ```bash
    soroban contract invoke \
      --id <CONTRACT_ID> \
      --source <ADMIN_SECRET> \
      --network <NETWORK> \
      -- \
      upgrade \
      --new_wasm_hash <NEW_WASM_HASH>
    ```

## Developer Guide (Adding a New Migration)

When you make a "breaking change" to the data structure (e.g., V1 to V2):

1.  **Increment Version:**
    In `lib.rs`, update the constant:
    ```rust
    const CURRENT_CONTRACT_VERSION: u32 = 2; // Was 1
    ```

2.  **Add Migration Logic:**
    Inside `migrate_data`, add a specific handler for the new version gap:
    ```rust
    if current_version < 2 {
        // specific logic to transform data from V1 format to V2
        migrate_v1_to_v2(env);
    }
    ```

3.  **Test:**
    Add a test case in `tests/test_migration.rs` that explicitly sets up "Old Data" and verifies it transforms correctly into "New Data".

## Deprecating Old Entry Points During Migration

If a release keeps a legacy function temporarily available, do not remove it immediately. Instead:

1. Add `#[deprecated(...)]` to the old function.
2. Register that function in the upgradeability deprecation registry.
3. Emit a deprecation warning event from the old function body.
4. Point callers to the replacement function and planned removal version.

See [docs/deprecation_migration.md](./deprecation_migration.md) for the recommended pattern.

## Governor & Escrow: Single-Key Storage Migration (2026-07)

### Overview
`Governor` and `EscrowContract` previously stored all proposals/escrows
(and, for Governor, all votes) inside a single bulk `Map<u64, T>` under
one persistent storage key (`props`/`votes` for Governor, `escrow` for
EscrowContract). Every read or write of any single item deserialized
and re-serialized the *entire* collection, making `propose`, `cast_vote`,
and `create_escrow` scale O(n) with the number of items already stored -
eventually hitting Soroban's per-invocation CPU/IO resource limits.

### What changed
Each proposal, vote, and escrow now lives under its own persistent
storage key:

- `Governor`: `DataKey::Proposal(id)`, `DataKey::Vote(proposal_id, voter)`
- `EscrowContract`: `DataKey::Escrow(order_id)`

This makes every read/write O(1) regardless of how many items exist.
ID generation is unaffected: `Governor` already used a separate
`P_COUNT` instance-storage counter (unchanged); `EscrowContract` accepts
a caller-supplied `order_id` and did not need a counter.

### Migrating an existing deployment
Both contracts expose a `migrate_storage` entrypoint that copies every
item out of the legacy bulk map into its new individual key, then
removes the legacy bulk key. It is safe to call multiple times
(idempotent) - once the legacy key is gone, further calls are a cheap
no-op that return `0`.

- `Governor::migrate_storage(caller)` - gated to the configured
  `timelock` address (Governor has no separate admin role). Migrates
  both the legacy `props` and `votes` maps.
- `EscrowContract::migrate_storage(caller)` - gated to the contract
  `admin`. Migrates the legacy `escrow` map.

```bash
soroban contract invoke \
  --id <GOVERNOR_CONTRACT_ID> \
  --source <TIMELOCK_SECRET> \
  --network <NETWORK> \
  -- \
  migrate_storage \
  --caller <TIMELOCK_ADDRESS>

soroban contract invoke \
  --id <ESCROW_CONTRACT_ID> \
  --source <ADMIN_SECRET> \
  --network <NETWORK> \
  -- \
  migrate_storage \
  --caller <ADMIN_ADDRESS>
```

A freshly-initialized contract (no legacy data) can call
`migrate_storage` safely too - it detects the absence of the legacy key
and returns `0` immediately.
