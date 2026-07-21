# Contract Interactions

## Registry Pattern

All production contracts resolve peer addresses through the `ContractRegistry`
instead of hardcoding them at initialization.

### How it works

1. **Deploy `ContractRegistry`** once and register every production contract:
   ```
   registry.set("governor", governor_address)
   registry.set("timelock", timelock_address)
   registry.set("escrow", escrow_address)
   ```

2. **Each contract stores the registry address** at init time (via `set_registry`
   or as an initialization parameter) and looks up peers at call time:
   ```
   let timelock_addr = registry.get("timelock").unwrap();
   ```

3. **Upgrades become atomic**: the `upgrade_manager` updates the registry entry
   for the upgraded contract, and all peers resolve the new address on the next
   call — no manual peer-contract updates required.

### Benefits

| Before (hardcoded) | After (registry) |
|---|---|
| O(n) manual updates on upgrade | O(1) registry update |
| Risk of partial updates | Atomic registry swap |
| No single source of truth | Canonical address map |

### Migration path

New contracts should accept a `registry_address` parameter at initialization.
Existing contracts gain a `set_registry` admin function. The upgrade manager
calls `registry.set(name, new_address)` atomically when deploying a new version.
