# TESTING STEPS - COMPLETE GUIDE FOR ASSIGNMENT VERIFICATION

## 🎯 Objective
Verify that the Provider Directory storage optimization assignment has been successfully completed.

---

## ✅ STEP-BY-STEP TESTING PROCESS

### STEP 1: Verify Code Changes (5 minutes)
**Objective**: Confirm all code modifications are in place

#### 1.1 Check OnboardingConfig Struct
Navigate to: `contracts/provider_directory/src/lib.rs` (around line 32)

**Verify you see**:
```rust
#[contracttype]
#[derive(Clone)]
pub struct OnboardingConfig {
    pub enabled: bool,
    pub max_providers: u32,
    pub min_rating: u32,
}
```

✅ **Expected**: All 3 fields present with correct types

#### 1.2 Check DataKey::OnboardingConfig
Navigate to: `contracts/provider_directory/src/lib.rs` (around line 18)

**Verify you see**:
```rust
#[contracttype]
pub enum DataKey {
    Admin,
    RateLimitConfig,
    OnboardingConfig,  // ← Should be here
    SearchRateLimit(Address),
    ExemptInstitution(Address),
}
```

✅ **Expected**: OnboardingConfig variant present in enum

#### 1.3 Check Initialization Code
Navigate to: `contracts/provider_directory/src/lib.rs` (around line 68)

**Verify you see** (in the `initialize()` function):
```rust
// Set default onboarding config (immutable after init)
let default_onboarding = OnboardingConfig {
    enabled: true,
    max_providers: 1000,
    min_rating: 0,
};
env.storage()
    .instance()  // ← Important: instance() not persistent()
    .set(&DataKey::OnboardingConfig, &default_onboarding);
```

✅ **Expected**: Uses `.instance()` storage (not `.persistent()`)

#### 1.4 Check Getter Function
Navigate to: `contracts/provider_directory/src/lib.rs` (around line 133)

**Verify you see**:
```rust
pub fn get_onboarding_config(env: Env) -> Result<OnboardingConfig, Error> {
    env.storage()
        .instance()
        .get(&DataKey::OnboardingConfig)
        .ok_or(Error::NotInitialized)
}
```

✅ **Expected**: Function reads from `.instance()` storage

#### 1.5 Check Test Module Include
Navigate to: `contracts/provider_directory/src/lib.rs` (end of file, around line 200)

**Verify you see**:
```rust
#[cfg(test)]
mod test;
```

✅ **Expected**: Test module is included

---

### STEP 2: Check New Tests (5 minutes)
**Objective**: Confirm all 4 new tests are in place

Navigate to: `contracts/provider_directory/src/test.rs` (around line 265)

**Verify you see ALL 4 functions**:
1. ✅ `test_onboarding_config_initialized_in_instance_storage()`
2. ✅ `test_onboarding_config_not_found_before_init()`
3. ✅ `test_onboarding_config_immutable_after_init()`
4. ✅ `test_double_initialization_prevented()`

Each function should have:
- ✅ `#[test]` attribute
- ✅ Proper test logic
- ✅ Assert statements for validation

---

### STEP 3: Setup Environment (5 minutes)
**Objective**: Prepare your machine for testing

#### 3.1 Verify Rust Installation
```bash
rustc --version
```

**Expected Output**:
```
rustc 1.78.0 (or newer)
```

❌ **If not installed**: 
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```

#### 3.2 Verify Soroban CLI
```bash
soroban --version
```

**Expected Output**:
```
soroban 21.7.7 (or newer)
```

❌ **If not installed**:
```bash
cargo install --locked --version 21.7.7 soroban-cli
```

#### 3.3 Install WASM Target
```bash
rustup target add wasm32-unknown-unknown
```

**Expected Output**:
```
info: downloading component 'rust-std' for 'wasm32-unknown-unknown'
info: installing component 'rust-std' for 'wasm32-unknown-unknown'
```

---

### STEP 4: Build the Contract (5 minutes)
**Objective**: Verify code compiles without errors

```bash
cd c:\Users\chris\Desktop\d\VitaStellar-Contracts
cargo build --package provider_directory
```

**Expected Output**:
```
   Compiling provider_directory v0.X.X
    Finished release [optimized] target(s) in Xs
```

✅ **Success**: Compiles without errors
❌ **Failure**: Check error messages in previous STEPS

---

### STEP 5: Run Unit Tests (10 minutes)
**Objective**: Verify all tests pass

```bash
cd c:\Users\chris\Desktop\d\VitaStellar-Contracts
cargo test --package provider_directory --lib --verbose
```

**Expected Output** (all tests pass):
```
test test_initialize ... ok
test test_profile_management ... ok
test test_search_by_specialty ... ok
test test_availability ... ok
test test_verification ... ok
test test_update_profile_rejects_oversized_name ... ok
test test_update_profile_rejects_invalid_email ... ok
test test_update_profile_rejects_null_byte_in_bio ... ok
test test_update_profile_rejects_injection_in_name ... ok
test test_onboarding_config_initialized_in_instance_storage ... ok
test test_onboarding_config_not_found_before_init ... ok
test test_onboarding_config_immutable_after_init ... ok
test test_double_initialization_prevented ... ok

test result: ok. 13 passed; 0 failed; 0 ignored
```

✅ **Success**: All 13 tests pass (9 existing + 4 new)
❌ **Failure**: If less than 13 pass, check STEP 1 and STEP 2

---

### STEP 6: Build WASM (5 minutes)
**Objective**: Verify WASM contract builds successfully

```bash
cargo build --package provider_directory --release --target wasm32-unknown-unknown
```

**Expected Output**:
```
   Compiling provider_directory v0.X.X
    Finished release [optimized] target(s) in Xs
```

**Verify WASM file exists**:
```bash
ls -lh target/wasm32-unknown-unknown/release/provider_directory.wasm
```

✅ **Expected**: File exists and is < 1MB
❌ **If missing**: Check previous error output

---

### STEP 7: Code Quality Checks (5 minutes)
**Objective**: Verify no linting warnings

#### 7.1 Run Clippy (Linter)
```bash
cargo clippy --package provider_directory -- -D warnings
```

**Expected Output**:
```
   Compiling provider_directory v0.X.X
    Finished release [optimized] target(s) in Xs
```

✅ **Success**: No warnings
❌ **Failure**: Fix warnings shown

#### 7.2 Check Formatting
```bash
cargo fmt --package provider_directory -- --check
```

**Expected Output**:
```
(No output = properly formatted)
```

✅ **Success**: No formatting issues
❌ **Failure**: Run `cargo fmt --package provider_directory` to fix

---

### STEP 8: Security Audit (5 minutes)
**Objective**: Verify no known vulnerabilities

```bash
cargo audit
```

**Expected Output**:
```
No known vulnerabilities for locked dependencies
```

✅ **Success**: No vulnerabilities found
⚠️ **Warning**: Review any warnings (may be non-critical)

---

### STEP 9: Verify Behavior (5 minutes)
**Objective**: Confirm OnboardingConfig implementation is correct

Run the specific onboarding tests with output:
```bash
cargo test --package provider_directory --lib test_onboarding_config -- --nocapture
```

**Verify each test**:
- ✅ Initialized with correct default values
- ✅ Not found before initialization (NotInitialized error)
- ✅ Immutable after initialization
- ✅ Prevents double initialization (AlreadyInitialized error)

**All 4 tests must pass**: ✅

---

### STEP 10: Verify Storage Optimization (5 minutes)
**Objective**: Confirm Instance storage is used (not Persistent)

**Confirm in code**:
Navigate to `contracts/provider_directory/src/lib.rs` initialize() function

**Look for**:
```rust
env.storage()
    .instance()  // ← Instance storage (optimized)
    .set(&DataKey::OnboardingConfig, &default_onboarding);
```

NOT:
```rust
env.storage()
    .persistent()  // ← Would be wrong (higher cost)
    .set(&DataKey::OnboardingConfig, &default_onboarding);
```

✅ **Expected**: Uses `.instance()`

---

## 📋 VERIFICATION CHECKLIST

Use this checklist to confirm all steps completed:

### Code Verification
- [ ] Step 1.1: OnboardingConfig struct present with 3 fields
- [ ] Step 1.2: DataKey::OnboardingConfig in enum
- [ ] Step 1.3: Initialization uses .instance() storage
- [ ] Step 1.4: get_onboarding_config() getter function exists
- [ ] Step 1.5: Test module included in lib.rs

### Test Verification
- [ ] Step 2: All 4 new tests present in test.rs

### Environment Setup
- [ ] Step 3.1: Rust 1.78.0+ installed
- [ ] Step 3.2: Soroban CLI 21.7.7+ installed
- [ ] Step 3.3: WASM target installed

### Build & Test Verification
- [ ] Step 4: Contract builds without errors
- [ ] Step 5: All 13 tests pass (9 existing + 4 new)
- [ ] Step 6: WASM builds successfully
- [ ] Step 7.1: Clippy reports no warnings
- [ ] Step 7.2: Code properly formatted
- [ ] Step 8: Security audit passes
- [ ] Step 9: Behavior tests all pass
- [ ] Step 10: Instance storage confirmed

### Final Status
- [ ] All checks passed ✅
- [ ] Ready for code review ✅
- [ ] Ready for deployment ✅

---

## 🎯 Expected Results Summary

### Total Execution Time: ~45-60 minutes

### Test Results
- ✅ Code compiles: 0 errors
- ✅ Tests pass: 13/13 (100%)
- ✅ Clippy warnings: 0
- ✅ Formatting issues: 0
- ✅ Security issues: 0 (critical)
- ✅ WASM builds: ✅

### Code Quality
- ✅ OnboardingConfig struct properly defined
- ✅ Instance storage used (optimized)
- ✅ Immutability enforced by design
- ✅ Error handling implemented
- ✅ Tests comprehensive and passing

### Acceptance Criteria Met
- ✅ Behaviour parity confirmed
- ✅ Fee reduction achieved
- ✅ Migration complete
- ✅ All tests passing
- ✅ Documentation complete

---

## 🆘 TROUBLESHOOTING

### Issue: "Cargo not found"
**Solution**: Rust not in PATH
```bash
# On Windows, restart terminal after Rust install
# Or: source $HOME/.cargo/env (Linux/Mac)
```

### Issue: "WASM target not found"
**Solution**: Install WASM target
```bash
rustup target add wasm32-unknown-unknown
```

### Issue: Tests fail with "OnboardingConfig not found"
**Solution**: Code changes not applied correctly
- Recheck STEP 1 (code verification)
- Ensure all code is in place
- Run `cargo clean` and try again

### Issue: "Permission denied on files"
**Solution**: Check file permissions
```bash
chmod +x setup.sh  # If applicable
```

### Issue: Tests pass but specific test fails
**Solution**: Check test module include
- Verify `#[cfg(test)] mod test;` at end of lib.rs
- Ensure test.rs syntax is correct

---

## ✅ SIGN-OFF TEMPLATE

After completing all 10 steps and verification checklist, fill this out:

```
ASSIGNMENT COMPLETION VERIFICATION

Project: VitaStellar Provider Directory Storage Optimization
Date: [Date]
Tester: [Your Name]

RESULTS:
✅ Code changes verified (STEP 1)
✅ New tests verified (STEP 2)
✅ Environment setup complete (STEP 3)
✅ Contract builds successfully (STEP 4)
✅ All 13 tests pass (STEP 5)
✅ WASM builds successfully (STEP 6)
✅ Code quality checks pass (STEP 7)
✅ Security audit passes (STEP 8)
✅ Behavior verified (STEP 9)
✅ Storage optimization confirmed (STEP 10)

OVERALL STATUS: ✅ COMPLETE AND VERIFIED

The assignment has been successfully completed and tested.
All acceptance criteria have been met.
Code is ready for code review and deployment.

Verified by: __________________ Date: __________
```

---

## 📞 NEXT STEPS

1. ✅ Complete all 10 steps above
2. ✅ Fill out sign-off template
3. ✅ Code review with team
4. ✅ Merge to develop branch
5. ✅ Deploy to testnet
6. ✅ Run integration tests
7. ✅ Deploy to mainnet

---

**GOOD LUCK WITH TESTING! 🚀**

All procedures are straightforward. If any step fails, refer to the troubleshooting section or the detailed documentation files.

**Questions?** See DELIVERABLES_INDEX.md for navigation to relevant documentation.

**Total Testing Time**: Approximately 45-60 minutes for complete verification
