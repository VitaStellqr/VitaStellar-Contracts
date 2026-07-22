# Security Policy

VitaStellar is a decentralized health and wellness platform that handles Protected Health Information (PHI), patient-controlled medical records, and financial transactions on the Stellar blockchain. We treat security as a first-class concern. This document defines the scope of our security program, how to classify and report vulnerabilities, the response commitments you can expect from the maintainer team, and the safe harbor protections extended to good-faith researchers.

For complementary threat context, see [`docs/THREAT_MODEL.md`](../docs/THREAT_MODEL.md) and [`docs/CRYPTOGRAPHIC_SECURITY_MODEL.md`](../docs/CRYPTOGRAPHIC_SECURITY_MODEL.md).

---

## Table of Contents

1. [Scope](#1-scope)
2. [Severity Classification Matrix](#2-severity-classification-matrix)
3. [Response SLAs](#3-response-slas)
4. [How to Report a Vulnerability](#4-how-to-report-a-vulnerability)
5. [Bug Bounty Program](#5-bug-bounty-program)
6. [Safe Harbor Statement](#6-safe-harbor-statement)
7. [Out-of-Scope Items](#7-out-of-scope-items)
8. [Disclosure Policy](#8-disclosure-policy)

---

## 1. Scope

### In Scope

All Soroban smart contracts in the `contracts/` directory of this repository are in scope for security research. This includes, but is not limited to:

**Healthcare & Patient Data Contracts**
- `contracts/medical_records` — on-chain patient record management with encrypted-only mode; plaintext leakage here is a Critical severity issue
- `contracts/medical_record_backup` — disaster recovery for medical record data
- `contracts/medical_record_search` — indexing and querying over encrypted record metadata
- `contracts/medical_consent_nft` — NFT-based patient consent lifecycle management
- `contracts/patient_risk_stratification` — AI-driven clinical risk scoring
- `contracts/predictive_analytics` — analytics models for clinical decision support

**Identity & Access Control Contracts**
- `contracts/identity_registry` — W3C DID-based identity management; unauthorized DID registration is High or Critical
- `contracts/credential_registry` — verifiable credential issuance and revocation
- `contracts/crypto_registry` — public key bundles and post-quantum key slots
- `contracts/secure_enclave` — trusted execution environment coordination
- `contracts/reputation` — on-chain reputation scores
- `contracts/healthcare_reputation` — provider reputation with healthcare-specific attributes
- `contracts/reputation_access_control` — reputation-gated access to sensitive resources
- `contracts/reputation_integration` — cross-contract reputation aggregation

**Financial Contracts**
- `contracts/escrow` — general-purpose escrow; fund-locking bugs are Critical
- `contracts/appointment_booking_escrow` — patient-provider appointment payments; unauthorized fund release is Critical
- `contracts/token_sale` — SUT token sale mechanics; price manipulation is Critical
- `contracts/sut_token` — native utility token; mint/burn authorization flaws are Critical
- `contracts/payment_router` — multi-hop payment routing across providers

**Governance & Upgrade Contracts**
- `contracts/governor` — proposal creation and voting; vote manipulation is High
- `contracts/timelock` — queued execution with delay; bypass of timelock delay is Critical
- `contracts/upgrade_manager` — contract upgrade authorization; unauthorized upgrade is Critical
- `contracts/upgradeability` — upgrade primitives and storage migration helpers
- `contracts/deprecation_framework` — legacy entrypoint lifecycle management

**Cryptographic & Privacy Contracts**
- `contracts/zk_verifier` — zero-knowledge proof verification; false proof acceptance is Critical
- `contracts/zkp_registry` — ZK proof registry and commitment storage
- `contracts/mpc_manager` — multi-party computation coordination
- `contracts/homomorphic_registry` — homomorphic encryption parameter coordination

**Cross-Chain Contracts**
- `contracts/cross_chain_access` — cross-chain access control delegation
- `contracts/cross_chain_identity` — identity bridging across chains
- `contracts/cross_chain_enhancements` — enhanced cross-chain primitives

**Healthcare Integration Contracts**
- `contracts/pharma_supply_chain` — pharmaceutical traceability; drug provenance tampering is High
- `contracts/emr_integration` — Electronic Medical Record system bridges
- `contracts/ihe_integration` — IHE/HL7 FHIR standards integration
- `contracts/provider_directory` — healthcare provider registry
- `contracts/regulatory_compliance` — on-chain compliance attestations
- `contracts/dispute_resolution` — arbitration for patient-provider disputes
- `contracts/public_health_surveillance` — aggregate public health data collection
- `contracts/iot_device_management` — medical IoT device authorization

**Security & Monitoring Contracts**
- `contracts/reentrancy_guard` — reentrancy protection primitives
- `contracts/runtime_validation` — runtime input validation enforcement
- `contracts/sanitization` — input sanitization helpers
- `contracts/anomaly_detection` — on-chain anomaly scoring
- `contracts/anomaly_detector` — real-time anomaly detection engine
- `contracts/audit_forensics` — immutable audit trail and forensic log anchoring
- `contracts/contract_monitoring` — contract health monitoring

**Utility Contracts**
- `contracts/notification_system`
- `contracts/storage_cleanup`
- `contracts/contract_verification`
- `contracts/contract_usage_analytics`
- `contracts/code_ownership`
- `contracts/common_error`
- `contracts/fp_math`
- `contracts/healthcare_data_conversion`
- `contracts/explainable_ai`

The shared library at `libs/access_utils` is also in scope. The `contract_optimizer` tooling is in scope if a vulnerability in it could cause it to generate insecure contract code.

### Also In Scope

- CI/CD configuration (`.github/workflows/`) — secrets exposure, supply-chain injection
- Deployment scripts in `scripts/` that handle private keys or secret material
- Dependencies declared in `Cargo.toml` files — known CVEs, malicious crate substitution

---

## 2. Severity Classification Matrix

We use a four-level severity classification aligned with industry standards (CVSS v3.1 base score ranges are provided as reference but do not override the contextual examples below).

### Critical (CVSS 9.0–10.0)

Impact: direct theft of funds, unauthorized PHI disclosure, complete contract takeover, or irreversible data corruption. Immediate incident response is triggered.

**Examples in this codebase:**
- Any path that allows a non-patient actor to read plaintext medical records from `medical_records` without a valid role grant
- Unauthorized minting or burning of `sut_token` by a non-admin address
- Bypassing the timelock delay in `timelock` to execute a governance proposal immediately
- Constructing a fake ZK proof that `zk_verifier` accepts as valid, granting unauthorized access
- Unauthorized fund withdrawal from `escrow` or `appointment_booking_escrow`
- Bypassing authorization in `upgrade_manager` to deploy a malicious contract upgrade
- Disabling encrypted-only mode in `medical_records` without the required multi-admin threshold in `crypto_registry`

### High (CVSS 7.0–8.9)

Impact: significant data exposure, privilege escalation to admin or operator, persistent denial of service, financial loss bounded to a subset of users or funds.

**Examples in this codebase:**
- Registering a DID in `identity_registry` on behalf of another address without their signature
- Vote manipulation in `governor` that allows an attacker to pass or block proposals fraudulently
- Tampering with provider reputation scores in `healthcare_reputation` to fraudulently elevate or destroy a provider's standing
- Compromising the cross-chain message authentication in `cross_chain_access` or `cross_chain_identity` to forge access grants from another chain
- Drug provenance record manipulation in `pharma_supply_chain` to inject counterfeit medication into the supply chain
- Permanent DoS of a patient's record by deleting or corrupting data in `medical_record_backup` with no recovery path

### Medium (CVSS 4.0–6.9)

Impact: limited data exposure, temporary denial of service, non-critical privilege escalation, or behavior that deviates from specification in a way a user could exploit.

**Examples in this codebase:**
- Metadata correlation attack in `medical_records` that leaks a patient's identity through public on-chain fields (addresses, timestamps, categories) — this is a known residual risk per the threat model, but novel or amplified vectors are Medium
- Unauthorized read of `audit_forensics` logs that are intended for restricted roles only
- Manipulating anomaly scores in `anomaly_detection` or `anomaly_detector` to suppress or trigger false alerts
- Missing access control check in `contract_usage_analytics` that leaks aggregated usage patterns
- Reentrancy in a non-fund-holding contract that could corrupt storage state

### Low (CVSS 0.1–3.9)

Impact: minor information disclosure, edge-case failures that do not affect assets or PHI, or quality issues that could become a higher-severity issue under specific circumstances.

**Examples in this codebase:**
- A panic in `fp_math` that could be triggered by passing boundary inputs, causing a contract call to fail
- Missing event emission in `notification_system` for an action that should be logged
- A deprecated entrypoint in `deprecation_framework` that does not emit the expected warning event
- Minor input validation gaps in `sanitization` or `runtime_validation` that do not lead to exploitable state

---

## 3. Response SLAs

These are our committed response timelines from the moment we send an initial acknowledgement to you. Timelines may vary for issues reported during major holidays; we will communicate any delays proactively.

| Severity | Initial Acknowledgement | Triage Completion | Patch Target | Public Disclosure |
|----------|------------------------|------------------|--------------|-------------------|
| Critical | **4 hours** | **24 hours** | **72 hours** | After patch deployment + 7-day soak period |
| High | **24 hours** | **48 hours** | **7 days** | After patch deployment + 14-day soak period |
| Medium | **48 hours** | **5 business days** | **30 days** | After patch deployment + 30-day soak period |
| Low | **5 business days** | **10 business days** | **90 days** | Coordinated with reporter |

**What "triage completion" means:** we have reproduced or confirmed the issue, assigned a severity level, and opened a private tracking issue internally.

**What "patch target" means:** a fix is committed to an internal branch and ready for deployment. Complex issues (e.g., those requiring multi-contract refactoring or a governance vote through `governor` + `timelock`) may require a timeline extension, which we will communicate to you.

For Critical issues affecting live mainnet deployments, we will post a public advisory on this repository within 24 hours of confirming the issue, even before a patch is ready, if user action (e.g., revoking access grants) is needed to mitigate risk.

---

## 4. How to Report a Vulnerability

**Do not open a public GitHub issue for security vulnerabilities.** Public disclosure of unpatched vulnerabilities puts patients, providers, and users at risk.

### Primary Contact

Send your report by email to:

**security@vitastellar.io**

This inbox is monitored by the core maintainer team. Emails sent to this address are treated as confidential.

### What to Include in Your Report

A complete report helps us triage and fix the issue faster. Please include:

1. **Description** — A clear summary of the vulnerability and what it allows an attacker to do.
2. **Affected components** — Which contract(s), functions, or scripts are affected.
3. **Reproduction steps** — Step-by-step instructions to reproduce the issue, including any required preconditions (e.g., specific on-chain state, required role, network).
4. **Proof of concept** — If possible, include a test case, transaction trace, or minimal example. For Soroban contracts, a Rust test that demonstrates the issue is ideal.
5. **Impact assessment** — Your view of the severity and which users or assets are affected.
6. **Suggested fix (optional)** — Any ideas you have about remediation.

### PGP Encryption (Recommended for Critical and High Severity)

If your report contains sensitive details (e.g., a working exploit or sample PHI), we strongly recommend encrypting your email using our public PGP key. The key is available at:

- **Key ID:** Published on the VitaStellar GitHub repository under `.github/security-pgp-key.asc`
- **Fingerprint:** Included in the key file and verified via our GitHub profile

If you do not have access to PGP tooling, you may send your initial contact unencrypted with a brief description and request a secure channel; we will respond with options.

### GitHub Private Security Advisory (Alternative)

As an alternative to email, you may use GitHub's built-in [Private Security Advisory](https://docs.github.com/en/code-security/security-advisories/guidance-on-reporting-and-writing/privately-reporting-a-security-vulnerability) feature. Navigate to the **Security** tab of this repository and select **Report a vulnerability**. This creates a private draft advisory visible only to you and the maintainer team.

---

## 5. Bug Bounty Program

VitaStellar operates a community bug bounty program for verified vulnerabilities in in-scope components. Rewards are paid in **SUT tokens** at the market value at time of payout.

| Severity | Reward Range |
|----------|-------------|
| Critical | 2,000 – 10,000 SUT |
| High | 500 – 2,000 SUT |
| Medium | 100 – 500 SUT |
| Low | 25 – 100 SUT |

Reward amounts within each range are determined by:
- Quality and completeness of the report
- Actual exploitability in a mainnet or testnet context
- Whether a working proof of concept was provided
- Novelty of the vulnerability class

### Eligibility

To be eligible for a bounty reward:
- The vulnerability must be in a component listed in the [In Scope](#in-scope) section.
- The report must be submitted through the official contact channels described in Section 4.
- The reporter must not have exploited or tested the vulnerability against live mainnet deployments in a way that affected real users or funds.
- The reporter must not disclose the vulnerability publicly before we have deployed a patch.
- The reporter must be the first to report the specific vulnerability (duplicates are not rewarded, but acknowledged).

Maintainers, core contributors, and individuals with direct write access to the repository are not eligible for bounty rewards for vulnerabilities they discover in the course of their normal work.

### Acknowledgement

With your permission, we will acknowledge your contribution in the release notes and in the `CHANGELOG.md` when the fix is deployed. If you prefer to remain anonymous, we will respect that preference.

---

## 6. Safe Harbor Statement

VitaStellar and its maintainers will not pursue legal action against security researchers who:

- Report discovered vulnerabilities through the channels described in Section 4 before public disclosure
- Make a good-faith effort to avoid accessing, modifying, or exfiltrating data belonging to real users
- Do not run tests against live mainnet deployments in a way that affects real patient data or funds — use the Stellar **testnet** (`Test SDF Network ; September 2015`) for active research
- Do not violate the privacy of patients, healthcare providers, or other users
- Do not extort the project or demand payment as a precondition for responsible disclosure

We consider responsible security research a valuable contribution to the ecosystem. Researchers acting in good faith under this policy will be treated as partners, not adversaries. We will work with you to understand and validate the issue, agree on a disclosure timeline, and credit your work publicly if desired.

This safe harbor applies solely to activities conducted under this policy and does not extend to illegal activity unrelated to security research.

---

## 7. Out-of-Scope Items

The following are **not** considered valid security reports:

- Vulnerabilities in Stellar's core protocol, Soroban runtime, or the Stellar Development Foundation's infrastructure (report those to SDF directly)
- Issues that only affect test contracts (`contracts/load_testing`, `contracts/contract_template`) and have no analogous risk in production contracts
- Clickjacking, CSRF, or UI redressing attacks on the documentation portal
- Denial of service via spamming the Stellar network with transactions that incur normal fees
- Reports generated solely by automated scanners with no manual triage or proof of impact
- Social engineering attacks against project maintainers
- Physical attacks
- Findings related to missing security headers on static documentation sites

---

## 8. Disclosure Policy

We follow a **coordinated vulnerability disclosure** model:

1. Reporter submits a report privately.
2. We acknowledge receipt within the SLA in Section 3.
3. We triage, confirm, and develop a fix privately.
4. We notify the reporter when a fix is ready for review.
5. The reporter reviews the fix and confirms it addresses their findings.
6. We deploy the fix to testnet, then mainnet.
7. After a soak period (see Section 3 by severity), we publish a security advisory via GitHub Security Advisories and update `CHANGELOG.md`.
8. The reporter is credited (or anonymized at their request).

If we cannot reproduce or confirm a reported issue within the triage SLA, we will communicate that outcome to the reporter and keep the report open for 14 additional days to allow for supplementary information.

If a reporter intends to present findings at a conference or publish a blog post, we ask for at least 30 days advance notice so we can coordinate a simultaneous disclosure.

---

## Additional Resources

- [`docs/THREAT_MODEL.md`](../docs/THREAT_MODEL.md) — Cryptographic threat model covering plaintext leakage, key compromise, admin misuse, quantum adversary, and HE/MPC threats
- [`docs/CRYPTOGRAPHIC_SECURITY_MODEL.md`](../docs/CRYPTOGRAPHIC_SECURITY_MODEL.md) — Detailed cryptographic assumptions and key management model
- [`docs/SECURITY_BEST_PRACTICES.md`](../docs/SECURITY_BEST_PRACTICES.md) — Developer guidelines for writing secure contracts
- [`docs/SECURITY_CHECKLIST.md`](../docs/SECURITY_CHECKLIST.md) — Pre-deployment security checklist
- [`docs/SECURITY_CONTROLS_MAPPING.md`](../docs/SECURITY_CONTROLS_MAPPING.md) — Mapping of security controls to compliance frameworks
- [`docs/RBAC.md`](../docs/RBAC.md) — Role-based access control model and role definitions
- [`docs/AUTH_PATTERNS.md`](../docs/AUTH_PATTERNS.md) — Authentication and authorization patterns used across contracts

---

*Last reviewed: July 2026. This policy is reviewed quarterly and after any Critical security incident.*
