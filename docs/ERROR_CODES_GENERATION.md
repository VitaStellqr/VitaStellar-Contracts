# Error Code Reference Auto-Generation

This document explains how error codes are managed and auto-generated in VitaStellar-Contracts.

## Overview

Error codes are used throughout the contract ecosystem to provide stable, numeric identifiers for contract errors. To prevent drift between contract source and documentation, error codes are **auto-generated** from contract source.

## File Structure

- `docs/ERROR_CODES.md` — Auto-generated reference; **do not edit manually**
- `scripts/generate_error_codes.py` — Generator script that scans all contracts
- `.github/workflows/ci.yml` — CI enforces that docs match source

## Adding Error Codes

### 1. Define in Your Contract

In `contracts/YOUR_CONTRACT/src/errors.rs` (or `lib.rs`/`types.rs`), define your error enum with numeric codes:

```rust
#[contracterror]
#[repr(u32)]
pub enum Error {
    /// Your error description here
    SomeError = 123,
    AnotherError = 124,
}
```

**Code ranges by category:**
- `100–199`: Access Control
- `200–299`: Input Validation
- `300–399`: Lifecycle & State
- `400–499`: Entity Existence
- `500–599`: Financial & Resource
- `600–699`: Cryptography
- `700–799`: Cross-Chain
- `800–899`: Reentrancy & Safety

### 2. Regenerate Docs

After adding or modifying error codes, regenerate the reference:

```bash
python3 scripts/generate_error_codes.py --write
```

This scans all contracts and updates `docs/ERROR_CODES.md`.

### 3. Commit Changes

```bash
git add docs/ERROR_CODES.md contracts/YOUR_CONTRACT/src/errors.rs
git commit -m "feat: add error codes for YOUR_CONTRACT"
```

## CI Enforcement

GitHub Actions verifies that `docs/ERROR_CODES.md` matches the contract source on every PR:

- Runs `scripts/generate_error_codes.py --write`
- Checks if `docs/ERROR_CODES.md` changed
- **Fails the build if docs are out of sync**

This prevents manual edits to docs and ensures they always reflect the source of truth.

## Development Workflow

1. **Modify error codes** in `contracts/*/src/errors.rs`
2. **Run generator locally**: `python3 scripts/generate_error_codes.py --write`
3. **Commit both** the source and regenerated docs
4. **Push to PR** — CI will verify they're in sync

## Troubleshooting

### CI fails: "docs/ERROR_CODES.md is out of sync"

Run locally:
```bash
python3 scripts/generate_error_codes.py --write
git add docs/ERROR_CODES.md
git commit --amend
git push --force-with-lease
```

### How to view generated docs

```bash
# See the full reference
cat docs/ERROR_CODES.md

# Or regenerate to stdout
python3 scripts/generate_error_codes.py
```

## Legacy Contracts

Contracts using 1–99 codes (legacy numbering) are gradually being migrated to modern ranges (100+). See `CONTRIBUTING.md` for migration guidelines.

## FAQ

**Q: Can I manually edit `docs/ERROR_CODES.md`?**  
A: No. Edits will be overwritten on the next build or CI run. Modify the contract source instead.

**Q: Do I need to add doc comments?**  
A: Optional but recommended. Add `///` doc comments above error variants:

```rust
/// Caller is not authorized to perform this action
Unauthorized = 100,
```

These appear in the generated reference.

**Q: What if I add a duplicate error code?**  
A: The generator will include both entries. CI won't fail, but the duplicate will be visible in the reference and should be fixed in the contract source.

## See Also

- [docs/ERROR_CODES.md](../docs/ERROR_CODES.md) — Full error reference
- [scripts/check_error_codes.sh](../scripts/check_error_codes.sh) — Validation (legacy codes only)
