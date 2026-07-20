# VitaStellar Contracts Integration Testing Framework

This framework provides a unified and simplified environment for testing complex interactions between multiple Soroban contracts in the VitaStellar ecosystem.

## Features

- **Unified Environment (`IntegrationTestEnv`)**: A wrapper around `soroban_sdk::Env` that pre-configures common settings and fixtures.
- **Healthcare Team Fixtures**: Automatically generates a complete set of test users (Admin, Doctors, Patients, etc.) using `HealthcareTeam`.
- **Time Control**: Easy-to-use methods for manipulating ledger time (`jump_time`, `set_time`).
- **Event Assertion**: Built-in helpers to verify that contracts are emitting the expected events (`assert_event_emitted`, `assert_event_topics`).
- **Multi-Contract Setup**: Streamlined process for registering and linking multiple contracts in a single test.
- **Registration Helpers**: Specialized methods for deploying and initializing common contracts like `MedicalRecords` and `SutToken`.

## Usage Guide

### 1. Initialize the Environment

```rust
use crate::utils::IntegrationTestEnv;

let test_env = IntegrationTestEnv::new();
let env = &test_env.env;
```

### 2. Access Test Users

```rust
let admin = &test_env.team.admin.address;
let doctor = &test_env.team.doctors[0].address;
let patient = &test_env.team.patients[0].address;
```

### 3. Deploy Contracts

You can deploy contracts manually or use the built-in helpers:

```rust
// Manual deployment
let medical_records_id = env.register_contract(None, MedicalRecordsContract);
let medical_records = MedicalRecordsContractClient::new(env, &medical_records_id);

// Using helpers (recommended)
let (records_id, records_client) = test_env.register_medical_records();
let (token_id, token_client) = test_env.register_token(&test_env.admin);
```

### 4. Control Time

```rust
// Advance time by 1 hour
test_env.jump_time(3600);

// Set to specific timestamp
test_env.set_time(2000000000);
```

### 5. Assert Events

```rust
// Verify that a specific event was emitted with certain topics
test_env.assert_event_topics(&contract_id, test_env.topics(&["EVENT", "REC_NEW"]));

// Verify full event data
test_env.assert_event_emitted(&contract_id, test_env.topics(&["EVENT", "REC_NEW"]), test_env.to_val(expected_data));
```

## Example Test

See `tests/integration/framework_tests.rs` for a complete demonstration of the framework in action.

## Shared Test Utilities

The shared test utilities live under `tests/utils` and are exposed through `tests/utils/mod.rs`. These helpers are intended for contributors writing integrations and contract tests across the VitaStellar repo.

- `tests/utils/contract_utils.rs` — `ContractSetup`, `assert_contract_error`, `assert_contract_success`, `to_soroban_string`, and timing helpers.
- `tests/utils/integration_framework.rs` — `IntegrationTestEnv`, `MockService`, time control helpers, event assertions, and contract registration helpers.
- `tests/utils/performance.rs` — `SorobanBenchmarkResult`, `SorobanBenchmarkSuite`, `PerformanceSuite`, `BenchmarkRunner`, and `LoadTest`.
- `tests/utils/test_fixtures.rs` — `UserFixtureFactory`, `HealthcareTeam`, `ScenarioFixture`, and reusable fixture scenarios.

### Using shared utilities

```rust
use crate::utils::{ContractSetup, IntegrationTestEnv, UserFixtureFactory};

let setup = ContractSetup::default().with_mock_auth();
let test_env = IntegrationTestEnv::default();
let team = UserFixtureFactory::create_healthcare_team(&test_env.env);
```

## Integration with CI

The framework is integrated into the standard Rust test suite. You can run the integration tests using:

```bash
make test-integration
# or
cargo test --test integration
```

## Healthcare Reputation Test Suite (Issue #193)

The original Python test suite at `tests/healthcare_reputation_test.py` only
mocked the contract responses with hardcoded return values, meaning it could
not catch regressions in the actual on-chain logic and required Python +
`stellar-sdk` dependencies to run. To address this, the suite has been
replaced by a native Rust integration test file:
`tests/integration/healthcare_reputation.rs`.

**Approach chosen:** Rust integration tests (preferred per the issue brief).

**Rationale:**

- **Real contract behavior**: under `soroban_sdk::Env::default()`, the tests
  run against the actual `HealthcareReputationSystem` contract code instead
  of returning canned JSON from a Python helper. This catches behavioral
  regressions that the Python mocks would never surface.
- **Zero external dependencies**: removes the Python + `stellar-sdk`
  requirement from CI. The CI `test` job picks the new file up via
  `cargo test --workspace` (or directly via `cargo test --test integration
  healthcare_reputation`).
- **Shared tooling**: reuses `tests/utils` (e.g. `Env::default()` plus
  `mock_all_auths()`) so contributors have a single, idiomatic place to add
  new healthcare-reputation coverage going forward.
- **Parity with the original scenarios**: every scenario exercised by the
  Python file maps to a Rust test in the new file — credentials, reputation
  scoring, patient feedback, conduct tracking, dispute workflow, and
  expired-credential detection.

The Python file is retained for history but is no longer referenced from CI
and is expected to be removed in a follow-up cleanup.

