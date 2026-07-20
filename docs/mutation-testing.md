# Mutation Testing with `cargo-mutants`

Mutation testing verifies that your test suite is capable of catching real bugs.
`cargo-mutants` introduces small mutations (mutants) into your source code and
checks whether your tests detect them. A **caught** mutant means your tests
caught the bug; a **missed** mutant indicates a gap.

## Setup

### Install

```sh
cargo install cargo-mutants
```

### Configuration

Project configuration lives in `.cargo-mutants.toml` at the repository root.

## Running Mutation Tests

### Per-contract

```sh
cargo mutants -p fp_math --timeout 120
cargo mutants -p medical_records --timeout 120
cargo mutants -p escrow --timeout 120
cargo mutants -p governor --timeout 120
cargo mutants -p identity_registry --timeout 120
```

### All target contracts at once

```sh
make mutation-test
```

### With focused function targets

```sh
# fp_math: arithmetic primitives
cargo mutants -p fp_math -f add -f mul -f div --timeout 120

# medical_records: access-control paths
cargo mutants -p medical_records -f grant_access -f revoke_access --timeout 120

# escrow: payment flow
cargo mutants -p escrow -f deposit -f release -f refund --timeout 120

# governor: governance decisions
cargo mutants -p governor -f propose -f execute -f vote --timeout 120

# identity_registry: DID operations
cargo mutants -p identity_registry -f register_did -f verify_did --timeout 120
```

## Interpreting Results

| Outcome | Meaning |
|---|---|
| `caught` | Test suite detected the mutation |
| `missed` | No test caught this mutation — add a test |
| `timeout` | Mutation caused an infinite loop — treat as caught |
| `unviable` | Mutation did not compile — not a real gap |

Results are written to `mutants.out/`.

## Baseline Mutation Scores

These are the **initial baseline** scores captured when setting up mutation
testing. Run `make mutation-test` and update this section as tests improve.

| Contract | Mutants | Caught | Missed | Score |
|---|---|---|---|---|
| `fp_math` | — | — | — | Target: 100% |
| `medical_records` | — | — | — | TBD |
| `escrow` | — | — | — | TBD |
| `governor` | — | — | — | TBD |
| `identity_registry` | — | — | — | TBD |

> **Goal**: `fp_math` must achieve 100% mutation score (all mutants caught).
> Other contracts should target >= 80% and improve over time.

### Updating Baselines

After running `make mutation-test`, copy the summary from `mutants.out/` and
update the table above. Commit the updated scores with your test improvements.

## CI Integration

A GitHub Actions workflow (`.github/workflows/mutation.yml`) runs mutation
tests **weekly** on a schedule. It is intentionally not run on every PR due to
the high compute cost of mutation testing.

To trigger manually:

```sh
gh workflow run mutation.yml
```

## Improving Mutation Scores

1. **Identify missed mutants** — review `mutants.out/` for `MISSED` entries
2. **Read the mutant diff** — understand what was changed in the source
3. **Add a targeted test** — write a test that would fail under that mutation
4. **Re-run** — verify the mutant is now `caught`

Common gaps:
- Unchecked arithmetic overflow/underflow
- Missing boundary-value assertions
- Access-control checks that don't verify all branches
- Error paths that are never exercised
