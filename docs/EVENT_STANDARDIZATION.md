# Event Topic Standardization Guide

## Overview

All contract events must follow the standardized event topic format to enable generic subscription from off-chain indexers and consumers.

## Standard Format

```
vst/<contract_name>/<event_name>
```

Where:
- **vst** = VitaStellar prefix
- **contract_name** = Full contract folder name (lowercase with underscores)
- **event_name** = Event identifier (passed as Soroban Symbol)

## Implementation Pattern

Events are published using Soroban's event system with two components:

### Topic (String)
```rust
String::from_str(env, "vst/<contract_name>")
```

### Event Name (Symbol)
```rust
Symbol::new(env, "<event_name>")
```

## Example

For the `identity_registry` contract:

```rust
env.events().publish(
    (
        String::from_str(&env, "vst/identity_registry"),
        Symbol::new(&env, "DIDCreated"),
    ),
    (subject, did_string),
);
```

This creates an event with the combined topic: `vst/identity_registry/DIDCreated`

## Contract Name Mapping

The following contracts must use their FULL names (not abbreviations):

| Contract Folder | Topic Prefix |
|---|---|
| anomaly_detection | vst/anomaly_detection |
| anomaly_detector | vst/anomaly_detector |
| credential_registry | vst/credential_registry |
| cross_chain_access | vst/cross_chain_access |
| cross_chain_enhancements | vst/cross_chain_enhancements |
| cross_chain_identity | vst/cross_chain_identity |
| crypto_registry | vst/crypto_registry |
| healthcare_reputation | vst/healthcare_reputation |
| homomorphic_registry | vst/homomorphic_registry |
| medical_consent_nft | vst/medical_consent_nft |
| medical_record_backup | vst/medical_record_backup |
| contract_verification | vst/contract_verification |
| contract_usage_analytics | vst/contract_usage_analytics |

## Event Name Rules

1. **Max 32 bytes**: Soroban Symbol constraint
2. **No spaces**: Use underscores or camelCase
3. **Clear naming**: Use descriptive names

Examples:
- ✓ `DIDCreated`
- ✓ `credential_issued`
- ✓ `access_granted`
- ✗ `Did Created` (spaces not allowed)
- ✗ `very_long_event_name_that_exceeds_32_bytes_limit` (too long)

## Event Helper Pattern

Use helper functions in `src/events.rs` for common patterns:

```rust
use soroban_sdk::{Address, Env, String, Symbol};

pub fn emit_initialized(env: &Env, admin: &Address) {
    env.events().publish(
        (
            String::from_str(env, "vst/my_contract"),
            Symbol::new(env, "initialized"),
        ),
        (admin.clone(),),
    );
}

pub fn emit_action(env: &Env, actor: &Address, action: &String) {
    env.events().publish(
        (
            String::from_str(env, "vst/my_contract"),
            Symbol::new(env, "action"),
        ),
        (actor.clone(), action.clone()),
    );
}
```

## Migration Checklist

For each contract that needs migration:

- [ ] Identify all `env.events().publish()` calls
- [ ] Check if topic prefix uses full contract name
- [ ] Update abbreviated names to full names
- [ ] Verify event names are <= 32 bytes
- [ ] Test events are emitted correctly
- [ ] Run CI regression test
- [ ] Update EVENTS.md

## Automated Validation

A CI test (`tests/event_topic_standardization_test.rs`) validates:

1. All topic prefixes match `vst/<full_contract_name>`
2. No abbreviated contract names are used
3. Event names are <= 32 bytes
4. Event names contain no spaces

To run the test locally:

```bash
cargo test event_topic_standardization
```

## Off-Chain Consumer Pattern

Consumers can now generically subscribe to all events from a contract:

```javascript
// Subscribe to all identity_registry events
client.on('vst/identity_registry/*', (event) => {
  console.log('Event:', event.name, event.data);
});

// Subscribe to specific event
client.on('vst/identity_registry/DIDCreated', (event) => {
  console.log('DID created:', event.data);
});
```

## Backward Compatibility

The standardization applies to **new events only**. Existing events with deprecated prefixes may be maintained during a grace period, but must not be used in new code.

## Related

- [docs/EVENTS.md](./EVENTS.md) - Auto-generated event registry
- [contracts/contract_template/src/events.rs](../contracts/contract_template/src/events.rs) - Reference implementation
