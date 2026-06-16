# Error Codes Quick Reference Guide

## Overview

VitaStellar uses numeric error codes organized by category. Each code maps to a unique error type across all contracts.

## Error Code Ranges

| Range | Category | Purpose |
|-------|----------|---------|
| 1–99 | Per-Contract | Unique to individual contracts (legacy/special) |
| 100–199 | Access Control | Authorization, authentication, permissions |
| 200–299 | Input Validation | Invalid arguments, format errors |
| 300–399 | Lifecycle & State | Initialization, pause, status transitions |
| 400–499 | Entity Existence | Not found, already exists |
| 500–599 | Financial & Resource | Funds, storage, limits |
| 600–699 | Cryptography | Key management, proofs |
| 700–799 | Cross-Chain | Bridge, oracle, chain operations |
| 800–899 | Reentrancy & Safety | Locking, circuit breaker |

## Adding a New Error Code

### Step 1: Choose the Right Range
Determine which category your error belongs to (see table above).

### Step 2: Find an Unused Code
```bash
# View all sorted codes in a category
grep -A 50 "^## Category Name" docs/ERROR_CODES.md | grep "^| [0-9]"

# Or use the analysis tool to find unused codes
python3 scripts/fix_error_codes.py
```

### Step 3: Add to `errors.rs`
Example (escrow contract):
```rust
#[contracterror]
pub enum Error {
    // ... existing errors ...
    MyNewError = 281,  // Choose unused code from appropriate range
}
```

### Step 4: Add to `docs/ERROR_CODES.md`
Find the appropriate section and add a row in sorted order:
```markdown
| 281 | `MyNewError` | escrow | Description of error | Common causes | How to fix |
```

**Keep tables sorted!** Insert in numeric order, not at the end.

### Step 5: Verify
```bash
./scripts/check_error_codes.sh
# OR
bash scripts/check_error_codes.sh
```

**Note**: Script requires bash. Do NOT use `sh scripts/check_error_codes.sh` as it will fail.

Should output:
```
✓ No duplicate codes in documentation
✓ Checked 14 error code files
✓ All error codes are valid and properly documented.
```

## Checking Current Codes

### View all codes in a category
```bash
grep -A 20 "^## Access Control" docs/ERROR_CODES.md
```

### Find unused codes
```bash
python3 scripts/fix_error_codes.py
# Shows unused codes in each range
```

### Run validation
```bash
./scripts/check_error_codes.sh
```

## Common Mistakes to Avoid

❌ **Don't**: Reuse a code already in ERROR_CODES.md
```rust
Unauthorized = 100,  // ✗ Already used by multiple contracts
```

✅ **Do**: Choose an unused code
```rust
MySpecialError = 143,  // ✓ Check ERROR_CODES.md first
```

---

❌ **Don't**: Add errors without sorting
```markdown
| 302 | `Error1` | contract | ...
| 281 | `Error2` | contract | ...  # ✗ Out of order
```

✅ **Do**: Keep tables sorted by code
```markdown
| 281 | `Error2` | contract | ...
| 302 | `Error1` | contract | ...  # ✓ Correct order
```

---

❌ **Don't**: Use codes outside the ranges
```rust
MyError = 999,  // ✗ Outside defined ranges
```

✅ **Do**: Use codes in appropriate category ranges
```rust
MyError = 450,  // ✓ In "Entity Existence" range (400-499)
```

## Quick Commands

| Task | Command |
|------|---------|
| View all error codes | `grep "^| [0-9]" docs/ERROR_CODES.md` |
| Check validation | `./scripts/check_error_codes.sh` |
| Analyze conflicts | `python3 scripts/fix_error_codes.py` |
| Find unused codes | `python3 scripts/fix_error_codes.py` |
| Count codes by range | `grep "^| [0-9]" docs/ERROR_CODES.md \| awk -F'[0-9]{3}' '{print $1}' \| sort \| uniq -c` |

## Validation Rules

The `check_error_codes.sh` script enforces:

1. ✅ No duplicate numeric codes in ERROR_CODES.md
2. ✅ All codes fall within approved ranges (100-999)
3. ✅ Per-contract codes (1-99) are contract-specific
4. ✅ Each code has a unique definition

## For Auditors

Verify error code integrity:
```bash
./scripts/check_error_codes.sh
```

Expected output:
```
✓ No duplicate codes in documentation
✓ Checked 14 error code files
✓ All error codes are valid and properly documented.
```

If this script returns non-zero, there's an issue to investigate.

## Reference Documents

- **Full Reference**: [docs/ERROR_CODES.md](docs/ERROR_CODES.md)
- **Validation Script**: [scripts/check_error_codes.sh](scripts/check_error_codes.sh)
- **Analysis Tool**: [scripts/fix_error_codes.py](scripts/fix_error_codes.py)
- **Recent Changes**: [DUPLICATE_FIX_DETAILS.md](DUPLICATE_FIX_DETAILS.md)

---

**Last Updated**: 2026-06-16  
**Status**: All error codes deduplicated and validated
