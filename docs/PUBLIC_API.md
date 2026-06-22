# VitaStellar — Public Contract API

> **Single source of truth.** Generated from `/// ` doc comments and `pub fn` signatures in each contract's `src/lib.rs`.
> Do not edit manually — run `make docs` to regenerate.

- **Generated**: `2026-06-20T14:53:13.852Z`
- **Contracts**: 56

## Table of Contents

- [healthcare_reputation](#healthcare-reputation)
- [identity_registry](#identity-registry)
- [credential_registry](#credential-registry)
- [payment_router](#payment-router)
- [anomaly_detection](#anomaly-detection)
- [anomaly_detector](#anomaly-detector)
- [appointment_booking_escrow](#appointment-booking-escrow)
- [audit_forensics](#audit-forensics)
- [code_ownership](#code-ownership)
- [common_error](#common-error)
- [contract_monitoring](#contract-monitoring)
- [contract_template](#contract-template)
- [contract_usage_analytics](#contract-usage-analytics)
- [contract_verification](#contract-verification)
- [cross_chain_access](#cross-chain-access)
- [cross_chain_enhancements](#cross-chain-enhancements)
- [cross_chain_identity](#cross-chain-identity)
- [crypto_registry](#crypto-registry)
- [deprecation_framework](#deprecation-framework)
- [dispute_resolution](#dispute-resolution)
- [emr_integration](#emr-integration)
- [escrow](#escrow)
- [explainable_ai](#explainable-ai)
- [fido2_authenticator](#fido2-authenticator)
- [fp_math](#fp-math)
- [governor](#governor)
- [healthcare_data_conversion](#healthcare-data-conversion)
- [homomorphic_registry](#homomorphic-registry)
- [ihe_integration](#ihe-integration)
- [iot_device_management](#iot-device-management)
- [load_testing](#load-testing)
- [medical_consent_nft](#medical-consent-nft)
- [medical_record_backup](#medical-record-backup)
- [medical_record_search](#medical-record-search)
- [mpc_manager](#mpc-manager)
- [notification_system](#notification-system)
- [patient_risk_stratification](#patient-risk-stratification)
- [pharma_supply_chain](#pharma-supply-chain)
- [predictive_analytics](#predictive-analytics)
- [provider_directory](#provider-directory)
- [public_health_surveillance](#public-health-surveillance)
- [regulatory_compliance](#regulatory-compliance)
- [reputation](#reputation)
- [reputation_access_control](#reputation-access-control)
- [reputation_integration](#reputation-integration)
- [runtime_validation](#runtime-validation)
- [sanitization](#sanitization)
- [secure_enclave](#secure-enclave)
- [storage_cleanup](#storage-cleanup)
- [sut_token](#sut-token)
- [timelock](#timelock)
- [token_sale](#token-sale)
- [upgrade_manager](#upgrade-manager)
- [upgradeability](#upgradeability)
- [zk_verifier](#zk-verifier)
- [zkp_registry](#zkp-registry)

---

## healthcare_reputation

```rust
pub fn initialize(env: Env, admin: Address) -> Result<(), Error>
```

```rust
pub fn add_credential( env: Env, provider: Address, credential_id: BytesN<32>, credential_type: CredentialType, issuer: Address, issue_date: u64, expiration_date: u64, credential_hash: BytesN<32>, ) -> Result<(), Error>
```

```rust
pub fn verify_credential( env: Env, admin: Address, provider: Address, credential_id: BytesN<32>, verified: bool, ) -> Result<(), Error>
```

```rust
pub fn add_feedback( env: Env, provider: Address, patient: Address, rating: u32, comment: String, feedback_type: FeedbackType, ) -> Result<(), Error>
```

```rust
pub fn add_conduct_entry( env: Env, reporter: Address, provider: Address, conduct_type: ConductType, description: String, severity: u32, action_taken: String, ) -> Result<(), Error>
```

```rust
pub fn create_dispute( env: Env, challenger: Address, provider: Address, dispute_type: DisputeType, description: String, evidence: Vec<String>, ) -> Result<(), Error>
```

```rust
pub fn resolve_dispute( env: Env, admin: Address, dispute_id: BytesN<32>, approved: bool, resolution: String, ) -> Result<(), Error>
```

```rust
pub fn get_reputation_score(env: Env, provider: Address) -> Result<u32, Error>
```

```rust
pub fn get_reputation_components( env: Env, provider: Address, ) -> Result<ReputationComponents, Error>
```

```rust
pub fn check_reputation_threshold( env: Env, provider: Address, threshold: u32, ) -> Result<bool, Error>
```

```rust
pub fn get_provider_credentials( env: Env, provider: Address, ) -> Result<Vec<ProviderCredential>, Error>
```

```rust
pub fn get_provider_feedback( env: Env, provider: Address, ) -> Result<Vec<PatientFeedback>, Error>
```

```rust
pub fn check_expired_credentials( env: Env, provider: Address, ) -> Result<Vec<BytesN<32>>, Error>
```

---

## identity_registry

> Initialize the contract with an owner and network identifier

```rust
pub fn initialize( env: Env, owner: Address, network_id: String, rbac_contract: Address, ) -> Result<(), Error>
```

> Perform a health check on the contract.
> Returns (status, version, timestamp) with standardized status values:
> "OK", "PAUSED", "NOT_INIT", "DEGRADED".

```rust
pub fn health_check(env: Env) -> (Symbol, u32, u64)
```

> Returns true if the contract is currently paused.

```rust
pub fn is_paused(env: Env) -> bool
```

```rust
pub fn pause(env: Env, caller: Address) -> Result<bool, Error>
```

```rust
pub fn unpause(env: Env, caller: Address) -> Result<bool, Error>
```

```rust
pub fn initialize_legacy(env: Env, owner: Address, rbac_contract: Address)
```

> Create a new DID Document for a subject
> Only the subject can create their own DID

```rust
pub fn create_did( env: Env, subject: Address, primary_public_key: BytesN<32>, services: Vec<ServiceEndpoint>, ) -> Result<String, Error>
```

> Resolve a DID Document by subject address

```rust
pub fn resolve_did(env: Env, subject: Address) -> Result<DIDDocument, Error>
```

> Resolve a DID Document by DID string

```rust
pub fn resolve_did_by_string(env: Env, did_string: String) -> Result<DIDDocument, Error>
```

> Update DID Document (add/modify services, also_known_as)

```rust
pub fn update_did( env: Env, subject: Address, new_services: Vec<ServiceEndpoint>, new_also_known_as: Vec<String>, ) -> Result<(), Error>
```

> Deactivate a DID (soft delete)

```rust
pub fn deactivate_did(env: Env, subject: Address) -> Result<(), Error>
```

> Add a new verification method to a DID

```rust
pub fn add_verification_method( env: Env, subject: Address, method_id: String, method_type: VerificationMethodType, public_key: BytesN<32>, relationships: Vec<VerificationRelationship>, ) -> Result<(), Error>
```

> Rotate a verification method key

```rust
pub fn rotate_key( env: Env, subject: Address, method_id: String, new_public_key: BytesN<32>, ) -> Result<(), Error>
```

> Revoke/deactivate a verification method

```rust
pub fn revoke_verification_method( env: Env, subject: Address, method_id: String, ) -> Result<(), Error>
```

> Issue a verifiable credential (only verifiers/issuers can do this)

```rust
pub fn issue_credential( env: Env, issuer: Address, subject: Address, credential_type: CredentialType, credential_hash: BytesN<32>, credential_uri: String, expiration_date: u64, ) -> Result<BytesN<32>, Error>
```

> Verify a credential's status

```rust
pub fn verify_credential( env: Env, credential_id: BytesN<32>, ) -> Result<CredentialStatus, Error>
```

> Get a credential by ID

```rust
pub fn get_credential( env: Env, credential_id: BytesN<32>, ) -> Result<VerifiableCredential, Error>
```

> Revoke a credential (only issuer can revoke)

```rust
pub fn revoke_credential( env: Env, issuer: Address, credential_id: BytesN<32>, reason: String, ) -> Result<(), Error>
```

> Get all credentials for a subject

```rust
pub fn get_subject_credentials(env: Env, subject: Address) -> Vec<VerifiableCredential>
```

> Verify if subject has a valid credential of a specific type

```rust
pub fn has_valid_credential( env: Env, subject: Address, credential_type: CredentialType, ) -> bool
```

> Add a recovery guardian

```rust
pub fn add_recovery_guardian( env: Env, subject: Address, guardian: Address, weight: u32, ) -> Result<(), Error>
```

> Remove a recovery guardian

```rust
pub fn remove_recovery_guardian( env: Env, subject: Address, guardian: Address, ) -> Result<(), Error>
```

> Set recovery threshold

```rust
pub fn set_recovery_threshold(env: Env, subject: Address, threshold: u32) -> Result<(), Error>
```

> Initiate identity recovery

```rust
pub fn initiate_recovery( env: Env, guardian: Address, subject: Address, new_controller: Address, new_primary_key: BytesN<32>, ) -> Result<u64, Error>
```

> Approve a recovery request

```rust
pub fn approve_recovery(env: Env, guardian: Address, request_id: u64) -> Result<(), Error>
```

> Execute recovery after timelock and threshold met

```rust
pub fn execute_recovery(env: Env, request_id: u64) -> Result<(), Error>
```

> Cancel a recovery request (only subject with existing key)

```rust
pub fn cancel_recovery(env: Env, subject: Address) -> Result<(), Error>
```

> Add a service endpoint to DID

```rust
pub fn add_service( env: Env, subject: Address, service_id: String, service_type: String, endpoint: String, ) -> Result<(), Error>
```

> Remove/deactivate a service endpoint

```rust
pub fn remove_service(env: Env, subject: Address, service_id: String) -> Result<(), Error>
```

> Add a verifier (only owner can do this).
> 
> SECURITY (issue #43): the previously-blind `assign_role(Staff)` call has
> been guarded so that a verifier who already holds a higher-privileged
> role (Admin, Doctor, Researcher) keeps that role untouched. Only
> verifiers without any of those higher roles receive the `Staff` marker.
> Either way, the local `Verifier(addr) -> true` flag is set so the
> contract-level verifier registry stays consistent.
> 
> Trade-off (intentional): if a verifier was originally added while only
> holding `Staff` and is later promoted to a higher role (Admin/Doctor
> /Researcher) without an intervening `remove_verifier`, the `Staff`
> row will remain in RBAC. `remove_verifier` will then leave it alone
> because of the higher-role guard. Operators that need the row
> removed should call `remove_verifier` before the promotion.

```rust
pub fn add_verifier(env: Env, verifier: Address) -> Result<(), Error>
```

> Remove a verifier (only owner can do this).
> 
> SECURITY (issue #43): as with `add_verifier`, the `remove_role(Staff)`
> call is now skipped whenever the target already holds a
> higher-privileged role (Admin, Doctor, Researcher). Stripping `Staff`
> from those users could be misinterpreted as a privilege revocation
> and risks disturbing the higher-privileged role state, so the call
> is intentionally a no-op in that case. The local `Verifier(addr)`
> flag is always cleared.
> 
> Trade-off (intentional, mirrors `add_verifier`): if a verifier was
> originally added while only holding `Staff` and was later promoted to
> a higher role, the pre-existing `Staff` row is preserved by this
> function alongside the higher role. To clear `Staff` from such an
> address, demote it back to non-staff roles first.

```rust
pub fn remove_verifier(env: Env, verifier: Address) -> Result<(), Error>
```

> Check if an address is a verifier

```rust
pub fn is_verifier(env: Env, account: Address) -> bool
```

> Get the contract owner

```rust
pub fn get_owner(env: Env) -> Result<Address, Error>
```

> Register an identity hash with metadata (legacy support)

```rust
pub fn register_identity_hash( env: Env, hash: BytesN<32>, subject: Address, meta: String, ) -> Result<(), Error>
```

> Create an attestation (legacy - only verifiers can do this)

```rust
pub fn attest( env: Env, verifier: Address, subject: Address, claim_hash: BytesN<32>, ) -> Result<(), Error>
```

> Revoke an attestation (legacy)

```rust
pub fn revoke_attestation( env: Env, verifier: Address, subject: Address, claim_hash: BytesN<32>, ) -> Result<(), Error>
```

> Get identity hash for a subject (legacy)

```rust
pub fn get_identity_hash(env: Env, subject: Address) -> Option<BytesN<32>>
```

> Get identity metadata for a subject (legacy)

```rust
pub fn get_identity_meta(env: Env, subject: Address) -> Option<String>
```

> Check if a specific attestation is active (legacy)

```rust
pub fn is_attested(env: Env, subject: Address, claim_hash: BytesN<32>) -> bool
```

> Get all active attestations for a subject (legacy)

```rust
pub fn get_attestations(env: Env, subject: Address) -> Vec<BytesN<32>>
```

> DID-based authorization check

```rust
pub fn verify_did_authorization( env: Env, subject: Address, required_relationship: VerificationRelationship, ) -> bool
```

> Registers a FIDO2 / WebAuthn authenticator device as a verification method
> in the subject's DID document.
> 
> Called by the `fido2_authenticator` contract after a successful device
> registration ceremony.  The public key is stored as a SHA-256 hash
> (`public_key_hash`) because DID verification methods use 32-byte keys and
> FIDO2 P-256 keys are 65 bytes; the hash acts as a stable, compact identifier.
> 
> # Arguments
> * `subject`          — DID owner; must have an active DID document.
> * `device_name`      — friendly name used as the verification method fragment ID.
> * `algorithm_tag`    — 1 = EdDSA (Ed25519), 2 = ES256 (P-256).
> * `public_key_hash`  — SHA-256 of the raw authenticator public key bytes.
> 
> If the subject has no DID document the call is silently ignored so that
> the `fido2_authenticator` registration is never blocked by DID state.

```rust
pub fn add_fido2_device( env: Env, subject: Address, device_name: String, algorithm_tag: u32, public_key_hash: BytesN<32>, ) -> Result<(), Error>
```

> Deposit stake for a healthcare provider.

```rust
pub fn deposit_stake( env: Env, provider: Address, amount: i128, token_address: Address, ) -> Result<(), Error>
```

> Withdraw stake after lock period if not slashed and in good standing.

```rust
pub fn withdraw_stake(env: Env, provider: Address) -> Result<i128, Error>
```

> Slash stake for verified misconduct (governance only).

```rust
pub fn slash_stake( env: Env, governance: Address, provider: Address, amount: i128, reason: String, ) -> Result<(), Error>
```

```rust
pub fn has_role(env: Env, address: Address, role: RbacRole) -> Result<bool, RbacError>
```

```rust
pub fn assign_role(env: Env, address: Address, role: RbacRole) -> Result<bool, RbacError>
```

```rust
pub fn remove_role(env: Env, address: Address, role: RbacRole) -> Result<bool, RbacError>
```

---

## credential_registry

```rust
pub fn initialize(env: Env, admin: Address) -> Result<(), Error>
```

```rust
pub fn set_issuer_admin( env: Env, caller: Address, issuer: Address, issuer_admin: Address, ) -> Result<bool, Error>
```

```rust
pub fn get_issuer_admin(env: Env, issuer: Address) -> Option<Address>
```

```rust
pub fn set_credential_root( env: Env, caller: Address, issuer: Address, root: BytesN<32>, metadata_hash: BytesN<32>, expiry: u64, signature: BytesN<64>, ) -> Result<u32, Error>
```

```rust
pub fn revoke_root( env: Env, caller: Address, issuer: Address, version: u32, ) -> Result<bool, Error>
```

```rust
pub fn set_revocation_root( env: Env, caller: Address, issuer: Address, revocation_root: BytesN<32>, ) -> Result<bool, Error>
```

```rust
pub fn get_active_root(env: Env, issuer: Address) -> Option<BytesN<32>>
```

```rust
pub fn get_active_version(env: Env, issuer: Address) -> u32
```

```rust
pub fn get_root(env: Env, issuer: Address, version: u32) -> Option<CredentialRootRecord>
```

```rust
pub fn get_revocation_root(env: Env, issuer: Address) -> Option<BytesN<32>>
```

```rust
pub fn is_root_revoked(env: Env, issuer: Address, root: BytesN<32>) -> bool
```

```rust
pub fn batch_set_credential_roots( env: Env, caller: Address, issuer: Address, roots: soroban_sdk::Vec<BytesN<32>>, metadata_hashes: soroban_sdk::Vec<BytesN<32>>, expiries: soroban_sdk::Vec<u64>, signatures: soroban_sdk::Vec<BytesN<64>>, ) -> Result<soroban_sdk::Vec<u32>, Error>
```

```rust
pub fn has_active_root(env: Env, issuer: Address) -> bool
```

---

## payment_router

```rust
pub fn set_fee_config( env: Env, fee_receiver: Address, platform_fee_bps: u32, ) -> Result<(), Error>
```

```rust
pub fn get_fee_config(env: Env) -> Option<RouterFeeConfig>
```

```rust
pub fn compute_split(env: Env, amount: i128) -> Result<(i128, i128), Error>
```

---

## anomaly_detection

```rust
pub fn initialize(env: Env, admin: Address, detector: Address, threshold_bps: u32) -> bool
```

```rust
pub fn update_config( env: Env, caller: Address, new_detector: Option<Address>, new_threshold: Option<u32>, new_sensitivity: Option<u32>, enabled: Option<bool>, ) -> Result<bool, Error>
```

```rust
pub fn set_audit_forensics( env: Env, admin: Address, forensics: Address, ) -> Result<bool, Error>
```

```rust
pub fn detect_anomaly( env: Env, caller: Address, record_id: u64, patient: Address, score_bps: u32, severity: u32, metadata: String, explanation_ref: String, ) -> Result<u64, Error>
```

```rust
pub fn get_anomaly_record(env: Env, anomaly_id: u64) -> Option<AnomalyRecord>
```

```rust
pub fn get_config(env: Env) -> Option<AnomalyDetectionConfig>
```

```rust
pub fn get_stats(env: Env) -> DetectionStats
```

```rust
pub fn get_anomaly_count_for_patient(env: Env, patient: Address) -> u64
```

```rust
pub fn whitelist_detector( env: Env, caller: Address, detector_addr: Address, ) -> Result<bool, Error>
```

```rust
pub fn is_whitelisted_detector(env: Env, detector_addr: Address) -> bool
```

> Promote an anomaly record to an active alert for investigation tracking.

```rust
pub fn create_alert(env: Env, caller: Address, anomaly_id: u64) -> Result<u64, Error>
```

> Acknowledge an alert (marks it as under review).

```rust
pub fn acknowledge_alert(env: Env, caller: Address, alert_id: u64) -> Result<bool, Error>
```

> Resolve an alert after investigation.

```rust
pub fn resolve_alert( env: Env, caller: Address, alert_id: u64, notes: String, ) -> Result<bool, Error>
```

> Mark alert as false positive. Feeds adaptive threshold learning.

```rust
pub fn mark_false_positive(env: Env, caller: Address, alert_id: u64) -> Result<bool, Error>
```

> Submit feedback on a detection. Adaptive threshold learning:
> - `confirmed = true`  → lower threshold by 50 bps (catch more)
> - `confirmed = false` → raise threshold by 50 bps (reduce noise)

```rust
pub fn submit_feedback( env: Env, caller: Address, anomaly_id: u64, confirmed: bool, ) -> Result<bool, Error>
```

```rust
pub fn get_alert(env: Env, alert_id: u64) -> Option<AnomalyAlert>
```

```rust
pub fn get_alert_count(env: Env) -> u64
```

---

## anomaly_detector

```rust
pub fn initialize(env: Env, admin: Address) -> Result<bool, Error>
```

```rust
pub fn add_validator(env: Env, caller: Address, validator: Address) -> Result<bool, Error>
```

```rust
pub fn remove_validator(env: Env, caller: Address, validator: Address) -> Result<bool, Error>
```

```rust
pub fn pause(env: Env, caller: Address) -> Result<bool, Error>
```

```rust
pub fn unpause(env: Env, caller: Address) -> Result<bool, Error>
```

> Update the anomaly detection threshold for a model (admin only).
> `threshold_bps` must be in range 1–9999 (basis points).

```rust
pub fn update_threshold( env: Env, caller: Address, model_id: BytesN<32>, threshold_bps: u32, ) -> Result<bool, Error>
```

> Clear active alerts up to `count` (admin only). Pass 0 to clear all.
> Marks each active alert as Resolved and emits a ClearAlerts event.

```rust
pub fn clear_alerts(env: Env, caller: Address, count: u64) -> Result<u64, Error>
```

> Register an ML model with its initial feature weights.
> `weights` must have exactly `feature_count` elements, each 0-10000 bps.

```rust
pub fn register_model( env: Env, caller: Address, model_id: BytesN<32>, name: String, feature_count: u32, weights: Vec<u32>, threshold_bps: u32, ) -> Result<bool, Error>
```

> Adjust a single feature weight (used by adaptive learning pipeline).
> `increase = true` adds `delta`; `increase = false` subtracts.

```rust
pub fn update_model_weight( env: Env, caller: Address, model_id: BytesN<32>, feature_index: u32, delta: u32, increase: bool, ) -> Result<bool, Error>
```

> Run on-chain ML inference over a feature vector.
> Score = weighted average of normalized features (0-10000 bps).
> Returns explainability-ready `DetectionResult`.

```rust
pub fn run_inference( env: Env, caller: Address, patient: Address, model_id: BytesN<32>, features: Vec<u32>, feature_names: Vec<String>, metadata: String, ) -> Result<DetectionResult, Error>
```

> Detect prescription anomaly patterns.
> 
> Scoring (weighted average, threshold = 5000 bps):
> - `high_risk_ratio` (40%): high_risk_count / drug_count
> - `drug_rate_score` (35%): prescriptions per hour, normalized
> - `pharmacy_dispersion` (25%): distinct pharmacy count, normalized

```rust
pub fn detect_prescription_anomaly( env: Env, caller: Address, patient: Address, drug_count: u32, high_risk_count: u32, unique_pharmacies: u32, time_window_hours: u32, metadata: String, ) -> Result<DetectionResult, Error>
```

> Detect access behavior anomalies.
> 
> Scoring (threshold = 5000 bps):
> - `access_count` (45%): absolute access count (30+ → max score)
> - `after_hours` (35%): 8000 bps if is_after_hours, else 0
> - `record_type_diversity` (20%): distinct record types accessed

```rust
pub fn detect_access_anomaly( env: Env, caller: Address, patient: Address, access_count: u32, time_window_secs: u32, is_after_hours: bool, distinct_record_types: u32, metadata: String, ) -> Result<DetectionResult, Error>
```

> Create a real-time alert from a `DetectionResult`. Returns the new alert_id.

```rust
pub fn create_alert( env: Env, caller: Address, patient: Address, model_id: BytesN<32>, result: DetectionResult, metadata: String, ) -> Result<u64, Error>
```

> Acknowledge an active alert (marks as reviewed, does not close).

```rust
pub fn acknowledge_alert(env: Env, caller: Address, alert_id: u64) -> Result<bool, Error>
```

> Resolve an alert after investigation. Accepted from Active or Acknowledged state.

```rust
pub fn resolve_alert( env: Env, caller: Address, alert_id: u64, resolution_notes: String, ) -> Result<bool, Error>
```

> Mark an alert as false positive, automatically feeding adaptive learning.

```rust
pub fn mark_false_positive(env: Env, caller: Address, alert_id: u64) -> Result<bool, Error>
```

> Submit feedback confirming or refuting an alert.
> 
> - `confirmed = true`: real anomaly → lower model threshold by LEARNING_RATE (more sensitive)
> - `confirmed = false`: false positive → raise threshold by LEARNING_RATE (less noisy)
> 
> Learning rate: 50 bps (0.5%) per feedback signal.

```rust
pub fn submit_feedback( env: Env, caller: Address, alert_id: u64, model_id: BytesN<32>, confirmed: bool, ) -> Result<u64, Error>
```

> Submit a privacy-preserving model update for a federated learning round.
> The `update_hash` commits to gradient updates without exposing patient data.
> Duplicate submissions per (round_id, participant) are rejected.

```rust
pub fn submit_federated_update( env: Env, participant: Address, round_id: u64, update_hash: BytesN<32>, num_samples: u32, ) -> Result<bool, Error>
```

```rust
pub fn get_alert(env: Env, alert_id: u64) -> Option<Alert>
```

```rust
pub fn get_model(env: Env, model_id: BytesN<32>) -> Option<AnomalyModel>
```

```rust
pub fn get_model_weights(env: Env, model_id: BytesN<32>) -> Option<Vec<u32>>
```

```rust
pub fn get_patient_profile(env: Env, patient: Address) -> Option<PatientRiskProfile>
```

```rust
pub fn get_alert_count(env: Env) -> u64
```

```rust
pub fn get_feedback(env: Env, feedback_id: u64) -> Option<ModelFeedback>
```

```rust
pub fn get_federated_update( env: Env, round_id: u64, participant: Address, ) -> Option<FederatedUpdate>
```

```rust
pub fn is_paused(env: Env) -> bool
```

```rust
pub fn is_validator(env: Env, addr: Address) -> bool
```

---

## appointment_booking_escrow

> Initialize the contract with an admin and token address

```rust
pub fn initialize(env: Env, admin: Address, _token: Address) -> Result<(), Error>
```

> Book an appointment with payment locked in escrow
> Transfers `amount` from patient to contract and creates an appointment escrow

```rust
pub fn book_appointment( env: Env, patient: Address, provider: Address, amount: i128, token: Address, ) -> Result<u64, Error>
```

> Confirm appointment completion and release funds to provider
> Only the provider can confirm the appointment

```rust
pub fn confirm_appointment( env: Env, provider: Address, appointment_id: u64, ) -> Result<(), Error>
```

> Refund appointment if canceled
> Only the patient can request a refund
> Can only be done if appointment is still in Booked state (not Confirmed/Refunded)

```rust
pub fn refund_appointment( env: Env, patient: Address, appointment_id: u64, ) -> Result<(), Error>
```

> Mark an appointment as a no-show (provider only).
> Only callable by the appointment's provider. No funds are released.

```rust
pub fn mark_no_show(env: Env, provider: Address, appointment_id: u64) -> Result<(), Error>
```

> Send an appointment reminder (provider or admin only).
> Records the timestamp when the reminder was last sent.

```rust
pub fn send_reminder(env: Env, caller: Address, appointment_id: u64) -> Result<(), Error>
```

> Get appointment details

```rust
pub fn get_appointment(env: Env, appointment_id: u64) -> Option<AppointmentEscrow>
```

> Get all appointments for a patient

```rust
pub fn get_patient_appointments(env: Env, patient: Address) -> Vec<u64>
```

> Get all appointments for a provider

```rust
pub fn get_provider_appointments(env: Env, provider: Address) -> Vec<u64>
```

> Get appointment status

```rust
pub fn get_appointment_status( env: Env, appointment_id: u64, ) -> Result<AppointmentStatus, Error>
```

> Get escrow balance (should be equal to sum of all booked but not confirmed/refunded appointments)

```rust
pub fn get_escrow_balance(env: Env) -> i128
```

> Get the current admin

```rust
pub fn get_admin(env: Env) -> Result<Address, Error>
```

> Get comprehensive health check

```rust
pub fn health_check(env: Env) -> ContractHealth
```

> Set pause status (admin only)

```rust
pub fn set_paused(env: Env, admin: Address, paused: bool) -> Result<(), Error>
```

> Check if contract is paused

```rust
pub fn is_paused(env: Env) -> bool
```

---

## audit_forensics

```rust
pub fn initialize(env: Env, admin: Address)
```

```rust
pub fn configure_audit_rule( env: Env, admin: Address, name: String, applies_to_language: String, severity_bps: u32, pattern_ref: String, remediation: String, ) -> u64
```

```rust
pub fn log_event( env: Env, actor: Address, action: AuditAction, record_id: Option<u64>, details_hash: BytesN<32>, metadata: Map<String, String>, ) -> u64
```

```rust
pub fn run_automated_audit( env: Env, caller: Address, contract_hash: BytesN<32>, language: String, analysis_mode: String, rule_ids: Vec<u64>, ml_confidence_bps: u32, ) -> u64
```

```rust
pub fn record_formal_verification( env: Env, admin: Address, execution_id: u64, property_name: String, proved: bool, proof_ref: String, ) -> bool
```

```rust
pub fn get_execution(env: Env, execution_id: u64) -> Option<AnalysisExecution>
```

```rust
pub fn get_finding(env: Env, finding_id: u64) -> Option<VulnerabilityFinding>
```

```rust
pub fn get_findings_by_execution(env: Env, execution_id: u64) -> Vec<VulnerabilityFinding>
```

```rust
pub fn get_formal_verification( env: Env, execution_id: u64, ) -> Option<FormalVerificationSummary>
```

```rust
pub fn generate_remediation_plan(env: Env, execution_id: u64) -> Vec<String>
```

```rust
pub fn analyze_timeline(env: Env, record_id: u64) -> Vec<AuditEntry>
```

```rust
pub fn investigate_user(env: Env, user: Address) -> Vec<AuditEntry>
```

```rust
pub fn generate_compliance_report( env: Env, start_time: u64, end_time: u64, ) -> Map<AuditAction, u32>
```

```rust
pub fn set_alert_threshold(env: Env, admin: Address, action: AuditAction, threshold: u32)
```

```rust
pub fn compress_logs(env: Env, admin: Address, before_timestamp: u64) -> BytesN<32>
```

```rust
pub fn archive_logs(env: Env, admin: Address, archive_ref: String)
```

```rust
pub fn sync_audit_cross_chain( env: Env, admin: Address, target_chain: String, audit_root: BytesN<32>, )
```

```rust
pub fn share_audit_with_regulator( env: Env, admin: Address, regulator: Address, filter_start: u64, filter_end: u64, proof_ref: String, )
```

---

## code_ownership

> Initialize the code ownership tracking system

```rust
pub fn initialize(env: Env, admin: Address) -> Result<(), Error>
```

> Register a module with ownership information

```rust
pub fn register_module( env: Env, admin: Address, module_id: String, module_name: String, primary_owner: Address, secondary_owners: Vec<Address>, expertise_areas: Vec<String>, ) -> Result<(), Error>
```

> Update module ownership

```rust
pub fn update_module_ownership( env: Env, admin: Address, module_id: String, new_primary_owner: Address, new_secondary_owners: Vec<Address>, ) -> Result<(), Error>
```

> Configure review routing for a module

```rust
pub fn configure_review_route( env: Env, admin: Address, module_id: String, required_reviewers: u32, escalation_threshold: u32, escalation_owner: Address, ) -> Result<(), Error>
```

> Get module ownership information

```rust
pub fn get_module_ownership(env: Env, module_id: String) -> Result<ModuleOwnership, Error>
```

> Get review routing for a module

```rust
pub fn get_review_route(env: Env, module_id: String) -> Result<ReviewRoute, Error>
```

> Get expertise matrix for all modules

```rust
pub fn get_expertise_matrix(env: Env) -> OwnershipMatrix
```

> Check if an address is an owner of a module

```rust
pub fn is_module_owner(env: Env, module_id: String, address: Address) -> Result<bool, Error>
```

> Get all modules owned by an address

```rust
pub fn get_owned_modules(env: Env, owner: Address) -> Vec<String>
```

---

## common_error

```rust
pub fn get_suggestion(error: CommonError) -> Symbol
```

---

## contract_monitoring

> Initialise the monitoring contract.

```rust
pub fn initialize( env: Env, admin: Address, alert_config: AlertConfig, ) -> Result<(), MonitoringError>
```

> Record a successful function call.
> 
> `caller` – the address that invoked the function.
> `function_name` – name of the function called.
> `gas_used` – estimated gas consumed (pass 0 if unknown).

```rust
pub fn record_call( env: Env, caller: Address, function_name: String, gas_used: u64, ) -> Result<(), MonitoringError>
```

> Record a failed function call / error.

```rust
pub fn record_error(env: Env, function_name: String) -> Result<(), MonitoringError>
```

> Update storage-entry count (call after writes to tracked contracts).

```rust
pub fn update_storage_count(env: Env, count: u32) -> Result<(), MonitoringError>
```

> Update alert thresholds (admin only).

```rust
pub fn update_alert_config(env: Env, config: AlertConfig) -> Result<(), MonitoringError>
```

> Return a full dashboard snapshot.

```rust
pub fn get_dashboard(env: Env) -> Result<DashboardSnapshot, MonitoringError>
```

> Return per-function statistics.

```rust
pub fn get_function_stats( env: Env, function_name: String, ) -> Result<FunctionStats, MonitoringError>
```

---

## contract_template

> Initialize the contract. Can only be called once.
> 
> # Auth
> No auth required — the deployer becomes the admin.

```rust
pub fn initialize(env: Env, admin: Address) -> Result<(), Error>
```

> Transfer admin rights to a new address.
> 
> # Auth
> Requires auth from the **current** admin.

```rust
pub fn transfer_admin(env: Env, new_admin: Address) -> Result<(), Error>
```

> Update the contract's stored data.
> 
> # Auth
> Requires auth from the admin.

```rust
pub fn update_data(env: Env, caller: Address, data: String) -> Result<(), Error>
```

> Return the current admin address.

```rust
pub fn get_admin(env: &Env) -> Result<Address, Error>
```

> Return the stored data, if any.

```rust
pub fn get_data(env: Env) -> Option<ContractData>
```

---

## contract_usage_analytics

```rust
pub fn initialize(env: Env, admin: Address) -> Result<(), Error>
```

```rust
pub fn record_call( env: Env, function_name: String, user: Address, cpu_usage: u64, ram_usage: u64, success: bool, latency_ms: u64, ) -> Result<(), Error>
```

```rust
pub fn take_snapshot(env: Env) -> Result<UsageSnapshot, Error>
```

```rust
pub fn get_function_metrics(env: Env, function_name: String) -> Option<FunctionMetric>
```

```rust
pub fn get_user_metrics(env: Env, user: Address) -> Option<UserMetric>
```

```rust
pub fn get_all_functions(env: Env) -> Vec<String>
```

```rust
pub fn get_snapshots(env: Env) -> Vec<UsageSnapshot>
```

---

## contract_verification

> Initialise the verification registry with an admin address.

```rust
pub fn initialize(env: Env, admin: Address) -> Result<(), VerificationError>
```

> Publish contract metadata.  Must be called by the admin.
> 
> Emits a `(VERIFY, META)` event that block explorers can index.

```rust
pub fn publish_metadata( env: Env, name: String, version: String, source_url: String, license: String, description: String, ) -> Result<(), VerificationError>
```

> Publish build reproducibility information.

```rust
pub fn publish_build_info( env: Env, rust_version: String, sdk_version: String, build_profile: String, wasm_hash: BytesN<32>, commit_sha: String, ) -> Result<(), VerificationError>
```

> Publish the ABI for all public functions.

```rust
pub fn publish_abi(env: Env, entries: Vec<AbiEntry>) -> Result<(), VerificationError>
```

> Mark the contract as fully verified (metadata + build + ABI all present).

```rust
pub fn mark_verified(env: Env) -> Result<(), VerificationError>
```

```rust
pub fn get_metadata(env: Env) -> Result<ContractMetadata, VerificationError>
```

```rust
pub fn get_build_info(env: Env) -> Result<BuildInfo, VerificationError>
```

```rust
pub fn get_abi(env: Env) -> Result<Vec<AbiEntry>, VerificationError>
```

```rust
pub fn is_verified(env: Env) -> bool
```

---

## cross_chain_access

```rust
pub fn initialize( env: Env, admin: Address, bridge_contract: Address, identity_contract: Address, ) -> Result<bool, Error>
```

```rust
pub fn grant_access( env: Env, grantor: Address, grantee_chain: ChainId, grantee_address: String, permission_level: PermissionLevel, record_scope: AccessScope, duration: u64, conditions: Vec<AccessCondition>, ) -> Result<u64, Error>
```

```rust
pub fn revoke_access(env: Env, caller: Address, grant_id: u64) -> Result<bool, Error>
```

```rust
pub fn update_grant_conditions( env: Env, caller: Address, grant_id: u64, new_conditions: Vec<AccessCondition>, ) -> Result<bool, Error>
```

```rust
pub fn extend_grant( env: Env, caller: Address, grant_id: u64, additional_duration: u64, ) -> Result<bool, Error>
```

```rust
pub fn request_access( env: Env, requester_chain: ChainId, requester_address: String, patient: Address, requested_records: Vec<u64>, purpose: String, is_emergency: bool, ) -> Result<u64, Error>
```

```rust
pub fn process_request( env: Env, caller: Address, request_id: u64, approve: bool, ) -> Result<bool, Error>
```

> Create access management delegation
> BUG FIX: Each (delegator, delegate) pair is stored under a unique key

```rust
pub fn create_delegation( env: Env, delegator: Address, delegate: Address, delegate_chain: ChainId, delegate_address: String, can_grant: bool, can_revoke: bool, can_manage_emergency: bool, duration: u64, ) -> Result<bool, Error>
```

```rust
pub fn revoke_delegation( env: Env, delegator: Address, delegate: Address, ) -> Result<bool, Error>
```

> Configure emergency access settings per patient
> BUG FIX: Each patient's config stored under unique key — was "emerg_key"

```rust
pub fn configure_emergency( env: Env, patient: Address, is_enabled: bool, auto_approve_duration: u64, required_attestations: u32, trusted_providers: Vec<String>, ) -> Result<bool, Error>
```

```rust
pub fn log_access( env: Env, accessor_chain: ChainId, accessor_address: String, patient: Address, record_id: u64, action: AccessAction, ip_hash: BytesN<32>, success: bool, ) -> Result<u64, Error>
```

> Propose an atomic access swap: offer a grant in exchange for cross-chain access

```rust
pub fn initiate_access_swap( env: Env, initiator: Address, counterpart_chain: ChainId, counterpart_address: String, offered_grant_id: u64, requested_permission: PermissionLevel, requested_scope: AccessScope, hash_lock: BytesN<32>, timelock_duration: u64, ) -> Result<u64, Error>
```

> Accept a swap proposal: counterpart provides a grant in return

```rust
pub fn accept_access_swap( env: Env, acceptor: Address, swap_id: u64, offered_grant_id: u64, // Grant the counterpart is offering in return ) -> Result<bool, Error>
```

> Finalize an accepted swap: atomically activates both sides of the exchange

```rust
pub fn finalize_access_swap( env: Env, caller: Address, swap_id: u64, secret: BytesN<32>, // Pre-image of hash_lock ) -> Result<bool, Error>
```

> Cancel a proposed swap (only initiator or after timelock expiry)

```rust
pub fn cancel_access_swap(env: Env, caller: Address, swap_id: u64) -> Result<bool, Error>
```

```rust
pub fn verify_access( env: Env, accessor_chain: ChainId, accessor_address: String, patient: Address, record_id: u64, required_permission: PermissionLevel, ) -> bool
```

```rust
pub fn get_grant(env: Env, grant_id: u64) -> Option<AccessGrant>
```

```rust
pub fn get_request(env: Env, request_id: u64) -> Option<AccessRequest>
```

```rust
pub fn get_delegation(env: Env, delegator: Address, delegate: Address) -> Option<Delegation>
```

```rust
pub fn get_emergency_config(env: Env, patient: Address) -> Option<EmergencyConfig>
```

```rust
pub fn get_audit_entry(env: Env, entry_id: u64) -> Option<AuditEntry>
```

```rust
pub fn get_swap(env: Env, swap_id: u64) -> Option<SwapProposal>
```

```rust
pub fn is_paused(env: Env) -> bool
```

```rust
pub fn pause(env: Env, caller: Address) -> Result<bool, Error>
```

```rust
pub fn unpause(env: Env, caller: Address) -> Result<bool, Error>
```

---

## cross_chain_enhancements

```rust
pub fn initialize(env: Env, admin: Address) -> Result<(), Error>
```

> Submit a zero-knowledge proof of data ownership
> This proves ownership of a medical record without revealing its contents

```rust
pub fn submit_zk_ownership_proof( env: Env, prover: Address, record_id: u64, chain: ChainId, proof_data: BytesN<64>, statement_hash: BytesN<32>, ) -> Result<BytesN<32>, Error>
```

> Verify a zero-knowledge ownership proof

```rust
pub fn verify_zk_ownership_proof( env: Env, verifier: Address, proof_id: BytesN<32>, ) -> Result<bool, Error>
```

> Create a data integrity proof using Merkle tree

```rust
pub fn create_data_integrity_proof( env: Env, caller: Address, data_hash: BytesN<32>, merkle_root: BytesN<32>, merkle_path: Vec<BytesN<32>>, leaf_index: u32, chain_id: ChainId, ) -> Result<BytesN<32>, Error>
```

> Check for replay attacks by tracking seen messages

```rust
pub fn check_replay_protection( env: Env, message_hash: BytesN<32>, source_chain: ChainId, ) -> Result<bool, Error>
```

> Set rate limit for an address

```rust
pub fn set_rate_limit( env: Env, admin: Address, address: Address, daily_limit: u64, ) -> Result<(), Error>
```

> Check and update rate limit for an operation

```rust
pub fn check_rate_limit(env: Env, caller: Address, amount: u64) -> Result<bool, Error>
```

> Get ZK proof status

```rust
pub fn get_zk_proof(env: Env, proof_id: BytesN<32>) -> Option<ZKOwnershipProof>
```

> Get integrity proof

```rust
pub fn get_integrity_proof(env: Env, proof_id: BytesN<32>) -> Option<ZKDataIntegrityProof>
```

---

## cross_chain_identity

```rust
pub fn initialize(env: Env, admin: Address, bridge_contract: Address) -> Result<bool, Error>
```

```rust
pub fn add_validator( env: Env, caller: Address, validator_address: Address, name: String, public_key: BytesN<32>, ) -> Result<bool, Error>
```

```rust
pub fn deactivate_validator( env: Env, caller: Address, validator_address: Address, ) -> Result<bool, Error>
```

```rust
pub fn update_trust_score( env: Env, caller: Address, validator_address: Address, trust_score: u32, ) -> Result<bool, Error>
```

```rust
pub fn set_min_attestations( env: Env, caller: Address, min_attestations: u32, ) -> Result<bool, Error>
```

```rust
pub fn pause(env: Env, caller: Address) -> Result<bool, Error>
```

```rust
pub fn unpause(env: Env, caller: Address) -> Result<bool, Error>
```

```rust
pub fn request_verification( env: Env, stellar_address: Address, external_chain: ChainId, external_address: String, proof: BytesN<64>, ) -> Result<u64, Error>
```

> Validator attests to a verification request
> BUG FIX: each attestation stored per (request_id, validator) — was "att_key"

```rust
pub fn attest_verification( env: Env, validator: Address, request_id: u64, is_valid: bool, signature: BytesN<64>, ) -> Result<bool, Error>
```

```rust
pub fn revoke_identity( env: Env, caller: Address, stellar_address: Address, external_chain: ChainId, ) -> Result<bool, Error>
```

```rust
pub fn initiate_sync( env: Env, stellar_address: Address, source_chain: ChainId, dest_chain: ChainId, ) -> Result<u64, Error>
```

```rust
pub fn update_sync_status( env: Env, validator: Address, sync_id: u64, status: SyncStatus, proof: BytesN<32>, ) -> Result<bool, Error>
```

> Get identity by Stellar address and external chain
> BUG FIX: each (stellar_address, chain) has a unique storage entry

```rust
pub fn get_identity( env: Env, stellar_address: Address, external_chain: ChainId, ) -> Option<CrossChainIdentity>
```

```rust
pub fn verify_identity(env: Env, stellar_address: Address, external_chain: ChainId) -> bool
```

```rust
pub fn get_request(env: Env, request_id: u64) -> Option<VerificationRequest>
```

```rust
pub fn get_sync(env: Env, sync_id: u64) -> Option<IdentitySync>
```

```rust
pub fn get_validator(env: Env, validator_address: Address) -> Option<IdentityValidator>
```

```rust
pub fn get_attestation(env: Env, request_id: u64, validator: Address) -> Option<Attestation>
```

```rust
pub fn is_paused(env: Env) -> bool
```

---

## crypto_registry

> Initialize the registry with an admin address for policy upgrades.
> Key registration/rotation is always self-authorized by the account.

```rust
pub fn initialize(env: Env, admin: Address) -> Result<(), Error>
```

> Register (or rotate) the caller's key bundle.
> 
> Returns the newly assigned version.

```rust
pub fn register_key_bundle( env: Env, owner: Address, encryption_key: PublicKey, pq_encryption_key: PublicKey, has_pq_encryption_key: bool, signing_key: PublicKey, has_signing_key: bool, ) -> Result<u32, Error>
```

> Revoke a specific key bundle version.

```rust
pub fn revoke_key_bundle(env: Env, owner: Address, version: u32) -> Result<(), Error>
```

```rust
pub fn get_current_version(env: Env, owner: Address) -> Result<u32, Error>
```

```rust
pub fn get_current_key_bundle(env: Env, owner: Address) -> Result<Option<KeyBundle>, Error>
```

```rust
pub fn get_key_bundle( env: Env, owner: Address, version: u32, ) -> Result<Option<KeyBundle>, Error>
```

> Rotate a specific key bundle for an owner with automatic old-key invalidation.
> This implements the envelope encryption pattern: the new key bundle replaces
> the old one atomically, and the old KEK is revoked so it cannot be used for
> future encryption operations.

```rust
pub fn rotate_key( env: Env, owner: Address, new_encryption_key: PublicKey, new_pq_encryption_key: PublicKey, has_pq_encryption_key: bool, new_signing_key: PublicKey, has_signing_key: bool, ) -> Result<u32, Error>
```

> Get all key bundle versions for an owner (including revoked ones).

```rust
pub fn get_all_key_versions(env: Env, owner: Address) -> Result<Vec<u32>, Error>
```

---

## deprecation_framework

> Initialize the deprecation framework

```rust
pub fn initialize(env: Env, admin: Address) -> Result<(), Error>
```

> Mark a contract for deprecation

```rust
pub fn mark_for_deprecation( env: Env, admin: Address, contract_id: String, contract_name: String, reason: String, replacement_contract: Option<String>, ) -> Result<(), Error>
```

> Set sunset timeline for a contract

```rust
pub fn set_sunset_timeline( env: Env, admin: Address, contract_id: String, announcement_date: u64, support_end_date: u64, removal_date: u64, ) -> Result<(), Error>
```

> Add migration guidance

```rust
pub fn add_migration_guide( env: Env, admin: Address, contract_id: String, guide_title: String, guide_content: String, code_examples: Vec<String>, ) -> Result<(), Error>
```

> Update deprecation phase

```rust
pub fn update_deprecation_phase( env: Env, admin: Address, contract_id: String, new_phase: DeprecationPhase, ) -> Result<(), Error>
```

> Publish user communication

```rust
pub fn publish_user_communication( env: Env, admin: Address, contract_id: String, message: String, communication_type: String, // "email", "notification", "announcement" ) -> Result<u64, Error>
```

> Create removal checklist

```rust
pub fn create_removal_checklist( env: Env, admin: Address, contract_id: String, checklist_items: Vec<String>, ) -> Result<(), Error>
```

> Mark checklist item as complete

```rust
pub fn mark_checklist_item_complete( env: Env, admin: Address, contract_id: String, item_index: u32, ) -> Result<(), Error>
```

> Get deprecation status

```rust
pub fn get_deprecation_status( env: Env, contract_id: String, ) -> Result<DeprecationStatus, Error>
```

> Get sunset timeline

```rust
pub fn get_sunset_timeline(env: Env, contract_id: String) -> Result<SunsetTimeline, Error>
```

> Get migration guide

```rust
pub fn get_migration_guide(env: Env, contract_id: String) -> Result<MigrationGuide, Error>
```

> Check if contract is deprecated

```rust
pub fn is_deprecated(env: Env, contract_id: String) -> bool
```

---

## dispute_resolution

```rust
pub fn initialize(env: Env, arbiters: Vec<Address>)
```

```rust
pub fn dispute(env: Env, proposal_id: u64, challenger: Address)
```

```rust
pub fn resolve( env: Env, proposal_id: u64, arbiter: Address, valid_proposal: bool, ) -> Result<(), Error>
```

```rust
pub fn is_disputed(env: Env, proposal_id: u64) -> bool
```

---

## emr_integration

```rust
pub fn initialize(env: Env, admin: Address, fhir_contract: Address) -> Result<bool, Error>
```

```rust
pub fn register_emr_system( env: Env, admin: Address, system_id: String, vendor_name: String, vendor_contact: String, system_version: String, supported_standards: Vec<String>, api_endpoints: Vec<String>, ) -> Result<bool, Error>
```

```rust
pub fn get_emr_system(env: Env, system_id: String) -> Result<EMRSystem, Error>
```

```rust
pub fn initiate_onboarding( env: Env, provider: Address, onboarding_id: String, provider_id: String, provider_name: String, provider_email: String, facility_name: String, npi: String, emr_system_id: String, compliance_checklist: Vec<String>, ) -> Result<bool, Error>
```

```rust
pub fn complete_onboarding( env: Env, admin: Address, onboarding_id: String, verification_id: String, license_number: String, license_state: String, license_expiration: String, board_certifications: Vec<String>, malpractice_insurance: String, background_check_id: String, ) -> Result<bool, Error>
```

```rust
pub fn get_onboarding_status( env: Env, onboarding_id: String, ) -> Result<ProviderOnboarding, Error>
```

```rust
pub fn get_provider_verification( env: Env, verification_id: String, ) -> Result<ProviderVerification, Error>
```

```rust
pub fn register_network_node( env: Env, admin: Address, node: NetworkNode, ) -> Result<bool, Error>
```

```rust
pub fn get_network_node(env: Env, node_id: String) -> Result<NetworkNode, Error>
```

```rust
pub fn register_interop_agreement( env: Env, admin: Address, agreement: InteroperabilityAgreement, ) -> Result<bool, Error>
```

```rust
pub fn get_interop_agreement( env: Env, agreement_id: String, ) -> Result<InteroperabilityAgreement, Error>
```

```rust
pub fn record_interop_test( env: Env, tester: Address, test: InteroperabilityTest, ) -> Result<bool, Error>
```

```rust
pub fn get_interop_test(env: Env, test_id: String) -> Result<InteroperabilityTest, Error>
```

```rust
pub fn parse_message( env: Env, sender: Address, message_id: String, source_system_id: String, encoding: CharacterEncoding, transport: TransportProtocol, content_type: String, payload: String, ) -> Result<HealthcareMessage, Error>
```

```rust
pub fn generate_message( env: Env, sender: Address, message_id: String, source_system_id: String, standard: MessagingStandard, version: String, message_type: String, encoding: CharacterEncoding, transport: TransportProtocol, content_type: String, metadata: Map<String, String>, ) -> Result<HealthcareMessage, Error>
```

```rust
pub fn transform_message( env: Env, sender: Address, transform_id: String, source_message_id: String, target_message_id: String, target_standard: MessagingStandard, target_version: String, target_message_type: String, target_encoding: CharacterEncoding, target_transport: TransportProtocol, target_content_type: String, ) -> Result<MessageTransformation, Error>
```

```rust
pub fn validate_message( env: Env, sender: Address, report_id: String, message_id: String, ) -> Result<MessageValidationReport, Error>
```

```rust
pub fn wrap_transport_payload(env: Env, message_id: String) -> Result<String, Error>
```

```rust
pub fn benchmark_message_processing( env: Env, benchmark_id: String, message_type: String, encoding: CharacterEncoding, transport: TransportProtocol, batch_size: u32, ) -> Result<ThroughputBenchmark, Error>
```

```rust
pub fn get_message(env: Env, message_id: String) -> Result<HealthcareMessage, Error>
```

```rust
pub fn get_validation_report( env: Env, report_id: String, ) -> Result<MessageValidationReport, Error>
```

```rust
pub fn get_transformation( env: Env, transform_id: String, ) -> Result<MessageTransformation, Error>
```

```rust
pub fn get_supported_message_types(env: Env) -> Vec<String>
```

```rust
pub fn pause(env: Env, admin: Address) -> Result<bool, Error>
```

```rust
pub fn resume(env: Env, admin: Address) -> Result<bool, Error>
```

---

## escrow

```rust
pub fn initialize(env: Env, admin: Address) -> Result<(), Error>
```

```rust
pub fn set_fee_config( env: Env, caller: Address, fee_receiver: Address, platform_fee_bps: u32, ) -> Result<(), Error>
```

```rust
pub fn get_fee_config(env: Env) -> Option<FeeConfig>
```

```rust
pub fn create_escrow( env: Env, order_id: u64, payer: Address, payee: Address, amount: i128, token: Address, ) -> Result<bool, Error>
```

```rust
pub fn mark_disputed(env: Env, caller: Address, order_id: u64) -> Result<(), Error>
```

```rust
pub fn approve_release(env: Env, order_id: u64, approver: Address) -> Result<(), Error>
```

```rust
pub fn release_escrow(env: Env, order_id: u64) -> Result<bool, Error>
```

```rust
pub fn refund_escrow(env: Env, order_id: u64, reason: String) -> Result<bool, Error>
```

```rust
pub fn get_escrow(env: Env, order_id: u64) -> Option<Escrow>
```

```rust
pub fn get_credit(env: Env, addr: Address) -> i128
```

```rust
pub fn withdraw(env: Env, caller: Address, token: Address, to: Address) -> Result<i128, Error>
```

```rust
pub fn get_total_volume(env: Env) -> i128
```

```rust
pub fn get_total_escrows(env: Env) -> u64
```

```rust
pub fn get_settled_rate(env: Env) -> u32
```

```rust
pub fn get_refund_rate(env: Env) -> u32
```

```rust
pub fn get_dispute_rate(env: Env) -> u32
```

```rust
pub fn get_active_escrows_count(env: Env) -> u64
```

```rust
pub fn get_stats_summary(env: Env) -> PlatformStats
```

```rust
pub fn get_platform_health_score(env: Env) -> u32
```

```rust
pub fn get_token_volume(env: Env, _token: Address) -> i128
```

```rust
pub fn get_donor_reputation(env: Env, _donor: Address) -> u32
```

```rust
pub fn get_daily_stats(env: Env, day_id: u64) -> Option<DailyStats>
```

```rust
pub fn export_summary(env: Env, format: String) -> ExportMetadata
```

---

## explainable_ai

```rust
pub fn initialize(env: Env, admin: Address) -> bool
```

```rust
pub fn request_explanation(env: Env, caller: Address, ai_insight_id: u64) -> u64
```

```rust
pub fn fulfill_explanation_request( env: Env, caller: Address, request_id: u64, model_id: BytesN<32>, explanation_method: String, feature_importance: Vec<FeatureImportance>, primary_factors: Vec<String>, confidence_impact: u32, explanation_ref: String, ) -> Result<bool, Error>
```

```rust
pub fn get_explanation_request(env: Env, request_id: u64) -> Option<ExplanationRequest>
```

```rust
pub fn get_explanation(env: Env, explanation_id: u64) -> Option<ExplanationMetadata>
```

```rust
pub fn get_explanations_for_patient( env: Env, _patient: Address, _page: u32, _page_size: u32, ) -> Vec<ExplanationMetadata>
```

```rust
pub fn submit_bias_audit( env: Env, caller: Address, model_id: BytesN<32>, audit_summary: String, recommendations: Vec<String>, ) -> Result<u64, Error>
```

```rust
pub fn get_bias_audit(env: Env, model_id: BytesN<32>) -> Option<BiasAuditResult>
```

```rust
pub fn run_fairness_metrics( env: Env, caller: Address, _model_id: BytesN<32>, _protected_attribute: String, _privileged_group: String, _unprivileged_group: String, ) -> Result<(u32, u32, u32), Error>
```

> Generate SHAP explanation for a prediction

```rust
pub fn generate_shap_explanation( env: Env, caller: Address, model_id: BytesN<32>, prediction_id: u64, base_value: i128, prediction: i128, feature_names: Vec<String>, feature_values: Vec<i128>, method: String, ) -> Result<u64, Error>
```

> Get SHAP explanation by ID

```rust
pub fn get_shap_explanation(env: Env, shap_id: u64) -> Option<ShapExplanation>
```

> Generate counterfactual explanation

```rust
pub fn generate_counterfactual( env: Env, caller: Address, original_prediction: i128, target_prediction: i128, current_features: Vec<(String, i128)>, target_features: Vec<(String, i128)>, ) -> Result<u64, Error>
```

> Get counterfactual explanation by ID

```rust
pub fn get_counterfactual(env: Env, cf_id: u64) -> Option<CounterfactualExplanation>
```

---

## fido2_authenticator

> Initializes the contract.  Must be called exactly once.
> 
> * `admin`      — address authorised to call administrative functions.
> * `rp_id_hash` — SHA-256 of the relying party identifier string
>                  (e.g., `sha256(b"vitastellar.health")`).

```rust
pub fn initialize(env: Env, admin: Address, rp_id_hash: BytesN<32>) -> Result<(), Error>
```

> Configures the identity registry contract address.
> When set, `register_device` will bind new credentials to the caller's DID.

```rust
pub fn set_identity_registry( env: Env, caller: Address, contract_id: Address, ) -> Result<(), Error>
```

> Configures the ZK verifier contract used for ES256 (P-256) assertions.

```rust
pub fn set_zk_verifier(env: Env, caller: Address, contract_id: Address) -> Result<(), Error>
```

> Issues a registration challenge for `user`.
> 
> The 32-byte challenge must be embedded in `clientDataJSON.challenge` during
> the FIDO2 attestation ceremony.  Valid for 5 minutes.

```rust
pub fn issue_registration_challenge(env: Env, user: Address) -> Result<BytesN<32>, Error>
```

> Completes device registration after the FIDO2 attestation ceremony.
> 
> Attestation statement verification is performed off-chain by a trusted
> relayer before calling this function.  The contract validates:
> - A non-expired challenge was issued for `user`.
> - The public key size matches the declared algorithm.
> - The credential has not been registered before.
> - `MAX_DEVICES` has not been reached.
> 
> When the identity registry is configured the credential is also bound to
> the user's DID document as a FIDO2 verification method.
> 
> Returns the zero-based device index.

```rust
pub fn register_device( env: Env, user: Address, credential_id_hash: BytesN<32>, public_key: Bytes, algorithm: PublicKeyAlgorithm, device_name: String, attachment: AuthenticatorAttachment, transports: Vec<AuthenticatorTransport>, initial_sign_count: u32, aaguid: BytesN<16>, backup_eligible: bool, ) -> Result<u32, Error>
```

> Issues a one-time authentication challenge for `user`.

```rust
pub fn issue_auth_challenge(env: Env, user: Address) -> Result<BytesN<32>, Error>
```

> Verifies a FIDO2 assertion signed with Ed25519 (EdDSA).
> 
> The signed payload per FIDO2 Level 2 spec is:
> `authenticatorData || SHA-256(clientDataJSON)`
> 
> # Arguments
> * `user`               — authenticating user address.
> * `credential_id_hash` — SHA-256 of the credential ID.
> * `authenticator_data` — raw `authenticatorData` bytes (≥ 37 bytes).
> * `client_data_hash`   — `SHA-256(clientDataJSON)`.
> * `signature`          — 64-byte Ed25519 signature.
> * `new_sign_count`     — monotonic counter value from the authenticator.
> 
> The transaction is aborted (host trap) if the Ed25519 signature is invalid.

```rust
pub fn verify_ed25519_assertion( env: Env, user: Address, credential_id_hash: BytesN<32>, authenticator_data: Bytes, client_data_hash: BytesN<32>, signature: BytesN<64>, new_sign_count: u32, ) -> Result<AssertionResult, Error>
```

> Verifies a FIDO2 assertion for a P-256 (ES256) credential using a ZK proof.
> 
> Because Soroban does not natively support P-256 ECDSA verification, the
> caller submits a ZK proof generated by a trusted off-chain prover that
> attests to a valid P-256 signature over `authenticatorData || clientDataHash`.
> 
> The proof also enables privacy-preserving authentication: the `nullifier`
> and `commitment` allow proving key ownership without disclosing the exact
> device on every authentication.
> 
> # Arguments
> * `credential_id_hash` — identifies which registered P-256 device is used.
> * `nullifier`          — unique value preventing proof replay.
> * `commitment`         — public commitment included in the ZK circuit.
> * `proof`              — ZK proof bytes forwarded to the verifier contract.
> * `new_sign_count`     — monotonic counter value from the authenticator.
> * `vk_version`         — verifying key version for the ZK circuit.

```rust
pub fn verify_zk_assertion( env: Env, user: Address, credential_id_hash: BytesN<32>, nullifier: BytesN<32>, commitment: BytesN<32>, proof: Bytes, new_sign_count: u32, vk_version: u32, ) -> Result<AssertionResult, Error>
```

> Revokes a device, preventing it from being used for future authentications.
> 
> Both the device owner (`user`) and the contract admin may revoke devices.
> A `RevocationRecord` is appended to the user's audit log.

```rust
pub fn revoke_device( env: Env, caller: Address, user: Address, credential_id_hash: BytesN<32>, reason: String, ) -> Result<(), Error>
```

> Updates the user-assigned friendly name of a registered device.

```rust
pub fn update_device_name( env: Env, user: Address, credential_id_hash: BytesN<32>, new_name: String, ) -> Result<(), Error>
```

> Returns all devices registered for `user` (active and revoked).
> 
> Only the user or the admin may call this function.

```rust
pub fn list_devices( env: Env, caller: Address, user: Address, ) -> Result<Vec<Fido2Device>, Error>
```

> Returns the total device count (active + revoked) for `user`.

```rust
pub fn get_device_count(env: Env, user: Address) -> u32
```

> Returns the number of active (non-revoked) devices for `user`.

```rust
pub fn get_active_device_count(env: Env, user: Address) -> u32
```

> Returns `true` if `credential_id_hash` is registered and active for `user`.

```rust
pub fn is_device_registered(env: Env, user: Address, credential_id_hash: BytesN<32>) -> bool
```

> Returns the full revocation audit history for `user`.
> 
> Only the user or the admin may call this function.

```rust
pub fn get_revocation_history( env: Env, caller: Address, user: Address, ) -> Result<Vec<RevocationRecord>, Error>
```

---

## fp_math

> Multiply `amount` by basis points (1 bps = 0.01%) using floor division.
> 
> Floor rounding ensures fees are never rounded up — the fee taker always
> receives ≤ the exact fractional amount. Callers can reconstruct the
> complementary side as `amount - fee` to guarantee `fee + remainder == amount`.
> 
> Returns `None` if the intermediate `amount * bps` overflows `i128`.

```rust
pub fn mul_bps(amount: i128, bps: u32) -> Option<i128>
```

> Multiply `amount` by basis points with round-half-up rounding.
> 
> Returns `None` on overflow.

```rust
pub fn mul_bps_round_half_up(amount: i128, bps: u32) -> Option<i128>
```

> Calculate tokens to allocate for a payment:
> `tokens = payment * 10^token_decimals / price_per_token`
> 
> Returns `None` on overflow or if `price_per_token` is zero.

```rust
pub fn tokens_for_payment( payment: u128, price_per_token: u128, token_decimals: u32, ) -> Option<u128>
```

---

## governor

```rust
pub fn initialize( env: Env, token: Address, timelock: Address, voting_delay: u64, voting_period: u64, quorum_bps: u32, proposal_threshold: i128, reputation_contract: Option<Address>, dispute_contract: Option<Address>, ) -> Result<(), Error>
```

```rust
pub fn propose( env: Env, proposer: Address, description_hash: Bytes, execution_data: Bytes, ) -> Result<u64, Error>
```

```rust
pub fn cast_vote( env: Env, proposal_id: u64, voter: Address, support: u32, ) -> Result<(), Error>
```

```rust
pub fn state(env: Env, proposal_id: u64) -> Result<u32, Error>
```

```rust
pub fn queue(env: Env, proposal_id: u64) -> Result<(), Error>
```

```rust
pub fn execute(env: Env, proposal_id: u64) -> Result<(), Error>
```

```rust
pub fn balance_of(env: Env, user: Address) -> i128
```

```rust
pub fn set_bal(env: Env, user: Address, amount: i128)
```

---

## healthcare_data_conversion

> Initialize the healthcare data conversion contract

```rust
pub fn initialize(env: Env, admin: Address) -> Result<bool, Error>
```

> Register a conversion rule

```rust
pub fn register_conversion_rule( env: Env, admin: Address, rule: ConversionRule, ) -> Result<bool, Error>
```

> Get conversion rule

```rust
pub fn get_conversion_rule(env: Env, rule_id: String) -> Result<ConversionRule, Error>
```

> Register healthcare coding mapping (e.g., ICD9 to ICD10)

```rust
pub fn register_coding_mapping( env: Env, admin: Address, mapping: CodingMapping, ) -> Result<bool, Error>
```

> Get coding mapping

```rust
pub fn get_coding_mapping(env: Env, mapping_id: String) -> Result<CodingMapping, Error>
```

> Get coding mapping by source and target codes

```rust
pub fn find_coding_mapping( env: Env, _source_system: String, _target_system: String, _source_code: String, ) -> Result<CodingMapping, Error>
```

> Register format specification

```rust
pub fn register_format_specification( env: Env, admin: Address, spec: FormatSpecification, ) -> Result<bool, Error>
```

> Get format specification

```rust
pub fn get_format_specification( env: Env, format: DataFormat, ) -> Result<FormatSpecification, Error>
```

> Validate data format conversion compatibility

```rust
pub fn validate_conversion( env: Env, validator: Address, source_format: DataFormat, target_format: DataFormat, _source_data_hash: BytesN<32>, ) -> Result<ValidationResult, Error>
```

> Record a data conversion request

```rust
pub fn record_conversion( env: Env, requester: Address, source_format: DataFormat, target_format: DataFormat, source_data_hash: BytesN<32>, target_data_hash: BytesN<32>, ) -> Result<u64, Error>
```

> Get conversion request details

```rust
pub fn get_conversion_request(env: Env, request_id: u64) -> Result<ConversionRequest, Error>
```

> Record lossy conversion warning

```rust
pub fn record_lossy_conversion_warning( env: Env, admin: Address, warning: LossyConversionWarning, ) -> Result<bool, Error>
```

> Get lossy conversion warning

```rust
pub fn get_lossy_conversion_warning( env: Env, warning_id: String, ) -> Result<LossyConversionWarning, Error>
```

> Pause contract operations

```rust
pub fn pause(env: Env, admin: Address) -> Result<bool, Error>
```

> Resume contract operations

```rust
pub fn resume(env: Env, admin: Address) -> Result<bool, Error>
```

---

## homomorphic_registry

```rust
pub fn initialize(env: Env, admin: Address) -> Result<(), Error>
```

```rust
pub fn register_key_bundle( env: Env, admin: Address, key_id: BytesN<32>, context_id: BytesN<32>, public_key_ref: String, eval_key_ref: String, relin_key_ref: String, galois_key_ref: String, key_hash: BytesN<32>, ) -> Result<(), Error>
```

```rust
pub fn get_active_key_bundle( env: Env, context_id: BytesN<32>, ) -> Result<Option<FHEKeyBundle>, Error>
```

```rust
pub fn set_performance_profile( env: Env, admin: Address, context_id: BytesN<32>, batching_enabled: bool, max_batch_size: u32, lazy_relinearization: bool, auto_bootstrap: bool, bootstrap_threshold: u32, ) -> Result<(), Error>
```

```rust
pub fn get_performance_profile( env: Env, context_id: BytesN<32>, ) -> Result<Option<PerformanceProfile>, Error>
```

```rust
pub fn encrypt_ckks_vector( env: Env, submitter: Address, ciphertext_id: BytesN<32>, context_id: BytesN<32>, values: Vec<i128>, scale: u32, ) -> Result<(), Error>
```

```rust
pub fn encrypt_bgv_vector( env: Env, submitter: Address, ciphertext_id: BytesN<32>, context_id: BytesN<32>, values: Vec<i128>, ) -> Result<(), Error>
```

```rust
pub fn fhe_add( env: Env, submitter: Address, output_id: BytesN<32>, left_id: BytesN<32>, right_id: BytesN<32>, ) -> Result<(), Error>
```

```rust
pub fn fhe_multiply( env: Env, submitter: Address, output_id: BytesN<32>, left_id: BytesN<32>, right_id: BytesN<32>, ) -> Result<(), Error>
```

```rust
pub fn bootstrap_ciphertext( env: Env, admin: Address, ciphertext_id: BytesN<32>, ) -> Result<(), Error>
```

```rust
pub fn get_ciphertext( env: Env, ciphertext_id: BytesN<32>, ) -> Result<Option<EncryptedVector>, Error>
```

```rust
pub fn encrypted_statistics( env: Env, submitter: Address, ciphertext_id: BytesN<32>, ) -> Result<EncryptedStats, Error>
```

```rust
pub fn encrypted_linear_inference( env: Env, submitter: Address, output_id: BytesN<32>, features_id: BytesN<32>, model_weights: Vec<i128>, bias: i128, ) -> Result<(), Error>
```

```rust
pub fn estimate_operation_cost( env: Env, context_id: BytesN<32>, multiplicative_depth: u32, slot_count: u32, ) -> Result<u64, Error>
```

```rust
pub fn register_context( env: Env, admin: Address, context_id: BytesN<32>, scheme: HEScheme, params_ref: String, params_hash: BytesN<32>, ) -> Result<(), Error>
```

```rust
pub fn deactivate_context( env: Env, admin: Address, context_id: BytesN<32>, ) -> Result<(), Error>
```

```rust
pub fn submit_encrypted_computation( env: Env, submitter: Address, computation_id: BytesN<32>, context_id: BytesN<32>, ciphertext_ref: String, ciphertext_hash: BytesN<32>, proof_ref: String, proof_hash: BytesN<32>, ) -> Result<(), Error>
```

```rust
pub fn get_context(env: Env, context_id: BytesN<32>) -> Result<Option<HEContext>, Error>
```

```rust
pub fn get_computation( env: Env, computation_id: BytesN<32>, ) -> Result<Option<EncryptedComputation>, Error>
```

---

## ihe_integration

```rust
pub fn initialize(env: Env, admin: Address) -> Result<(), Error>
```

> Register a new document in the XDS registry

```rust
pub fn xds_register_document( env: Env, author: Address, entry: XDSDocumentEntry, ) -> Result<(), Error>
```

> Deprecate an existing XDS document entry

```rust
pub fn xds_deprecate_document( env: Env, author: Address, document_id: String, ) -> Result<(), Error>
```

> Query XDS documents for a patient

```rust
pub fn xds_query_documents( env: Env, requester: Address, patient_id: String, ) -> Result<Vec<XDSDocumentEntry>, Error>
```

> Retrieve a single XDS document entry

```rust
pub fn xds_retrieve_document( env: Env, requester: Address, document_id: String, ) -> Result<XDSDocumentEntry, Error>
```

> Submit an XDS submission set (groups documents from one clinical event)

```rust
pub fn xds_submit_document_set( env: Env, author: Address, submission_set: XDSSubmissionSet, ) -> Result<(), Error>
```

> Register a patient identity and return the cross-reference record ID

```rust
pub fn pix_register_patient( env: Env, actor: Address, local_id: PatientIdentifier, cross_ids: Vec<PatientIdentifier>, ) -> Result<u64, Error>
```

> Query all cross-referenced identifiers for a patient

```rust
pub fn pix_query_identifiers( env: Env, requester: Address, patient_id: String, ) -> Result<Vec<PIXCrossReference>, Error>
```

> Merge two patient identities (PIX merge operation)

```rust
pub fn pix_merge_patients( env: Env, actor: Address, surviving_ref_id: u64, subsumed_ref_id: u64, ) -> Result<(), Error>
```

> Register or update patient demographics

```rust
pub fn pdq_register_demographics( env: Env, actor: Address, demographics: PatientDemographics, ) -> Result<(), Error>
```

> Execute a PDQ demographics query; returns matching records

```rust
pub fn pdq_query( env: Env, requester: Address, query_params: Map<String, String>, requesting_system: String, hl7_type: HL7MessageType, domain_filter: String, ) -> Result<u64, Error>
```

> Retrieve patient demographics by patient ID

```rust
pub fn pdq_get_demographics( env: Env, requester: Address, patient_id: String, ) -> Result<PatientDemographics, Error>
```

> Log an ATNA-compliant audit event (used by external actors and other profiles)

```rust
pub fn atna_log_event( env: Env, actor: Address, event_type: ATNAEventType, event_action_code: String, event_outcome: ATNAEventOutcome, source_id: String, source_type: String, active_participants: Vec<ATNAParticipant>, participant_objects: Vec<ATNAParticipantObject>, hl7_message_id: String, profile: IHEProfile, ) -> Result<u64, Error>
```

> Retrieve an ATNA audit event by ID

```rust
pub fn atna_get_event(env: Env, event_id: u64) -> Result<ATNAAuditEvent, Error>
```

> Authenticate a node and record the ATNA authentication event

```rust
pub fn atna_authenticate_node( env: Env, node: Address, node_id: String, certificate_hash: BytesN<32>, ) -> Result<u64, Error>
```

> Register a cross-community gateway

```rust
pub fn xca_register_gateway( env: Env, admin: Address, gateway: XCAGateway, ) -> Result<(), Error>
```

> Initiate a cross-gateway query (returns gateway record for routing)

```rust
pub fn xca_initiate_query( env: Env, requester: Address, gateway_id: String, patient_id: String, ) -> Result<XCAGateway, Error>
```

> Register a master patient record linking multiple local identifiers

```rust
pub fn mpi_register_master_patient( env: Env, actor: Address, global_patient_id: String, demographics: PatientDemographics, linked_ids: Vec<PatientIdentifier>, confidence_score: u32, ) -> Result<u64, Error>
```

> Find a master patient record by global patient ID

```rust
pub fn mpi_find_patient( env: Env, requester: Address, global_patient_id: String, ) -> Result<MPIMasterPatient, Error>
```

> Reliable document interchange — wraps XDS registration with delivery confirmation

```rust
pub fn xdr_send_document( env: Env, sender: Address, entry: XDSDocumentEntry, intended_recipient: String, ) -> Result<(), Error>
```

> Record a media interchange package (content hash stored on-chain)

```rust
pub fn xdm_record_media_package( env: Env, actor: Address, package_id: String, patient_id: String, content_hash: BytesN<32>, media_type: String, document_ids: Vec<String>, ) -> Result<(), Error>
```

> Record a time synchronization event on-chain

```rust
pub fn ct_record_time_sync( env: Env, actor: Address, node_id: String, reported_time: u64, ) -> Result<u64, Error>
```

> Register a patient privacy consent document

```rust
pub fn bppc_register_consent( env: Env, author: Address, patient_id: String, policy_id: String, access_consent_list: Vec<String>, expiry_time: u64, document_ref: String, ) -> Result<u64, Error>
```

> Revoke a privacy consent

```rust
pub fn bppc_revoke_consent(env: Env, author: Address, consent_id: u64) -> Result<(), Error>
```

> Verify consent is active and not expired

```rust
pub fn bppc_verify_consent(env: Env, consent_id: u64) -> Result<BPPCConsent, Error>
```

> Record a digital signature for a document

```rust
pub fn dsg_sign_document( env: Env, signer: Address, document_id: String, signature_hash: BytesN<32>, signature_algorithm: String, certificate_ref: String, signature_purpose: String, ) -> Result<u64, Error>
```

> Verify a document signature by signature ID

```rust
pub fn dsg_verify_signature(env: Env, signature_id: u64) -> Result<DSGSignature, Error>
```

> Get all signatures for a document

```rust
pub fn dsg_get_document_signatures( env: Env, document_id: String, ) -> Result<Vec<DSGSignature>, Error>
```

> Register a provider in the Healthcare Provider Directory

```rust
pub fn hpd_register_provider( env: Env, actor: Address, provider: HPDProvider, ) -> Result<u64, Error>
```

> Query a provider by ID

```rust
pub fn hpd_get_provider(env: Env, provider_id: u64) -> Result<HPDProvider, Error>
```

> Register a named value set

```rust
pub fn svs_register_value_set( env: Env, actor: Address, value_set: SVSValueSet, ) -> Result<u64, Error>
```

> Retrieve a value set by OID

```rust
pub fn svs_get_value_set_by_oid(env: Env, oid: String) -> Result<SVSValueSet, Error>
```

> Record the result of a Connectathon conformance test

```rust
pub fn connectathon_record_test( env: Env, tester: Address, profile: IHEProfile, actor_name: String, test_name: String, passed: bool, notes: String, ) -> Result<u64, Error>
```

> Get all Connectathon test results for a profile

```rust
pub fn connectathon_get_profile_results( env: Env, profile: IHEProfile, ) -> Vec<ConnectathonTestResult>
```

> Check if a profile passes all recorded Connectathon tests

```rust
pub fn connectathon_is_compliant(env: Env, profile: IHEProfile) -> bool
```

---

## iot_device_management

```rust
pub fn initialize(env: Env, admin: Address) -> Result<(), Error>
```

```rust
pub fn pause(env: Env, admin: Address) -> Result<(), Error>
```

```rust
pub fn unpause(env: Env, admin: Address) -> Result<(), Error>
```

```rust
pub fn set_role(env: Env, admin: Address, user: Address, role: Role) -> Result<(), Error>
```

```rust
pub fn get_role(env: Env, user: Address) -> Role
```

```rust
pub fn register_manufacturer( env: Env, admin: Address, manufacturer_id: BytesN<32>, name: String, certification_hash: BytesN<32>, ) -> Result<(), Error>
```

```rust
pub fn get_manufacturer(env: Env, manufacturer_id: BytesN<32>) -> Result<Manufacturer, Error>
```

```rust
pub fn deactivate_manufacturer( env: Env, admin: Address, manufacturer_id: BytesN<32>, ) -> Result<(), Error>
```

```rust
pub fn register_device( env: Env, operator: Address, device_id: BytesN<32>, manufacturer_id: BytesN<32>, device_type: DeviceType, model: String, serial_number: String, location: String, encryption_key_hash: BytesN<32>, metadata_ref: String, ) -> Result<(), Error>
```

```rust
pub fn get_device(env: Env, device_id: BytesN<32>) -> Result<Device, Error>
```

```rust
pub fn get_device_count(env: Env) -> u64
```

```rust
pub fn get_devices_by_operator(env: Env, operator: Address) -> Vec<BytesN<32>>
```

```rust
pub fn activate_device(env: Env, caller: Address, device_id: BytesN<32>) -> Result<(), Error>
```

```rust
pub fn suspend_device(env: Env, caller: Address, device_id: BytesN<32>) -> Result<(), Error>
```

```rust
pub fn decommission_device( env: Env, admin: Address, device_id: BytesN<32>, ) -> Result<(), Error>
```

```rust
pub fn publish_firmware( env: Env, caller: Address, manufacturer_id: BytesN<32>, version: u32, device_type: DeviceType, binary_hash: BytesN<32>, release_notes_ref: String, min_version: u32, size_bytes: u64, ) -> Result<(), Error>
```

```rust
pub fn approve_firmware( env: Env, admin: Address, manufacturer_id: BytesN<32>, version: u32, ) -> Result<(), Error>
```

```rust
pub fn reject_firmware( env: Env, admin: Address, manufacturer_id: BytesN<32>, version: u32, ) -> Result<(), Error>
```

```rust
pub fn get_firmware( env: Env, manufacturer_id: BytesN<32>, version: u32, ) -> Result<FirmwareVersion, Error>
```

```rust
pub fn get_latest_firmware_version( env: Env, manufacturer_id: BytesN<32>, device_type: DeviceType, ) -> Result<u32, Error>
```

```rust
pub fn update_device_firmware( env: Env, caller: Address, device_id: BytesN<32>, target_version: u32, ) -> Result<u64, Error>
```

```rust
pub fn submit_heartbeat( env: Env, caller: Address, device_id: BytesN<32>, health_status: HealthStatus, battery_pct: u32, signal_strength: u32, error_count: u32, metrics_ref: String, ) -> Result<(), Error>
```

```rust
pub fn get_device_heartbeats(env: Env, device_id: BytesN<32>) -> Result<Vec<Heartbeat>, Error>
```

```rust
pub fn get_device_uptime_bps(env: Env, device_id: BytesN<32>) -> Result<u32, Error>
```

```rust
pub fn get_active_device_count(env: Env) -> u64
```

```rust
pub fn set_heartbeat_interval( env: Env, admin: Address, interval_secs: u64, ) -> Result<(), Error>
```

```rust
pub fn create_comm_channel( env: Env, caller: Address, device_id: BytesN<32>, channel_id: BytesN<32>, encryption_key_hash: BytesN<32>, protocol: String, ) -> Result<(), Error>
```

```rust
pub fn get_comm_channel(env: Env, channel_id: BytesN<32>) -> Result<CommChannel, Error>
```

```rust
pub fn rotate_encryption_key( env: Env, caller: Address, channel_id: BytesN<32>, new_encryption_key_hash: BytesN<32>, ) -> Result<(), Error>
```

```rust
pub fn rotate_device_key( env: Env, caller: Address, device_id: BytesN<32>, new_encryption_key_hash: BytesN<32>, ) -> Result<(), Error>
```

```rust
pub fn set_key_rotation_interval( env: Env, admin: Address, interval_secs: u64, ) -> Result<(), Error>
```

```rust
pub fn get_devices_by_manufacturer(env: Env, manufacturer_id: BytesN<32>) -> Vec<BytesN<32>>
```

```rust
pub fn get_device_firmware_history( env: Env, device_id: BytesN<32>, ) -> Result<Vec<FirmwareUpdateRecord>, Error>
```

```rust
pub fn get_manufacturer_count(env: Env) -> u32
```

```rust
pub fn get_firmware_update_record( env: Env, update_id: u64, ) -> Result<FirmwareUpdateRecord, Error>
```

---

## load_testing

> Execute a load-test run and persist the result.
> 
> Each "operation" is a lightweight storage read/write that exercises the
> contract's execution path.  Latency is measured in ledger sequence units.

```rust
pub fn run(env: Env, config: LoadTestConfig) -> LoadTestResult
```

> Return the result of the most recent run.

```rust
pub fn last_result(env: Env) -> Option<LoadTestResult>
```

> Return the total number of runs executed.

```rust
pub fn run_count(env: Env) -> u32
```

---

## medical_consent_nft

> Initialize the contract with an admin

```rust
pub fn initialize(env: Env, admin: Address) -> Result<(), ContractError>
```

> Add an authorized issuer (clinic/healthcare provider)

```rust
pub fn add_issuer(env: Env, issuer: Address)
```

> Remove an authorized issuer

```rust
pub fn remove_issuer(env: Env, issuer: Address)
```

> Check if address is an authorized issuer

```rust
pub fn is_issuer(env: Env, address: Address) -> bool
```

> Mint a new consent token - FIXED: Add issuer: Address param, require_auth on it, use for check & metadata (no env.invoker())

```rust
pub fn mint_consent( env: Env, issuer: Address, // FIXED: Passed by caller (must be their own Address::AccountId) patient: Address, // Renamed from 'to' for clarity metadata_uri: String, consent_type: String, expiry_timestamp: u64, ) -> Result<u64, ContractError>
```

> Update consent metadata (creates new version)

```rust
pub fn update_consent( env: Env, token_id: u64, new_metadata_uri: String, ) -> Result<(), ContractError>
```

> Revoke consent (marks as revoked, prevents transfers) - Patient authorizes via require_auth on their address from metadata

```rust
pub fn revoke_consent(env: Env, token_id: u64) -> Result<(), ContractError>
```

> Transfer consent token (blocked if revoked)

```rust
pub fn transfer( env: Env, from: Address, to: Address, token_id: u64, ) -> Result<(), ContractError>
```

> Get token owner

```rust
pub fn owner_of(env: Env, token_id: u64) -> Address
```

> Get consent metadata

```rust
pub fn get_metadata(env: Env, token_id: u64) -> ConsentMetadata
```

> Check if consent is revoked

```rust
pub fn is_revoked(env: Env, token_id: u64) -> bool
```

> Get consent history (audit trail)

```rust
pub fn get_history(env: Env, token_id: u64) -> Vec<ConsentHistoryEntry>
```

> Get all tokens owned by an address

```rust
pub fn tokens_of_owner(env: Env, owner: Address) -> Vec<u64>
```

> Check if doctor has valid consent for patient and type (for cross-contract access control)

```rust
pub fn has_consent(env: Env, patient: Address, doctor: Address, consent_type: String) -> bool
```

> Check if consent is valid (not revoked and not expired)

```rust
pub fn is_valid(env: Env, token_id: u64) -> bool
```

> Set granular permissions for a consent token

```rust
pub fn set_granular_permissions( env: Env, caller: Address, token_id: u64, permissions: GranularPermissions, ) -> Result<(), ContractError>
```

> Get granular permissions for a consent token

```rust
pub fn get_granular_permissions( env: Env, token_id: u64, ) -> Result<GranularPermissions, ContractError>
```

> Check if requester has permission for specific data type

```rust
pub fn has_permission( env: Env, token_id: u64, requester: Address, data_type: DataType, required_level: PermissionLevel, ) -> bool
```

> Set access controls for a consent token

```rust
pub fn set_access_controls( env: Env, token_id: u64, access_control: AccessControl, ) -> Result<(), ContractError>
```

> Check if access is allowed based on access controls

```rust
pub fn check_access_allowed( env: Env, token_id: u64, _requester: Address, ) -> Result<bool, ContractError>
```

> Record access attempt

```rust
pub fn record_access( env: Env, token_id: u64, _requester: Address, ) -> Result<(), ContractError>
```

> Delegate consent to another address

```rust
pub fn delegate_consent( env: Env, token_id: u64, delegate: Address, permissions: GranularPermissions, expiry_timestamp: u64, ) -> Result<(), ContractError>
```

> Revoke delegation

```rust
pub fn revoke_delegation( env: Env, token_id: u64, delegate: Address, ) -> Result<(), ContractError>
```

> Get active delegations for a token

```rust
pub fn get_delegations(env: Env, token_id: u64) -> Vec<Delegation>
```

> Set consent inheritance (child consent inherits from parent)

```rust
pub fn set_inheritance( env: Env, child_token_id: u64, parent_token_id: u64, inherited_permissions: GranularPermissions, ) -> Result<(), ContractError>
```

> Add emergency authority

```rust
pub fn add_emergency_authority(env: Env, authority: Address) -> Result<(), ContractError>
```

> Emergency override access

```rust
pub fn emergency_override( env: Env, caller: Address, token_id: u64, reason: String, duration: u64, ) -> Result<u64, ContractError>
```

> Enable/disable marketplace

```rust
pub fn set_marketplace_enabled(env: Env, enabled: bool) -> Result<(), ContractError>
```

> List consent on marketplace for research

```rust
pub fn list_on_marketplace( env: Env, token_id: u64, price: i128, data_types: Vec<DataType>, research_purpose: String, duration: u64, ) -> Result<(), ContractError>
```

> Get marketplace listing

```rust
pub fn get_marketplace_listing( env: Env, token_id: u64, ) -> Result<MarketplaceListing, ContractError>
```

> Purchase marketplace listing (simplified - would need payment integration)

```rust
pub fn purchase_marketplace_listing( env: Env, token_id: u64, buyer: Address, ) -> Result<(), ContractError>
```

> Enhanced dynamic consent update with version history

```rust
pub fn update_consent_dynamic( env: Env, caller: Address, token_id: u64, new_metadata_uri: String, change_summary: String, ) -> Result<(), ContractError>
```

> Get version history

```rust
pub fn get_version_history(env: Env, token_id: u64) -> Vec<VersionHistoryEntry>
```

> Enable dynamic updates for a consent

```rust
pub fn enable_dynamic_updates(env: Env, token_id: u64) -> Result<(), ContractError>
```

> Get analytics data

```rust
pub fn get_analytics(env: Env) -> AnalyticsData
```

> Generate consent report for a patient

```rust
pub fn generate_consent_report(env: Env, patient: Address) -> Vec<u64>
```

---

## medical_record_backup

```rust
pub fn initialize(env: Env, admin: Address) -> Result<(), Error>
```

```rust
pub fn set_paused(env: Env, caller: Address, paused: bool) -> Result<bool, Error>
```

```rust
pub fn assign_role( env: Env, caller: Address, user: Address, role_mask: u32, ) -> Result<bool, Error>
```

```rust
pub fn set_policy(env: Env, caller: Address, policy: BackupPolicy) -> Result<bool, Error>
```

```rust
pub fn get_policy(env: Env) -> Result<BackupPolicy, Error>
```

```rust
pub fn register_target( env: Env, caller: Address, network: BackupNetwork, region: GeoRegion, endpoint_hash: BytesN<32>, encrypted_only: bool, cost_weight: u32, max_capacity_units: u64, ) -> Result<u32, Error>
```

```rust
pub fn set_target_active( env: Env, caller: Address, target_id: u32, active: bool, ) -> Result<bool, Error>
```

```rust
pub fn get_target(env: Env, target_id: u32) -> Option<BackupTarget>
```

```rust
pub fn list_targets(env: Env) -> Vec<BackupTarget>
```

```rust
pub fn run_scheduled_backup( env: Env, caller: Address, source_root: BytesN<32>, snapshot_ref: String, encryption_key_version: u32, ) -> Result<u64, Error>
```

```rust
pub fn run_backup_now( env: Env, caller: Address, source_root: BytesN<32>, snapshot_ref: String, encryption_key_version: u32, ) -> Result<u64, Error>
```

```rust
pub fn verify_backup_integrity( env: Env, caller: Address, artifact_id: u64, observed_checksum: BytesN<32>, ) -> Result<bool, Error>
```

```rust
pub fn request_restore( env: Env, caller: Address, artifact_id: u64, reason_hash: BytesN<32>, ) -> Result<u64, Error>
```

```rust
pub fn approve_restore(env: Env, caller: Address, request_id: u64) -> Result<bool, Error>
```

```rust
pub fn execute_restore(env: Env, caller: Address, request_id: u64) -> Result<String, Error>
```

```rust
pub fn run_recovery_test( env: Env, caller: Address, artifact_id: u64, validation_hash: BytesN<32>, ) -> Result<u64, Error>
```

```rust
pub fn optimize_and_cleanup(env: Env, caller: Address) -> Result<CleanupReport, Error>
```

```rust
pub fn report_target_failure( env: Env, caller: Address, target_id: u32, reason_hash: BytesN<32>, ) -> Result<bool, Error>
```

```rust
pub fn resolve_alert(env: Env, caller: Address, alert_id: u64) -> Result<bool, Error>
```

```rust
pub fn list_alerts(env: Env, open_only: bool) -> Vec<AlertEntry>
```

```rust
pub fn list_artifacts(env: Env, include_archived: bool) -> Vec<BackupArtifact>
```

```rust
pub fn get_artifact(env: Env, artifact_id: u64) -> Option<BackupArtifact>
```

```rust
pub fn get_execution(env: Env, execution_id: u64) -> Option<BackupExecution>
```

```rust
pub fn get_restore_request(env: Env, request_id: u64) -> Option<RestoreRequest>
```

```rust
pub fn get_recovery_test(env: Env, test_id: u64) -> Option<RecoveryTest>
```

```rust
pub fn get_health(env: Env) -> BackupHealth
```

```rust
pub fn get_schedule(env: Env) -> (u64, u64)
```

---

## medical_record_search

```rust
pub fn initialize(env: Env, admin: Address) -> Result<(), Error>
```

```rust
pub fn set_paused(env: Env, caller: Address, paused: bool) -> Result<bool, Error>
```

```rust
pub fn assign_role( env: Env, caller: Address, user: Address, role_mask: u32, ) -> Result<bool, Error>
```

```rust
pub fn set_cache_policy(env: Env, caller: Address, policy: CachePolicy) -> Result<bool, Error>
```

```rust
pub fn set_ranking(env: Env, caller: Address, cfg: RankingConfig) -> Result<bool, Error>
```

```rust
pub fn index_record(env: Env, caller: Address, input: IndexInput) -> Result<bool, Error>
```

```rust
pub fn batch_index_records( env: Env, caller: Address, inputs: Vec<IndexInput>, ) -> Result<(u32, u32), Error>
```

```rust
pub fn search( env: Env, caller: Address, query: SearchQuery, page: u32, page_size: u32, ) -> Result<Vec<SearchResult>, Error>
```

```rust
pub fn get_cache_entry(env: Env, query_hash: BytesN<32>) -> Result<QueryCacheEntry, Error>
```

```rust
pub fn invalidate_cache(env: Env, caller: Address) -> Result<bool, Error>
```

```rust
pub fn get_audit(env: Env, caller: Address, query_id: u64) -> Result<SearchAuditEntry, Error>
```

```rust
pub fn preview_query_hash(env: Env, query: SearchQuery) -> BytesN<32>
```

```rust
pub fn get_indexed_entry(env: Env, record_id: u64) -> Result<SearchIndexEntry, Error>
```

---

## mpc_manager

```rust
pub fn initialize(env: Env, admin: Address) -> Result<(), Error>
```

```rust
pub fn start_session( env: Env, initiator: Address, session_id: BytesN<32>, participants: Vec<Address>, threshold: u32, purpose: String, ttl_secs: u64, computation_type: ComputationType, ) -> Result<(), Error>
```

```rust
pub fn commit_share( env: Env, participant: Address, session_id: BytesN<32>, commitment_hash: BytesN<32>, ) -> Result<(), Error>
```

```rust
pub fn reveal_share( env: Env, participant: Address, session_id: BytesN<32>, share_ref: String, share_hash: BytesN<32>, ) -> Result<(), Error>
```

```rust
pub fn finalize_session( env: Env, initiator: Address, session_id: BytesN<32>, result_ref: String, result_hash: BytesN<32>, proof_ref: String, proof_hash: BytesN<32>, ) -> Result<(), Error>
```

```rust
pub fn get_session(env: Env, session_id: BytesN<32>) -> Result<Option<MPCSession>, Error>
```

```rust
pub fn get_commitment( env: Env, session_id: BytesN<32>, participant: Address, ) -> Result<Option<BytesN<32>>, Error>
```

```rust
pub fn get_reveal( env: Env, session_id: BytesN<32>, participant: Address, ) -> Result<Option<ShareReveal>, Error>
```

> Create Shamir's Secret Sharing shares for medical record encryption keys

```rust
pub fn create_secret_shares( env: Env, participant: Address, session_id: BytesN<32>, secret: Bytes, num_shares: u32, threshold: u32, ) -> Result<Vec<SecretShare>, Error>
```

> Submit a computation proof for verification

```rust
pub fn submit_computation_proof( env: Env, participant: Address, session_id: BytesN<32>, proof: ComputationProof, ) -> Result<(), Error>
```

> Perform privacy-preserving statistical analysis

```rust
pub fn perform_statistical_analysis( env: Env, participant: Address, session_id: BytesN<32>, _analysis_type: String, encrypted_data: Bytes, ) -> Result<BytesN<32>, Error>
```

> Train machine learning model on encrypted data

```rust
pub fn train_secure_ml_model( env: Env, participant: Address, session_id: BytesN<32>, model_params: Bytes, training_data: Bytes, ) -> Result<BytesN<32>, Error>
```

> Get audit trail for a session

```rust
pub fn get_audit_trail(env: Env, session_id: BytesN<32>) -> Result<Vec<AuditEntry>, Error>
```

> Get gas usage statistics for a session

```rust
pub fn get_gas_stats(env: Env, session_id: BytesN<32>) -> Result<u64, Error>
```

---

## notification_system

> Initialise the contract. Must be called exactly once.

```rust
pub fn initialize(env: Env, admin: Address) -> Result<(), Error>
```

> Returns the current admin address.

```rust
pub fn get_admin(env: Env) -> Result<Address, Error>
```

> Authorise `sender` to create notifications on behalf of integrated contracts.

```rust
pub fn add_authorized_sender(env: Env, caller: Address, sender: Address) -> Result<(), Error>
```

> Revoke a sender's authorisation.

```rust
pub fn remove_authorized_sender( env: Env, caller: Address, sender: Address, ) -> Result<(), Error>
```

> Returns the list of all currently authorised sender addresses.

```rust
pub fn get_authorized_senders(env: Env) -> Result<Vec<Address>, Error>
```

> Upsert `user`'s notification preferences. The user must sign the call.

```rust
pub fn set_preferences( env: Env, user: Address, prefs: NotificationPreferences, ) -> Result<(), Error>
```

> Returns the preferences for `user`, or `None` if not configured.

```rust
pub fn get_preferences( env: Env, user: Address, ) -> Result<Option<NotificationPreferences>, Error>
```

> Create a single notification for `recipient`.
> Caller must be the admin or an authorised sender.
> Returns the assigned notification ID.

```rust
pub fn create_notification( env: Env, sender: Address, recipient: Address, notif_type: NotificationType, priority: AlertPriority, title: String, message: String, reference_id: Option<u64>, expires_at: Option<u64>, ) -> Result<u64, Error>
```

> Create one notification per recipient in `recipients`.
> Bounded by MAX_BULK_RECIPIENTS to cap gas cost.

```rust
pub fn create_bulk_notifications( env: Env, sender: Address, recipients: Vec<Address>, notif_type: NotificationType, priority: AlertPriority, title: String, message: String, reference_id: Option<u64>, expires_at: Option<u64>, ) -> Result<Vec<u64>, Error>
```

> Fetch a single notification by ID.
> Only the recipient or admin may view it.

```rust
pub fn get_notification( env: Env, caller: Address, notif_id: u64, ) -> Result<Notification, Error>
```

> Paginated query over a user's notification history.
> Caller must be the user or admin.
> Results are returned newest-first; `filter.offset` skips matching records.

```rust
pub fn get_notifications( env: Env, caller: Address, user: Address, filter: NotificationFilter, ) -> Result<NotificationPage, Error>
```

> Returns the number of unread (Pending + Delivered) notifications for a user.

```rust
pub fn get_unread_count(env: Env, user: Address) -> Result<u32, Error>
```

> Mark a single notification as Read.
> Only the recipient may call this.

```rust
pub fn mark_read(env: Env, caller: Address, notif_id: u64) -> Result<(), Error>
```

> Mark all Pending / Delivered notifications for the caller as Read.
> Returns the count of newly-read notifications.

```rust
pub fn mark_all_read(env: Env, caller: Address) -> Result<u32, Error>
```

> Archive a notification so it no longer appears in default queries.
> Caller must be the recipient or admin.

```rust
pub fn archive_notification(env: Env, caller: Address, notif_id: u64) -> Result<(), Error>
```

> Create a new alert rule. Only admin may call this.

```rust
pub fn create_alert_rule( env: Env, caller: Address, name: String, watches_type: u32, priority: AlertPriority, recipients: Vec<Address>, ) -> Result<u64, Error>
```

> Update the active state, priority, and recipients of an existing rule.

```rust
pub fn update_alert_rule( env: Env, caller: Address, rule_id: u64, is_active: bool, priority: AlertPriority, recipients: Vec<Address>, ) -> Result<(), Error>
```

> Permanently delete an alert rule.

```rust
pub fn delete_alert_rule(env: Env, caller: Address, rule_id: u64) -> Result<(), Error>
```

> Returns all non-deleted alert rules. Admin only.

```rust
pub fn get_alert_rules(env: Env, caller: Address) -> Result<Vec<AlertRule>, Error>
```

> Trigger a specific alert rule: creates notifications for each of its recipients.
> Caller must be admin or an authorised sender.
> Returns the IDs of all created notifications.

```rust
pub fn trigger_alert( env: Env, sender: Address, rule_id: u64, reference_id: Option<u64>, custom_message: Option<String>, ) -> Result<Vec<u64>, Error>
```

> Upsert a localised notification template. Admin only.

```rust
pub fn set_template( env: Env, caller: Address, template: NotificationTemplate, ) -> Result<(), Error>
```

> Retrieve a template by notification type and locale.

```rust
pub fn get_template( env: Env, notif_type: u32, locale: String, ) -> Result<NotificationTemplate, Error>
```

> Returns aggregated send/read/pending counters. Admin only.

```rust
pub fn get_analytics(env: Env, caller: Address) -> Result<NotificationAnalytics, Error>
```

---

## patient_risk_stratification

```rust
pub fn initialize(env: Env, admin: Address) -> bool
```

```rust
pub fn register_risk_model( env: Env, caller: Address, model_id: BytesN<32>, model_type: RiskModelType, specialty: String, version: String, min_confidence_bps: u32, description: String, ) -> Result<bool, Error>
```

```rust
pub fn perform_risk_assessment( env: Env, caller: Address, patient: Address, model_id: BytesN<32>, risk_score_bps: u32, confidence_bps: u32, prediction_horizon_days: u32, risk_factors: Vec<RiskFactor>, interventions: Vec<InterventionRecommendation>, auc_score_bps: u32, ) -> Result<u64, Error>
```

```rust
pub fn get_risk_assessment(env: Env, assessment_id: u64) -> Option<RiskAssessment>
```

```rust
pub fn get_patient_risk_profile(env: Env, patient: Address) -> Option<PatientRiskProfile>
```

```rust
pub fn get_risk_model(env: Env, model_id: BytesN<32>) -> Option<RiskModel>
```

```rust
pub fn get_patient_risk_factors( env: Env, patient: Address, specialty: String, ) -> Vec<RiskFactor>
```

```rust
pub fn get_intervention_recommendations( env: Env, patient: Address, ) -> Vec<InterventionRecommendation>
```

```rust
pub fn update_model_status( env: Env, caller: Address, model_id: BytesN<32>, enabled: bool, ) -> Result<bool, Error>
```

---

## pharma_supply_chain

```rust
pub fn initialize(env: Env, admin: Address) -> Result<(), Error>
```

```rust
pub fn register_manufacturer( env: Env, admin: Address, operator: Address, name: String, license_number: String, ) -> Result<u64, Error>
```

```rust
pub fn register_medication( env: Env, caller: Address, manufacturer_id: u64, name: String, ndc: String, requires_cold_chain: bool, min_temp_c: i32, max_temp_c: i32, regulatory_region: String, ) -> Result<u64, Error>
```

```rust
pub fn create_batch( env: Env, caller: Address, medication_id: u64, lot_number: String, quantity: u32, auth_hash: BytesN<32>, expires_at: u64, ) -> Result<u64, Error>
```

```rust
pub fn verify_batch_authenticity( env: Env, batch_id: u64, auth_hash: BytesN<32>, ) -> Result<bool, Error>
```

```rust
pub fn create_shipment( env: Env, caller: Address, batch_id: u64, to: Address, carrier_ref: String, ) -> Result<u64, Error>
```

```rust
pub fn log_condition_data( env: Env, caller: Address, shipment_id: u64, temperature_c: i32, humidity_bps: u32, latitude_e6: i32, longitude_e6: i32, ) -> Result<bool, Error>
```

```rust
pub fn complete_shipment( env: Env, caller: Address, shipment_id: u64, verified: bool, ) -> Result<bool, Error>
```

```rust
pub fn run_compliance_check(env: Env, batch_id: u64) -> Result<bool, Error>
```

```rust
pub fn get_inventory_snapshot(env: Env, owner: Address) -> InventorySnapshot
```

```rust
pub fn optimize_inventory( env: Env, owner: Address, forecast_units: u32, ) -> InventoryRecommendation
```

```rust
pub fn get_batch(env: Env, batch_id: u64) -> Result<Batch, Error>
```

```rust
pub fn get_shipment(env: Env, shipment_id: u64) -> Result<Shipment, Error>
```

---

## predictive_analytics

```rust
pub fn initialize( env: Env, admin: Address, predictor: Address, prediction_horizon_days: u32, min_confidence_bps: u32, ) -> bool
```

```rust
pub fn update_config( env: Env, caller: Address, new_predictor: Option<Address>, new_horizon: Option<u32>, new_min_confidence: Option<u32>, enabled: Option<bool>, ) -> Result<bool, Error>
```

```rust
pub fn make_prediction( env: Env, caller: Address, patient: Address, model_id: BytesN<32>, outcome_type: String, predicted_value: u32, confidence_bps: u32, features_used: Vec<String>, explanation_ref: String, risk_factors: Vec<String>, ) -> Result<u64, Error>
```

```rust
pub fn get_prediction(env: Env, prediction_id: u64) -> Option<HealthPrediction>
```

```rust
pub fn get_config(env: Env) -> Option<PredictionConfig>
```

```rust
pub fn get_patient_summary(env: Env, patient: Address) -> Option<PatientPredictionsSummary>
```

```rust
pub fn get_model_metrics(env: Env, model_id: BytesN<32>) -> Option<PredictionMetrics>
```

```rust
pub fn update_model_metrics( env: Env, caller: Address, model_id: BytesN<32>, metrics: PredictionMetrics, ) -> Result<bool, Error>
```

```rust
pub fn has_high_risk_prediction(env: Env, patient: Address) -> bool
```

```rust
pub fn whitelist_predictor( env: Env, caller: Address, predictor_addr: Address, ) -> Result<bool, Error>
```

```rust
pub fn is_whitelisted_predictor(env: Env, predictor_addr: Address) -> bool
```

---

## provider_directory

```rust
pub fn initialize(env: Env, admin: Address) -> Result<(), Error>
```

```rust
pub fn set_rate_limit_config( env: Env, admin: Address, max_searches: u32, window_secs: u64, ) -> Result<(), Error>
```

```rust
pub fn set_institution_exemption( env: Env, admin: Address, institution: Address, is_exempt: bool, ) -> Result<(), Error>
```

```rust
pub fn search_providers( env: Env, caller: Address, _query: String, ) -> Result<Vec<Provider>, Error>
```

---

## public_health_surveillance

> Initialize the public health surveillance platform

```rust
pub fn initialize(env: Env, admin: Address) -> Result<(), Error>
```

> Report outbreak data with privacy preservation

```rust
pub fn report_outbreak_data( env: Env, provider: Address, data_id: BytesN<32>, encrypted_region: Bytes, disease_code: String, aggregated_cases: u64, time_period_start: u64, time_period_end: u64, aggregation_method: AggregationMethod, privacy_epsilon: u64, confidence_bps: u32, ) -> Result<(), Error>
```

> Create epidemic model for disease prediction

```rust
pub fn create_epidemic_model( env: Env, modeler: Address, model_id: BytesN<32>, disease_code: String, encrypted_scope: Bytes, model_type: String, r0_estimate: u64, incubation_days: u32, infectious_days: u32, case_fatality_bps: u32, ) -> Result<(), Error>
```

> Create public health alert

```rust
pub fn create_public_health_alert( env: Env, authority: Address, alert_type: AlertType, severity: DiseaseSeverity, encrypted_affected_regions: Bytes, message: String, recommended_actions: Vec<String>, expiration_hours: u32, ) -> Result<u64, Error>
```

> Report vaccination coverage with privacy preservation

```rust
pub fn report_vaccination_coverage( env: Env, provider: Address, coverage_id: BytesN<32>, encrypted_region: Bytes, vaccine_type: String, encrypted_target_population: u64, private_vaccinated_count: u64, coverage_bps: u32, reporting_period_start: u64, reporting_period_end: u64, ) -> Result<(), Error>
```

> Report environmental health data

```rust
pub fn report_environmental_health( env: Env, monitoring_station: Address, env_data_id: BytesN<32>, encrypted_location: Bytes, metric_type: String, aggregated_value: u64, risk_bps: u32, measurement_period_start: u64, measurement_period_end: u64, aggregation_method: AggregationMethod, privacy_epsilon: u64, ) -> Result<(), Error>
```

> Report antimicrobial resistance data

```rust
pub fn report_antimicrobial_resistance( env: Env, testing_lab: Address, amr_data_id: BytesN<32>, encrypted_region: Bytes, pathogen_code: String, antibiotic_class: String, resistance_bps: u32, private_sample_size: u64, aggregation_method: AggregationMethod, privacy_epsilon: u64, ) -> Result<(), Error>
```

> Report social determinants of health data

```rust
pub fn report_social_determinants( env: Env, data_source: Address, sdoh_data_id: BytesN<32>, encrypted_region: Bytes, determinant_type: String, aggregated_metric: u64, impact_bps: u32, aggregation_method: AggregationMethod, privacy_epsilon: u64, ) -> Result<(), Error>
```

> Create public health intervention

```rust
pub fn create_intervention( env: Env, coordinator: Address, intervention_id: BytesN<32>, intervention_type: String, encrypted_target_population: Bytes, encrypted_scope: Bytes, start_date: u64, end_date: u64, implementation_cost: u64, expected_outcomes: Vec<String>, aggregation_method: AggregationMethod, ) -> Result<(), Error>
```

> Create global health collaboration

```rust
pub fn create_global_collaboration( env: Env, lead_organization: Address, collaboration_id: BytesN<32>, participants: Vec<Address>, collaboration_type: String, data_sharing_protocol: String, exchange_method: AggregationMethod, objectives: Vec<String>, start_date: u64, end_date: u64, ) -> Result<(), Error>
```

> Get outbreak data

```rust
pub fn get_outbreak_data(env: Env, data_id: BytesN<32>) -> Result<OutbreakData, Error>
```

> Get epidemic model

```rust
pub fn get_epidemic_model(env: Env, model_id: BytesN<32>) -> Result<EpidemicModel, Error>
```

> Get public health alert

```rust
pub fn get_public_health_alert(env: Env, alert_id: u64) -> Result<PublicHealthAlert, Error>
```

> Get vaccination coverage

```rust
pub fn get_vaccination_coverage( env: Env, coverage_id: BytesN<32>, ) -> Result<VaccinationCoverage, Error>
```

> Get environmental health data

```rust
pub fn get_environmental_health( env: Env, env_data_id: BytesN<32>, ) -> Result<EnvironmentalHealth, Error>
```

> Get antimicrobial resistance data

```rust
pub fn get_antimicrobial_resistance( env: Env, amr_data_id: BytesN<32>, ) -> Result<AntimicrobialResistance, Error>
```

> Get social determinants of health data

```rust
pub fn get_social_determinants( env: Env, sdoh_data_id: BytesN<32>, ) -> Result<SocialHealthDeterminants, Error>
```

> Get public health intervention

```rust
pub fn get_public_health_intervention( env: Env, intervention_id: BytesN<32>, ) -> Result<PublicHealthIntervention, Error>
```

> Get global health collaboration

```rust
pub fn get_global_collaboration( env: Env, collaboration_id: BytesN<32>, ) -> Result<GlobalHealthCollaboration, Error>
```

> Get privacy budget for address

```rust
pub fn get_privacy_budget(env: Env, user: Address) -> Result<u64, Error>
```

---

## regulatory_compliance

```rust
pub fn initialize(env: Env, admin: Address) -> Result<(), Error>
```

```rust
pub fn set_rule(env: Env, framework: String, rule: ComplianceRule) -> Result<(), Error>
```

```rust
pub fn get_rule(env: Env, framework: String) -> Option<ComplianceRule>
```

```rust
pub fn grant_consent(env: Env, user: Address, action: String) -> Result<(), Error>
```

```rust
pub fn revoke_consent(env: Env, user: Address, action: String) -> Result<(), Error>
```

```rust
pub fn has_consent(env: Env, user: Address, action: String) -> bool
```

```rust
pub fn log_audit(env: Env, user: Address, action: String, details: String)
```

```rust
pub fn get_audit_logs(env: Env, user: Address) -> Result<Vec<AuditLog>, Error>
```

```rust
pub fn invoke_right_to_be_forgotten(env: Env, user: Address) -> Result<(), Error>
```

```rust
pub fn is_forgotten(env: &Env, user: Address) -> bool
```

---

## reputation

```rust
pub fn initialize(env: Env, admin: Address) -> Result<(), Error>
```

```rust
pub fn get_score(env: Env, user: Address) -> i128
```

```rust
pub fn mint(env: Env, user: Address, amount: i128) -> Result<(), Error>
```

```rust
pub fn slash(env: Env, user: Address, amount: i128) -> Result<(), Error>
```

---

## reputation_access_control

```rust
pub fn initialize( env: Env, admin: Address, _reputation_contract: Address, ) -> Result<(), Error>
```

```rust
pub fn set_access_policy( env: Env, admin: Address, resource_type: ResourceType, policy: AccessPolicy, ) -> Result<(), Error>
```

```rust
pub fn check_access( env: Env, provider: Address, resource_type: ResourceType, requested_access: AccessLevel, ) -> Result<bool, Error>
```

```rust
pub fn request_access( env: Env, provider: Address, resource_type: ResourceType, requested_access: AccessLevel, justification: String, ) -> Result<BytesN<32>, Error>
```

```rust
pub fn approve_request(env: Env, admin: Address, request_id: BytesN<32>) -> Result<(), Error>
```

```rust
pub fn deny_request(env: Env, admin: Address, request_id: BytesN<32>) -> Result<(), Error>
```

```rust
pub fn grant_emergency_access( env: Env, admin: Address, provider: Address, _duration_hours: u32, ) -> Result<(), Error>
```

```rust
pub fn revoke_emergency_access( env: Env, admin: Address, provider: Address, ) -> Result<(), Error>
```

```rust
pub fn get_provider_access_level( env: Env, provider: Address, resource_type: ResourceType, ) -> Result<AccessLevel, Error>
```

```rust
pub fn get_provider_requests(env: Env, provider: Address) -> Result<Vec<AccessRequest>, Error>
```

```rust
pub fn set_reputation_threshold( env: Env, admin: Address, resource_type: ResourceType, threshold: u32, ) -> Result<(), Error>
```

---

## reputation_integration

```rust
pub fn initialize( env: Env, admin: Address, base_reputation_contract: Address, healthcare_reputation_contract: Address, ) -> Result<(), Error>
```

```rust
pub fn sync_provider_reputation( env: Env, admin: Address, provider: Address, ) -> Result<i128, Error>
```

```rust
pub fn batch_sync_providers( env: Env, admin: Address, providers: Vec<Address>, ) -> Result<Vec<i128>, Error>
```

```rust
pub fn auto_sync_all_providers(env: Env, admin: Address) -> Result<u32, Error>
```

```rust
pub fn update_score_mapping( env: Env, admin: Address, base_weight: u32, healthcare_weight: u32, adjustment_factor: i32, ) -> Result<(), Error>
```

```rust
pub fn update_sync_settings( env: Env, admin: Address, settings: SyncSettings, ) -> Result<(), Error>
```

```rust
pub fn get_combined_score(env: Env, provider: Address) -> Result<i128, Error>
```

```rust
pub fn get_sync_history( env: Env, provider: Address, limit: u32, ) -> Result<Vec<SyncRecord>, Error>
```

```rust
pub fn trigger_credential_sync(env: Env, provider: Address) -> Result<(), Error>
```

```rust
pub fn trigger_feedback_sync(env: Env, provider: Address) -> Result<(), Error>
```

```rust
pub fn trigger_conduct_sync(env: Env, provider: Address) -> Result<(), Error>
```

---

## runtime_validation

> Initialize the runtime validation system

```rust
pub fn initialize(env: Env, admin: Address) -> Result<(), Error>
```

> Register an invariant check

```rust
pub fn register_invariant( env: Env, admin: Address, check_id: String, description: String, severity: u32, // 1=low, 2=medium, 3=high, 4=critical ) -> Result<(), Error>
```

> Register a state consistency check

```rust
pub fn register_state_check( env: Env, admin: Address, check_id: String, description: String, expected_state: String, ) -> Result<(), Error>
```

> Register a permission check

```rust
pub fn register_permission_check( env: Env, admin: Address, check_id: String, description: String, required_role: String, ) -> Result<(), Error>
```

> Register a resource tracker

```rust
pub fn register_resource_tracker( env: Env, admin: Address, tracker_id: String, resource_type: String, max_allocation: i128, ) -> Result<(), Error>
```

> Report a validation violation

```rust
pub fn report_violation( env: Env, reporter: Address, check_id: String, violation_type: ViolationType, details: String, ) -> Result<u64, Error>
```

> Verify an invariant check

```rust
pub fn verify_invariant( env: Env, check_id: String, current_value: i128, expected_range_min: i128, expected_range_max: i128, ) -> Result<bool, Error>
```

> Verify state consistency

```rust
pub fn verify_state_consistency( env: Env, check_id: String, current_state: String, ) -> Result<bool, Error>
```

> Check permission

```rust
pub fn verify_permission(env: Env, check_id: String, user_role: String) -> Result<bool, Error>
```

> Update resource usage

```rust
pub fn update_resource_usage( env: Env, tracker_id: String, usage_delta: i128, ) -> Result<(), Error>
```

> Get validation report

```rust
pub fn get_violation_report(env: Env, violation_id: u64) -> Result<ValidationReport, Error>
```

> Get total violations

```rust
pub fn get_violation_count(env: Env) -> u64
```

---

## sanitization

> Validates a general-purpose string: non-empty, within `max_len` bytes,
> no null bytes, no ASCII control characters (allows tab/LF/CR).

```rust
pub fn sanitize_string(_env: &Env, input: &String, max_len: u32) -> Result<(), SanitizationError>
```

> Validates a human name: letters (any UTF-8), digits, spaces, hyphens,
> apostrophes, commas, and periods only (ASCII subset).

```rust
pub fn sanitize_name(_env: &Env, input: &String) -> Result<(), SanitizationError>
```

> Validates an email address: single '@', non-empty local and domain parts,
> domain contains at least one '.', all chars from the RFC 5321 allowed set.

```rust
pub fn sanitize_email(_env: &Env, input: &String) -> Result<(), SanitizationError>
```

> Validates an identifier: alphanumeric chars, hyphens, underscores, colons,
> dots, and forward slashes (covers DIDs, slugs, and resource paths).

```rust
pub fn sanitize_id(_env: &Env, input: &String) -> Result<(), SanitizationError>
```

> Validates a URL: printable ASCII only, length within MAX_URL_LEN.

```rust
pub fn sanitize_url(_env: &Env, input: &String) -> Result<(), SanitizationError>
```

---

## secure_enclave

```rust
pub fn initialize(env: Env, admin: Address)
```

```rust
pub fn register_enclave( env: Env, caller: Address, node_id: BytesN<32>, provider: CloudProvider, quote: Bytes, public_key: BytesN<32>, )
```

```rust
pub fn verify_attestation(env: Env, admin: Address, node_id: BytesN<32>, is_valid: bool)
```

```rust
pub fn submit_task( env: Env, submitter: Address, task_id: BytesN<32>, payload_hash: BytesN<32>, require_zk_proof: bool, )
```

```rust
pub fn assign_task(env: Env, admin: Address, task_id: BytesN<32>, node_id: BytesN<32>)
```

```rust
pub fn complete_task( env: Env, node_address: Address, task_id: BytesN<32>, result: Bytes, zk_proof: Option<Bytes>, )
```

```rust
pub fn fallback_to_mpc(env: Env, admin: Address, task_id: BytesN<32>, mpc_manager_id: Address)
```

---

## storage_cleanup

```rust
pub fn initialize(env: Env, admin: Address) -> Result<(), Error>
```

```rust
pub fn set_paused(env: Env, caller: Address, paused: bool) -> Result<(), Error>
```

```rust
pub fn set_retention_config( env: Env, caller: Address, config: RetentionConfig, ) -> Result<(), Error>
```

```rust
pub fn register_credential(env: Env, id: u64, expires_at: u64)
```

```rust
pub fn register_audit_log(env: Env, id: u64, logged_at: u64)
```

```rust
pub fn register_escrow(env: Env, id: u64, settled_at: u64)
```

```rust
pub fn register_consent(env: Env, id: u64, revoked_at: u64)
```

```rust
pub fn register_schedule(env: Env, id: u64, end_at: u64)
```

> Clean up expired items across all categories.
> Returns total number of items removed.

```rust
pub fn cleanup_expired(env: Env, caller: Address, max_items: u32) -> Result<u32, Error>
```

> Preview how many items would be cleaned without removing them.

```rust
pub fn preview_cleanup(env: Env, max_items: u32) -> Result<u32, Error>
```

```rust
pub fn cleanup_credentials(env: Env, caller: Address, max_items: u32) -> Result<u32, Error>
```

```rust
pub fn cleanup_audit_logs(env: Env, caller: Address, max_items: u32) -> Result<u32, Error>
```

```rust
pub fn cleanup_escrows(env: Env, caller: Address, max_items: u32) -> Result<u32, Error>
```

```rust
pub fn cleanup_consents(env: Env, caller: Address, max_items: u32) -> Result<u32, Error>
```

```rust
pub fn cleanup_schedules(env: Env, caller: Address, max_items: u32) -> Result<u32, Error>
```

```rust
pub fn get_cleanup_log(env: Env) -> Vec<CleanupEntry>
```

```rust
pub fn is_paused(env: Env) -> bool
```

---

## sut_token

> Initialize the token contract

```rust
pub fn initialize( env: Env, admin: Address, name: String, symbol: String, decimals: u32, supply_cap: i128, ) -> Result<(), Error>
```

> Get token name

```rust
pub fn name(env: Env) -> Result<String, Error>
```

> Get token symbol

```rust
pub fn symbol(env: Env) -> Result<String, Error>
```

> Get token decimals

```rust
pub fn decimals(env: Env) -> Result<u32, Error>
```

> Get total supply

```rust
pub fn total_supply(env: Env) -> Result<i128, Error>
```

> Get supply cap

```rust
pub fn supply_cap(env: Env) -> Result<i128, Error>
```

> Get balance of an address

```rust
pub fn balance_of(env: Env, account: Address) -> i128
```

> Get allowance between owner and spender

```rust
pub fn allowance(env: Env, owner: Address, spender: Address) -> i128
```

> Transfer tokens

```rust
pub fn transfer(env: Env, from: Address, to: Address, amount: i128) -> Result<(), Error>
```

> Transfer tokens from one address to another (requires allowance)

```rust
pub fn transfer_from( env: Env, spender: Address, from: Address, to: Address, amount: i128, ) -> Result<(), Error>
```

> Approve spender to spend tokens

```rust
pub fn approve(env: Env, owner: Address, spender: Address, amount: i128) -> Result<(), Error>
```

> Mint new tokens (only by minter)

```rust
pub fn mint(env: Env, minter: Address, to: Address, amount: i128) -> Result<(), Error>
```

> Burn tokens (only by minter)

```rust
pub fn burn(env: Env, minter: Address, from: Address, amount: i128) -> Result<(), Error>
```

> Add a new minter (only by admin)

```rust
pub fn add_minter(env: Env, minter: Address) -> Result<(), Error>
```

> Remove a minter (only by admin)

```rust
pub fn remove_minter(env: Env, minter: Address) -> Result<(), Error>
```

> Check if address is a minter

```rust
pub fn is_minter(env: Env, address: Address) -> bool
```

> Create a snapshot for voting/rewards

```rust
pub fn snapshot(env: Env) -> Result<u32, Error>
```

> Get balance at snapshot

```rust
pub fn balance_of_at(env: Env, account: Address, snapshot_id: u32) -> Result<i128, Error>
```

> Get total supply at snapshot

```rust
pub fn total_supply_at(env: Env, snapshot_id: u32) -> Result<i128, Error>
```

---

## timelock

```rust
pub fn initialize(env: Env, admin: Address, delay_seconds: u64) -> Result<(), Error>
```

```rust
pub fn get_config(env: Env) -> Option<TimelockConfig>
```

```rust
pub fn queue(env: Env, id: u64, target: Address, call: BytesN<32>) -> Result<(), Error>
```

```rust
pub fn execute(env: Env, id: u64) -> Result<(), Error>
```

---

## token_sale

_No public entry-points found._

---

## upgrade_manager

```rust
pub fn initialize(env: Env, admin: Address, validators: Vec<Address>) -> Result<(), Error>
```

```rust
pub fn propose_upgrade( env: Env, proposer: Address, target: Address, new_wasm_hash: BytesN<32>, new_version: u32, description: Symbol, is_emergency: bool, ) -> Result<u64, Error>
```

```rust
pub fn approve(env: Env, validator: Address, proposal_id: u64) -> Result<(), Error>
```

```rust
pub fn execute(env: Env, proposal_id: u64) -> Result<(), Error>
```

```rust
pub fn execute_emergency(env: Env, proposal_id: u64) -> Result<(), Error>
```

```rust
pub fn validate_proposal(env: Env, proposal_id: u64) -> Result<UpgradeValidation, Error>
```

---

## upgradeability

```rust
pub fn get_version(env: &Env) -> u32
```

```rust
pub fn set_version(env: &Env, version: u32)
```

```rust
pub fn get_admin(env: &Env) -> Option<Address>
```

```rust
pub fn set_admin(env: &Env, admin: &Address)
```

```rust
pub fn is_frozen(env: &Env) -> bool
```

```rust
pub fn freeze(env: &Env)
```

```rust
pub fn add_history(env: &Env, history: UpgradeHistory)
```

```rust
pub fn get_history(env: &Env) -> Vec<UpgradeHistory>
```

```rust
pub fn set_deprecated_functions(env: &Env, deprecations: &Vec<DeprecatedFunction>)
```

```rust
pub fn get_deprecated_functions(env: &Env) -> Vec<DeprecatedFunction>
```

```rust
pub fn authorize_upgrade(env: &Env) -> Result<Address, UpgradeError>
```

```rust
pub fn execute_upgrade<T: migration::Migratable>( env: &Env, new_wasm_hash: BytesN<32>, new_version: u32, description: Symbol, ) -> Result<(), UpgradeError>
```

```rust
pub fn execute_upgrade_with_deprecations<T: migration::Migratable>( env: &Env, new_wasm_hash: BytesN<32>, new_version: u32, description: Symbol, deprecations: Vec<DeprecatedFunction>, ) -> Result<(), UpgradeError>
```

```rust
pub fn validate_upgrade<T: migration::Migratable>( env: &Env, new_wasm_hash: BytesN<32>, ) -> Result<UpgradeValidation, UpgradeError>
```

```rust
pub fn rollback(env: &Env) -> Result<(), UpgradeError>
```

```rust
pub fn set_deprecated_functions( env: &Env, deprecations: Vec<DeprecatedFunction>, ) -> Result<(), UpgradeError>
```

```rust
pub fn get_deprecated_functions(env: &Env) -> Vec<DeprecatedFunction>
```

```rust
pub fn get_deprecated_function(env: &Env, function: Symbol) -> Option<DeprecatedFunction>
```

```rust
pub fn emit_deprecation_warning(env: &Env, function: Symbol) -> Result<(), UpgradeError>
```

---

## zk_verifier

```rust
pub fn initialize(env: Env, admin: Address, default_ttl: u64) -> Result<(), Error>
```

```rust
pub fn set_default_ttl(env: Env, caller: Address, ttl: u64) -> Result<bool, Error>
```

```rust
pub fn get_default_ttl(env: Env) -> u64
```

```rust
pub fn register_verifying_key( env: Env, caller: Address, vk_hash: BytesN<32>, circuit_id: BytesN<32>, attestor: Address, metadata_hash: BytesN<32>, ) -> Result<u32, Error>
```

```rust
pub fn deactivate_verifying_key( env: Env, caller: Address, version: u32, ) -> Result<bool, Error>
```

```rust
pub fn get_verifying_key(env: Env, version: u32) -> Option<VerifyingKeyConfig>
```

```rust
pub fn get_current_version(env: Env) -> u32
```

```rust
pub fn submit_attestation( env: Env, attestor: Address, vk_version: u32, public_inputs_hash: BytesN<32>, proof_hash: BytesN<32>, verified: bool, ttl: u64, ) -> Result<bool, Error>
```

```rust
pub fn verify_proof( env: Env, vk_version: u32, public_inputs_hash: BytesN<32>, proof: Bytes, ) -> bool
```

```rust
pub fn get_attestation( env: Env, vk_version: u32, public_inputs_hash: BytesN<32>, proof_hash: BytesN<32>, ) -> Option<ProofAttestation>
```

```rust
pub fn compute_proof_hash(env: Env, proof: Bytes) -> BytesN<32>
```

```rust
pub fn mark_nullifier_used(env: Env, nullifier: BytesN<32>) -> bool
```

```rust
pub fn is_nullifier_used(env: Env, nullifier: BytesN<32>) -> bool
```

---

## zkp_registry

```rust
pub fn initialize(env: Env, admin: Address) -> Result<(), Error>
```

```rust
pub fn get_range_proof(env: Env, proof_id: BytesN<32>) -> Result<RangeProof, Error>
```

```rust
pub fn get_circuit_params(env: Env, circuit_id: String) -> Result<ZKPCircuitParams, Error>
```

```rust
pub fn get_gas_stats(env: Env, user: Address) -> Result<u64, Error>
```

---

