# Bugfix Requirements Document

## Introduction

Event topics emitted by production contracts use inconsistent, ad-hoc prefixes — or no prefix at all. The codebase shows at least four distinct prefix styles in active use: bare event names (e.g. `DIDCreated`, `Attested`), domain-scoped pairs (e.g. `AUDIT·LOG`, `consent·issued`), abbreviation-based pairs (e.g. `phs·alert_crt`, `mpc·start`), and contract-local acronyms (e.g. `ZKVER·ATTEST`, `DT_INIT`). `docs/EVENTS.md` outlines the intended standard — the `vst/<contract>/<event>` prefix scheme — but none of the production contracts have adopted it.

Because there is no stable, predictable prefix, off-chain consumers (indexers, monitoring scripts such as `scripts/monitor_deployments.sh`) cannot subscribe generically; they must hard-code every known prefix variant, and any new contract or renamed event silently breaks their filters.

The fix is to standardise all event topic tuples to begin with the `vst/` namespace, following the `vst/<contract>/<event>` pattern documented in `docs/EVENTS.md`. The `contract_template` helper functions and the `identity_registry` contract are the first targets, establishing the pattern for the remaining production contracts.

## Bug Analysis

### Current Behavior (Defect)

1.1 WHEN any production contract emits an event THEN the system publishes a topic tuple whose first element does not start with `vst/`, using instead one of many inconsistent styles (bare name, domain abbreviation, contract acronym, or snake_case label).

1.2 WHEN `contracts/contract_template/src/events.rs` emits `init`, `adm_xfer`, or `upd_data` THEN the system publishes topics without a `vst/` prefix, making the template a non-conforming reference for new contracts.

1.3 WHEN `contracts/identity_registry` emits events such as `DIDCreated`, `Attested`, `CredentialIssued`, `Init`, or `Initialized` THEN the system publishes topics that carry no standardised namespace prefix.

1.4 WHEN `docs/EVENTS.md` is regenerated from source THEN the system produces a registry that lists events under mixed prefix conventions, confirming no contract has adopted the documented standard.

1.5 WHEN CI runs THEN the system has no automated check that detects non-conforming event topic prefixes, so prefix drift accumulates silently across pull requests.

### Expected Behavior (Correct)

2.1 WHEN any production contract emits an event THEN the system SHALL publish a topic tuple whose first element is a symbol starting with `vst/`, following the format `vst/<contract_name>/<event_name>`.

2.2 WHEN `contracts/contract_template/src/events.rs` emits `init`, `adm_xfer`, or `upd_data` THEN the system SHALL publish topics prefixed as `vst/contract_template/init`, `vst/contract_template/adm_xfer`, and `vst/contract_template/upd_data` respectively.

2.3 WHEN `contracts/identity_registry` emits `DIDCreated`, `Attested`, `CredentialIssued`, `Init`, or `Initialized` THEN the system SHALL publish topics prefixed with `vst/identity_registry/` as the first tuple element.

2.4 WHEN `docs/EVENTS.md` is regenerated THEN the system SHALL produce a registry where every entry's Topics column begins with a `vst/` symbol.

2.5 WHEN CI runs THEN the system SHALL execute a regression test that scans all `contracts/**/src/**/*.rs` event publish calls and fails the build if any topic tuple's first element does not start with `vst/`.

### Unchanged Behavior (Regression Prevention)

3.1 WHEN a consumer subscribes to a `vst/`-prefixed topic THEN the system SHALL CONTINUE TO deliver the same event payload (field values and tuple arity) that the contract emitted before the prefix migration.

3.2 WHEN `contracts/contract_template` compiles after the migration THEN the system SHALL CONTINUE TO expose the same public `emit_*` function signatures so that contracts derived from the template do not need signature-level changes.

3.3 WHEN `contracts/identity_registry` processes identity operations (DID creation, credential issuance, recovery, attestation) THEN the system SHALL CONTINUE TO emit events for every operation that previously emitted one — no events may be silently dropped by the migration.

3.4 WHEN existing unit and integration tests for migrated contracts run THEN the system SHALL CONTINUE TO pass, with topic assertions updated to match the new `vst/` prefix.

3.5 WHEN contracts not yet migrated emit events THEN the system SHALL CONTINUE TO operate under their current (non-`vst/`) prefix until they are explicitly migrated, so that in-flight deployments are not broken by a partial rollout.

---

## Derived Bug Condition

**Bug Condition Function:**

```pascal
FUNCTION isBugCondition(topic_tuple)
  INPUT: topic_tuple — the topics argument passed to env.events().publish(...)
  OUTPUT: boolean

  first_element ← topic_tuple[0]
  RETURN NOT starts_with(string_value(first_element), "vst/")
END FUNCTION
```

**Fix-Checking Property:**

```pascal
// Property: every newly-emitted topic starts with vst/
FOR ALL emit_call WHERE isBugCondition(emit_call.topics) DO
  result ← migrated_emit_call.topics
  ASSERT starts_with(string_value(result[0]), "vst/")
END FOR
```

**Preservation Property:**

```pascal
// Property: payload is unchanged by the prefix migration
FOR ALL emit_call WHERE NOT isBugCondition(emit_call.topics) DO
  // Once migrated, payload fields must equal the pre-migration payload fields
  ASSERT migrated_emit_call.payload = original_emit_call.payload
END FOR
```
