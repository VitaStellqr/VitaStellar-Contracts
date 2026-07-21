# VitaStellar Contracts — API Reference

> Auto-generated from contract source code. Do not edit manually.

- **API version**: `1.0.0`
- **Generated**: `2026-07-21T10:18:12.527Z`
- **Contracts documented**: 58

## Table of Contents

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
- [credential_registry](#credential-registry)
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
- [healthcare_reputation](#healthcare-reputation)
- [homomorphic_registry](#homomorphic-registry)
- [identity_registry](#identity-registry)
- [ihe_integration](#ihe-integration)
- [iot_device_management](#iot-device-management)
- [load_testing](#load-testing)
- [medical_consent_nft](#medical-consent-nft)
- [medical_record_backup](#medical-record-backup)
- [medical_record_search](#medical-record-search)
- [medical_records](#medical-records)
- [mpc_manager](#mpc-manager)
- [notification_system](#notification-system)
- [patient_risk_stratification](#patient-risk-stratification)
- [payment_router](#payment-router)
- [pharma_supply_chain](#pharma-supply-chain)
- [predictive_analytics](#predictive-analytics)
- [provider_directory](#provider-directory)
- [public_health_surveillance](#public-health-surveillance)
- [reentrancy_guard](#reentrancy-guard)
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

## anomaly_detection

### Functions

| Function | Parameters | Returns | Description |
|---|---|---|---|
| `initialize` | `env: Env, admin: Address, detector: Address, threshold_bps: u32` | `bool` | — |
| `get_anomaly_record` | `env: Env, anomaly_id: u64` | `Option<AnomalyRecord>` | — |
| `get_config` | `env: Env` | `Option<AnomalyDetectionConfig>` | — |
| `get_stats` | `env: Env` | `DetectionStats` | — |
| `get_anomaly_count_for_patient` | `env: Env, patient: Address` | `u64` | — |
| `is_whitelisted_detector` | `env: Env, detector_addr: Address` | `bool` | — |
| `create_alert` | `env: Env, caller: Address, anomaly_id: u64` | `Result<u64, Error>` | Promote an anomaly record to an active alert for investigation tracking. |
| `acknowledge_alert` | `env: Env, caller: Address, alert_id: u64` | `Result<bool, Error>` | Acknowledge an alert (marks it as under review). |
| `mark_false_positive` | `env: Env, caller: Address, alert_id: u64` | `Result<bool, Error>` | Mark alert as false positive. Feeds adaptive threshold learning. |
| `get_alert` | `env: Env, alert_id: u64` | `Option<AnomalyAlert>` | — |
| `get_alert_count` | `env: Env` | `u64` | — |

### Types

#### `enum AlertStatus`

| Variant | Value | Description |
|---|---|---|
| `Active` | — | — |
| `Acknowledged` | — | — |
| `Resolved` | — | — |
| `FalsePositive` | — | — |

#### `struct AnomalyAlert`

| Field | Type | Description |
|---|---|---|
| `alert_id` | `u64` | — |
| `anomaly_id` | `u64` | — |
| `patient` | `Address` | — |
| `score_bps` | `u32` | — |
| `severity` | `u32` | — |
| `status` | `AlertStatus` | — |
| `created_at` | `u64` | — |
| `updated_at` | `u64` | — |
| `resolution_notes` | `String` | — |

#### `struct AnomalyDetectionConfig`

| Field | Type | Description |
|---|---|---|
| `admin` | `Address` | — |
| `detector` | `Address` | — |
| `threshold_bps` | `u32` | — |
| `sensitivity` | `u32` | — |
| `enabled` | `bool` | — |

#### `struct AnomalyRecord`

| Field | Type | Description |
|---|---|---|
| `record_id` | `u64` | — |
| `patient` | `Address` | — |
| `detector_address` | `Address` | — |
| `score_bps` | `u32` | — |
| `severity` | `u32` | — |
| `detected_at` | `u64` | — |
| `metadata` | `String` | — |
| `explanation_ref` | `String` | — |

#### `struct DetectionStats`

| Field | Type | Description |
|---|---|---|
| `total_anomalies` | `u64` | — |
| `high_severity_count` | `u64` | — |
| `last_detection_at` | `u64` | — |

#### `enum DataKey`

| Variant | Value | Description |
|---|---|---|
| `Config` | — | — |
| `AnomalyRecord` | — | — |
| `u64` | — | — |
| `AnomalyCountByPatient` | — | — |
| `Address` | — | — |
| `Stats` | — | — |
| `Whitelist` | — | — |
| `Address` | — | — |
| `Alert` | — | — |
| `u64` | — | — |
| `AlertCount` | — | — |
| `FeedbackCount` | — | — |
| `AuditForensicsContract` | — | — |

#### `enum Error`

| Variant | Value | Description |
|---|---|---|
| `NotAuthorized` | 1 | — |
| `ConfigNotSet` | 2 | — |
| `Disabled` | 3 | — |
| `InvalidScore` | 4 | — |
| `InvalidSeverity` | 5 | — |
| `RecordNotFound` | 6 | — |
| `NotWhitelisted` | 7 | — |
| `AlertNotFound` | 8 | — |
| `AlertAlreadyResolved` | 9 | — |

---

## anomaly_detector

### Functions

| Function | Parameters | Returns | Description |
|---|---|---|---|
| `initialize` | `env: Env, admin: Address` | `Result<bool, Error>` | — |
| `add_validator` | `env: Env, caller: Address, validator: Address` | `Result<bool, Error>` | — |
| `remove_validator` | `env: Env, caller: Address, validator: Address` | `Result<bool, Error>` | — |
| `pause` | `env: Env, caller: Address` | `Result<bool, Error>` | — |
| `unpause` | `env: Env, caller: Address` | `Result<bool, Error>` | — |
| `clear_alerts` | `env: Env, caller: Address, count: u64` | `Result<u64, Error>` | Clear active alerts up to `count` (admin only). Pass 0 to clear all. Marks each active alert as Resolved and emits a ClearAlerts event. |
| `acknowledge_alert` | `env: Env, caller: Address, alert_id: u64` | `Result<bool, Error>` | Acknowledge an active alert (marks as reviewed, does not close). |
| `mark_false_positive` | `env: Env, caller: Address, alert_id: u64` | `Result<bool, Error>` | Mark an alert as false positive, automatically feeding adaptive learning. |
| `get_alert` | `env: Env, alert_id: u64` | `Option<Alert>` | — |
| `get_model` | `env: Env, model_id: BytesN<32>` | `Option<AnomalyModel>` | — |
| `get_model_weights` | `env: Env, model_id: BytesN<32>` | `Option<Vec<u32>>` | — |
| `get_patient_profile` | `env: Env, patient: Address` | `Option<PatientRiskProfile>` | — |
| `get_alert_count` | `env: Env` | `u64` | — |
| `get_feedback` | `env: Env, feedback_id: u64` | `Option<ModelFeedback>` | — |
| `is_paused` | `env: Env` | `bool` | — |
| `is_validator` | `env: Env, addr: Address` | `bool` | — |

### Types

#### `enum AlertLevel`

| Variant | Value | Description |
|---|---|---|
| `Low` | — | — |
| `Medium` | — | — |
| `High` | — | — |
| `Critical` | — | — |

#### `enum AlertStatus`

| Variant | Value | Description |
|---|---|---|
| `Active` | — | — |
| `Acknowledged` | — | — |
| `Resolved` | — | — |
| `FalsePositive` | — | — |

#### `enum HealthcarePatternType`

| Variant | Value | Description |
|---|---|---|
| `Accessing` | — | — |
| `too` | — | — |
| `many` | — | — |
| `records` | — | — |
| `in` | — | — |
| `a` | — | — |
| `short` | — | — |
| `time` | — | — |
| `window` | — | — |
| `BulkRecordAccess` | — | — |
| `Access` | — | — |
| `outside` | — | — |
| `normal` | — | — |
| `business` | — | — |
| `hours` | — | — |
| `UnusualTimeAccess` | — | — |
| `Unusual` | — | — |
| `prescription` | — | — |
| `volume` | — | — |
| `or` | — | — |
| `high` | — | — |
| `risk` | — | — |
| `drug` | — | — |
| `ratio` | — | — |
| `PrescriptionAnomaly` | — | — |
| `Accessing` | — | — |
| `records` | — | — |
| `outside` | — | — |
| `practitioner` | — | — |
| `specialty` | — | — |
| `scope` | — | — |
| `UnauthorizedSpecialtyAccess` | — | — |
| `Very` | — | — |
| `rapid` | — | — |
| `sequential` | — | — |
| `access` | — | — |
| `to` | — | — |
| `records` | — | — |
| `RapidSequentialAccess` | — | — |
| `Attempted` | — | — |
| `bulk` | — | — |
| `export` | — | — |
| `of` | — | — |
| `records` | — | — |
| `SuspiciousExport` | — | — |
| `Generic` | — | — |
| `ML` | — | — |
| `scored` | — | — |
| `anomaly` | — | — |
| `no` | — | — |
| `specific` | — | — |
| `pattern` | — | — |
| `matched` | — | — |
| `MlScored` | — | — |

#### `struct FeatureContribution`

| Field | Type | Description |
|---|---|---|
| `feature_index` | `u32` | — |
| `feature_name` | `String` | — |
| `feature_value` | `u32` | — |
| `weight` | `u32` | — |
| `contribution` | `u32` | — |

#### `struct DetectionResult`

| Field | Type | Description |
|---|---|---|
| `anomaly_score` | `u32` | — |
| `is_anomalous` | `bool` | — |
| `confidence` | `u32` | — |
| `alert_level` | `AlertLevel` | — |
| `pattern_type` | `HealthcarePatternType` | — |
| `top_features` | `Vec<FeatureContribution>` | — |
| `explanation_summary` | `String` | — |
| `detected_at` | `u64` | — |

#### `struct AnomalyModel`

| Field | Type | Description |
|---|---|---|
| `model_id` | `BytesN<32>` | — |
| `name` | `String` | — |
| `feature_count` | `u32` | — |
| `threshold_bps` | `u32` | — |
| `version` | `u32` | — |
| `total_inferences` | `u64` | — |
| `confirmed_anomalies` | `u64` | — |
| `false_positives` | `u64` | — |
| `created_at` | `u64` | — |
| `updated_at` | `u64` | — |

#### `struct Alert`

| Field | Type | Description |
|---|---|---|
| `alert_id` | `u64` | — |
| `patient` | `Address` | — |
| `triggered_by` | `Address` | — |
| `model_id` | `BytesN<32>` | — |
| `anomaly_score` | `u32` | — |
| `alert_level` | `AlertLevel` | — |
| `status` | `AlertStatus` | — |
| `pattern_type` | `HealthcarePatternType` | — |
| `explanation_summary` | `String` | — |
| `metadata` | `String` | — |
| `created_at` | `u64` | — |
| `updated_at` | `u64` | — |

#### `struct ModelFeedback`

| Field | Type | Description |
|---|---|---|
| `feedback_id` | `u64` | — |
| `alert_id` | `u64` | — |
| `model_id` | `BytesN<32>` | — |
| `submitted_by` | `Address` | — |
| `confirmed` | `bool` | — |
| `submitted_at` | `u64` | — |

#### `struct FederatedUpdate`

| Field | Type | Description |
|---|---|---|
| `round_id` | `u64` | — |
| `participant` | `Address` | — |
| `update_hash` | `BytesN<32>` | — |
| `num_samples` | `u32` | — |
| `submitted_at` | `u64` | — |

#### `struct PatientRiskProfile`

| Field | Type | Description |
|---|---|---|
| `patient` | `Address` | — |
| `rolling_risk_score` | `u32` | — |
| `total_alerts` | `u64` | — |
| `active_alerts` | `u64` | — |
| `false_positive_count` | `u64` | — |
| `last_alert_at` | `u64` | — |

#### `enum DataKey`

| Variant | Value | Description |
|---|---|---|
| `Admin` | — | — |
| `Paused` | — | — |
| `AlertCount` | — | — |
| `FeedbackCount` | — | — |
| `Model` | — | — |
| `weights` | — | — |
| `stored` | — | — |
| `separately` | — | — |
| `from` | — | — |
| `metadata` | — | — |
| `to` | — | — |
| `keep` | — | — |
| `structs` | — | — |
| `small` | — | — |
| `ModelWeights` | — | — |
| `BytesN` | — | — |
| `32` | — | — |
| `Model` | — | — |
| `BytesN` | — | — |
| `32` | — | — |
| `Alert` | — | — |
| `u64` | — | — |
| `Feedback` | — | — |
| `u64` | — | — |
| `FederatedUpdate` | — | — |
| `u64` | — | — |
| `Address` | — | — |
| `PatientProfile` | — | — |
| `Address` | — | — |
| `Validator` | — | — |
| `Address` | — | — |

#### `enum Error`

| Variant | Value | Description |
|---|---|---|
| `AlreadyInitialized` | 1 | — |
| `NotInitialized` | 2 | — |
| `NotAuthorized` | 3 | — |
| `ContractPaused` | 4 | — |
| `ModelNotFound` | 5 | — |
| `AlertNotFound` | 6 | — |
| `FeatureCountMismatch` | 7 | — |
| `InvalidWeight` | 8 | — |
| `InvalidThreshold` | 9 | — |
| `AlertAlreadyResolved` | 10 | — |
| `DuplicateFederatedUpdate` | 11 | — |
| `InvalidFeatureCount` | 12 | — |
| `InvalidScore` | 13 | — |

### Examples

#### `test_initialize`

```rust
let env = Env::default();
    let (client, _admin) = setup(&env);
    assert!(!client.is_paused());
    assert_eq!(client.get_alert_count(), 0);
```

#### `test_initialize_twice_fails`

```rust
let env = Env::default();
    let (client, admin) = setup(&env);
    env.mock_all_auths();
    let result = client.try_initialize(&admin);
    assert_eq!(result, Err(Ok(Error::AlreadyInitialized)));
```

#### `test_add_and_remove_validator`

```rust
let env = Env::default();
    let (client, admin) = setup(&env);
    let validator = Address::generate(&env);

    env.mock_all_auths();
    assert!(!client.is_validator(&validator));

    client.add_validator(&admin, &validator);
    assert!(client.is_validator(&validator));
```

---

## appointment_booking_escrow

### Functions

| Function | Parameters | Returns | Description |
|---|---|---|---|
| `initialize` | `env: Env, admin: Address, _token: Address` | `Result<(), Error>` | Initialize the contract with an admin and token address |
| `mark_no_show` | `env: Env, provider: Address, appointment_id: u64` | `Result<(), Error>` | Mark an appointment as a no-show (provider only). Only callable after the scheduled appointment time. Releases the escrowed funds to the provider on no-show. |
| `send_reminder` | `env: Env, caller: Address, appointment_id: u64` | `Result<(), Error>` | Send an appointment reminder (provider or admin only). Records the timestamp when the reminder was last sent. |
| `get_appointment` | `env: Env, appointment_id: u64` | `Option<AppointmentEscrow>` | Get appointment details |
| `get_patient_appointments` | `env: Env, patient: Address` | `Vec<u64>` | Get all appointments for a patient |
| `get_provider_appointments` | `env: Env, provider: Address` | `Vec<u64>` | Get all appointments for a provider |
| `get_escrow_balance` | `env: Env` | `i128` | Get escrow balance (should be equal to sum of all booked but not confirmed/refunded appointments) |
| `get_admin` | `env: Env` | `Result<Address, Error>` | Get the current admin |
| `health_check` | `env: Env` | `ContractHealth` | Get comprehensive health check |
| `set_paused` | `env: Env, admin: Address, paused: bool` | `Result<(), Error>` | Set pause status (admin only) |
| `is_paused` | `env: Env` | `bool` | Check if contract is paused |

### Types

#### `enum AppointmentStatus`

| Variant | Value | Description |
|---|---|---|
| `Booked` | 0 | — |
| `Confirmed` | 1 | — |
| `Refunded` | 2 | — |
| `Completed` | 3 | — |
| `NoShow` | 4 | — |

#### `struct AppointmentEscrow`

| Field | Type | Description |
|---|---|---|
| `appointment_id` | `u64` | — |
| `patient` | `Address` | — |
| `provider` | `Address` | — |
| `amount` | `i128` | — |
| `token` | `Address` | — |
| `booked_at` | `u64` | — |
| `scheduled_time` | `u64` | — |
| `confirmed_at` | `u64` | — |
| `refunded_at` | `u64` | — |
| `reminder_sent_at` | `u64` | — |
| `no_show_marked_at` | `u64` | — |
| `status` | `AppointmentStatus` | — |
| `funds_released` | `bool` | — |

#### `enum DataKey`

| Variant | Value | Description |
|---|---|---|
| `Initialized` | — | — |
| `Admin` | — | — |
| `AppointmentCounter` | — | — |
| `Appointment` | — | — |
| `u64` | — | — |
| `appointment_id` | — | — |
| `AppointmentEscrow` | — | — |
| `PatientAppointments` | — | — |
| `Address` | — | — |
| `patient` | — | — |
| `Vec` | — | — |
| `u64` | — | — |
| `ProviderAppointments` | — | — |
| `Address` | — | — |
| `provider` | — | — |
| `Vec` | — | — |
| `u64` | — | — |
| `Paused` | — | — |
| `LastActivity` | — | — |
| `TotalOperations` | — | — |
| `FailedOperations` | — | — |
| `Version` | — | — |

#### `struct ContractHealth`

| Field | Type | Description |
|---|---|---|
| `version` | `String` | — |
| `is_paused` | `bool` | — |
| `storage_usage` | `u64` | — |
| `last_activity` | `u64` | — |
| `total_operations` | `u64` | — |
| `failed_operations` | `u64` | — |
| `success_rate` | `u32` | — |
| `total_appointments` | `u64` | — |
| `active_escrow_balance` | `i128` | — |

### Error Codes

| Variant | Code | Description |
|---|---|---|
| `Unauthorized` | 100 | — |
| `OnlyPatientCanRefund` | 110 | — |
| `OnlyProviderCanConfirm` | 111 | — |
| `InvalidAmount` | 205 | — |
| `InvalidPatient` | 210 | — |
| `InvalidProvider` | 211 | — |
| `NotInitialized` | 300 | — |
| `AlreadyInitialized` | 301 | — |
| `InvalidState` | 304 | — |
| `AppointmentNotFound` | 410 | — |
| `AppointmentAlreadyConfirmed` | 411 | — |
| `AppointmentAlreadyRefunded` | 412 | — |
| `AppointmentNoShow` | 413 | — |
| `InsufficientFunds` | 500 | — |
| `TokenTransferFailed` | 501 | — |
| `DoubleWithdrawal` | 505 | — |

### Examples

#### `test_initialize`

```rust
let (_env, client, admin, token_id) = setup();
        client.initialize(&admin, &token_id);
    }

    #[test]
    fn test_initialize_twice_fails() {
        let (_env, client, admin, token_id) = setup();
        client.initialize(&admin, &token_id);
        let result = client.try_initialize(&admin, &token_id);
```

---

## audit_forensics

### Functions

| Function | Parameters | Returns | Description |
|---|---|---|---|
| `initialize` | `env: Env, admin: Address` | `()` | — |
| `get_execution` | `env: Env, execution_id: u64` | `Option<AnalysisExecution>` | — |
| `get_finding` | `env: Env, finding_id: u64` | `Option<VulnerabilityFinding>` | — |
| `get_findings_by_execution` | `env: Env, execution_id: u64` | `Vec<VulnerabilityFinding>` | — |
| `generate_remediation_plan` | `env: Env, execution_id: u64` | `Vec<String>` | — |
| `analyze_timeline` | `env: Env, record_id: u64` | `Vec<AuditEntry>` | — |
| `investigate_user` | `env: Env, user: Address` | `Vec<AuditEntry>` | — |
| `set_alert_threshold` | `env: Env, admin: Address, action: AuditAction, threshold: u32` | `()` | — |
| `compress_logs` | `env: Env, admin: Address, before_timestamp: u64` | `BytesN<32>` | — |
| `archive_logs` | `env: Env, admin: Address, archive_ref: String` | `()` | — |

### Types

#### `enum AuditAction`

| Variant | Value | Description |
|---|---|---|
| `RecordAccess` | — | — |
| `RecordUpdate` | — | — |
| `RecordDelete` | — | — |
| `PermissionGrant` | — | — |
| `PermissionRevoke` | — | — |
| `RecordCreated` | — | — |
| `AnomalyDetected` | — | — |
| `ComplianceReportGenerated` | — | — |
| `AlertTriggered` | — | — |

#### `struct AuditEntry`

| Field | Type | Description |
|---|---|---|
| `id` | `u64` | — |
| `timestamp` | `u64` | — |
| `actor` | `Address` | — |
| `action` | `AuditAction` | — |
| `record_id` | `Option<u64>` | — |
| `details_hash` | `BytesN<32>` | — |
| `metadata` | `Map<String` | — |

#### `struct ForensicReport`

| Field | Type | Description |
|---|---|---|
| `target_id` | `u64` | — |
| `entries` | `Vec<AuditEntry>` | — |
| `summary` | `String` | — |
| `detected_anomalies` | `Vec<u64>` | — |

#### `struct AuditRule`

| Field | Type | Description |
|---|---|---|
| `id` | `u64` | — |
| `name` | `String` | — |
| `applies_to_language` | `String` | — |
| `severity_bps` | `u32` | — |
| `enabled` | `bool` | — |
| `pattern_ref` | `String` | — |
| `remediation` | `String` | — |

#### `struct VulnerabilityFinding`

| Field | Type | Description |
|---|---|---|
| `id` | `u64` | — |
| `execution_id` | `u64` | — |
| `rule_id` | `u64` | — |
| `contract_hash` | `BytesN<32>` | — |
| `title` | `String` | — |
| `severity_bps` | `u32` | — |
| `confidence_bps` | `u32` | — |
| `language` | `String` | — |
| `analysis_mode` | `String` | — |
| `remediation` | `String` | — |
| `detected_at` | `u64` | — |

#### `struct AnalysisExecution`

| Field | Type | Description |
|---|---|---|
| `id` | `u64` | — |
| `contract_hash` | `BytesN<32>` | — |
| `language` | `String` | — |
| `analysis_mode` | `String` | — |
| `finding_ids` | `Vec<u64>` | — |
| `started_at` | `u64` | — |
| `completed_at` | `u64` | — |
| `duration_minutes` | `u32` | — |
| `passed` | `bool` | — |

#### `struct FormalVerificationSummary`

| Field | Type | Description |
|---|---|---|
| `execution_id` | `u64` | — |
| `property_name` | `String` | — |
| `proved` | `bool` | — |
| `proof_ref` | `String` | — |
| `checked_at` | `u64` | — |

#### `enum DataKey`

| Variant | Value | Description |
|---|---|---|
| `Admin` | — | — |
| `NextAuditId` | — | — |
| `AuditEntry` | — | — |
| `u64` | — | — |
| `UserAudits` | — | — |
| `Address` | — | — |
| `RecordAudits` | — | — |
| `u64` | — | — |
| `AlertThresholds` | — | — |
| `Symbol` | — | — |
| `NextRuleId` | — | — |
| `Rule` | — | — |
| `u64` | — | — |
| `NextExecutionId` | — | — |
| `Execution` | — | — |
| `u64` | — | — |
| `NextFindingId` | — | — |
| `Finding` | — | — |
| `u64` | — | — |
| `FindingsByExecution` | — | — |
| `u64` | — | — |
| `FormalSummary` | — | — |
| `u64` | — | — |

### Examples

#### `test_audit_flow`

```rust
let env = Env::default();
        let contract_id = env.register_contract(None, AuditForensicsContract);
        let client = AuditForensicsContractClient::new(&env, &contract_id);

        let admin = Address::generate(&env);
        client.initialize(&admin);

        let doctor = Address::generate(&env);
        let record_id = 101u64;
```

---

## code_ownership

### Functions

| Function | Parameters | Returns | Description |
|---|---|---|---|
| `initialize` | `env: Env, admin: Address` | `Result<(), Error>` | Initialize the code ownership tracking system |
| `get_module_ownership` | `env: Env, module_id: String` | `Result<ModuleOwnership, Error>` | Get module ownership information |
| `get_review_route` | `env: Env, module_id: String` | `Result<ReviewRoute, Error>` | Get review routing for a module |
| `get_expertise_matrix` | `env: Env` | `OwnershipMatrix` | Get expertise matrix for all modules |
| `is_module_owner` | `env: Env, module_id: String, address: Address` | `Result<bool, Error>` | Check if an address is an owner of a module |
| `get_owned_modules` | `env: Env, owner: Address` | `Vec<String>` | Get all modules owned by an address |

### Error Codes

| Variant | Code | Description |
|---|---|---|
| `NotInitialized` | 1 | — |
| `AlreadyInitialized` | 2 | — |
| `NotAuthorized` | 3 | — |
| `ModuleNotFound` | 4 | — |
| `ModuleAlreadyExists` | 5 | — |
| `ReviewRouteNotFound` | 6 | — |
| `InvalidOwnerCount` | 7 | — |

### Examples

#### `test_initialize`

```rust
let env = Env::default();
        env.mock_all_auths();

        let admin = Address::generate(&env);
        let contract_id = env.register_contract(None, CodeOwnership);
        let client = CodeOwnershipClient::new(&env, &contract_id);

        client.initialize(&admin);
    }
```

---

## common_error

### Functions

| Function | Parameters | Returns | Description |
|---|---|---|---|
| `get_suggestion` | `error: CommonError` | `Symbol` | — |

### Types

#### `enum CommonError`

| Variant | Value | Description |
|---|---|---|
| `Unknown` | 0 | — |
| `Unauthorized` | 1 | — |
| `NotInitialized` | 2 | — |
| `AlreadyInitialized` | 3 | — |
| `ContractPaused` | 4 | — |
| `DeadlineExceeded` | 5 | — |
| `RateLimitExceeded` | 6 | — |
| `InsufficientFunds` | 7 | — |
| `InvalidInput` | 8 | — |
| `InvalidState` | 9 | — |
| `NotFound` | 10 | — |
| `AccessDenied` | 11 | — |
| `Timeout` | 12 | — |
| `InvalidArgument` | 13 | — |
| `ExternalContractNotSet` | 14 | — |
| `InvalidData` | 15 | — |
| `InvalidPayload` | 16 | — |
| `DuplicateSubmission` | 17 | — |
| `UnauthorizedCaller` | 18 | — |

---

## contract_monitoring

### Functions

| Function | Parameters | Returns | Description |
|---|---|---|---|
| `record_error` | `env: Env, function_name: String` | `Result<(), MonitoringError>` | Record a failed function call / error. |
| `update_storage_count` | `env: Env, count: u32` | `Result<(), MonitoringError>` | Update storage-entry count (call after writes to tracked contracts). |
| `update_alert_config` | `env: Env, config: AlertConfig` | `Result<(), MonitoringError>` | Update alert thresholds (admin only). |
| `get_dashboard` | `env: Env` | `Result<DashboardSnapshot, MonitoringError>` | Return a full dashboard snapshot. |

### Types

#### `struct AlertConfig`

| Field | Type | Description |
|---|---|---|
| `max_error_rate_pct` | `u32` | — |
| `max_gas_per_window` | `u64` | — |
| `max_storage_entries` | `u32` | — |

#### `struct FunctionStats`

| Field | Type | Description |
|---|---|---|
| `call_count` | `u64` | — |
| `error_count` | `u64` | — |
| `total_gas` | `u64` | — |
| `last_called_at` | `u64` | — |

#### `struct DashboardSnapshot`

| Field | Type | Description |
|---|---|---|
| `total_calls` | `u64` | — |
| `total_errors` | `u64` | — |
| `error_rate_pct` | `u32` | — |
| `total_gas_used` | `u64` | — |
| `active_users` | `u32` | — |
| `storage_entries` | `u32` | — |
| `snapshot_at` | `u64` | — |
| `alert_active` | `bool` | — |

#### `enum DataKey`

| Variant | Value | Description |
|---|---|---|
| `Admin` | — | — |
| `AlertConfig` | — | — |
| `TotalCalls` | — | — |
| `TotalErrors` | — | — |
| `TotalGas` | — | — |
| `ActiveUsers` | — | — |
| `StorageEntries` | — | — |
| `Per` | — | — |
| `function` | — | — |
| `stats` | — | — |
| `keyed` | — | — |
| `by` | — | — |
| `function` | — | — |
| `name` | — | — |
| `FnStats` | — | — |
| `String` | — | — |
| `Tracks` | — | — |
| `whether` | — | — |
| `an` | — | — |
| `address` | — | — |
| `has` | — | — |
| `been` | — | — |
| `seen` | — | — |
| `before` | — | — |
| `SeenUser` | — | — |
| `Address` | — | — |

#### `enum MonitoringError`

| Variant | Value | Description |
|---|---|---|
| `NotInitialized` | 1 | — |
| `AlreadyInitialized` | 2 | — |
| `Unauthorized` | 3 | — |

---

## contract_template

### Functions

| Function | Parameters | Returns | Description |
|---|---|---|---|
| `initialize` | `env: Env, admin: Address` | `Result<(), Error>` | Initialize the contract. Can only be called once.  # Auth No auth required — the deployer becomes the admin. |
| `transfer_admin` | `env: Env, new_admin: Address` | `Result<(), Error>` | Transfer admin rights to a new address.  # Auth Requires auth from the **current** admin. |
| `update_data` | `env: Env, caller: Address, data: String` | `Result<(), Error>` | Update the contract's stored data.  # Auth Requires auth from the admin. |
| `get_admin` | `env: &Env` | `Result<Address, Error>` | Return the current admin address. |
| `get_data` | `env: Env` | `Option<ContractData>` | Return the stored data, if any. |

### Error Codes

| Variant | Code | Description |
|---|---|---|
| `NotInitialized` | 1 | — |
| `AlreadyInitialized` | 2 | — |
| `Unauthorized` | 3 | — |
| `InputTooLong` | 4 | — |
| `ReentrantCall` | 5 | — |

### Examples

#### `test_initialize`

```rust
let (_, _, client) = setup();
    let admin2 = Address::generate(&client.env);
    assert_eq!(
        client.try_initialize(&admin2),
        Err(Ok(Error::AlreadyInitialized))
    );
```

#### `test_update_data_as_admin`

```rust
let (env, admin, client) = setup();
    let data = String::from_str(&env, "hello");
    assert!(client.try_update_data(&admin, &data).is_ok());
    let stored = client.get_data().unwrap();
    assert_eq!(stored.value, data);
```

#### `test_update_data_unauthorized`

```rust
let (env, _, client) = setup();
    let other = Address::generate(&env);
    let data = String::from_str(&env, "hack");
    assert_eq!(
        client.try_update_data(&other, &data),
        Err(Ok(Error::Unauthorized))
    );
```

---

## contract_usage_analytics

### Functions

| Function | Parameters | Returns | Description |
|---|---|---|---|
| `initialize` | `env: Env, admin: Address` | `Result<(), Error>` | — |
| `take_snapshot` | `env: Env` | `Result<UsageSnapshot, Error>` | — |
| `get_function_metrics` | `env: Env, function_name: String` | `Option<FunctionMetric>` | — |
| `get_user_metrics` | `env: Env, user: Address` | `Option<UserMetric>` | — |
| `get_all_functions` | `env: Env` | `Vec<String>` | — |
| `get_snapshots` | `env: Env` | `Vec<UsageSnapshot>` | — |

### Types

#### `struct FunctionMetric`

| Field | Type | Description |
|---|---|---|
| `name` | `String` | — |
| `call_count` | `u64` | — |
| `total_cpu_usage` | `u64` | — |
| `total_ram_usage` | `u64` | — |
| `error_count` | `u64` | — |
| `avg_latency_ms` | `u64` | — |
| `last_called` | `u64` | — |

#### `struct UserMetric`

| Field | Type | Description |
|---|---|---|
| `user` | `Address` | — |
| `total_calls` | `u64` | — |
| `last_active` | `u64` | — |

#### `struct UsageSnapshot`

| Field | Type | Description |
|---|---|---|
| `timestamp` | `u64` | — |
| `total_calls` | `u64` | — |
| `active_users` | `u32` | — |
| `error_rate_bps` | `u32` | — |

#### `enum DataKey`

| Variant | Value | Description |
|---|---|---|
| `Admin` | — | — |
| `FunctionMetric` | — | — |
| `String` | — | — |
| `UserMetric` | — | — |
| `Address` | — | — |
| `Snapshots` | — | — |
| `AllFunctions` | — | — |
| `ActiveUsers` | — | — |
| `u64` | — | — |
| `Day` | — | — |
| `based` | — | — |
| `key` | — | — |
| `for` | — | — |
| `active` | — | — |
| `users` | — | — |

#### `enum Error`

| Variant | Value | Description |
|---|---|---|
| `NotAuthorized` | 1 | — |
| `AlreadyInitialized` | 2 | — |
| `NotInitialized` | 3 | — |
| `InvalidInput` | 4 | — |

---

## contract_verification

### Functions

| Function | Parameters | Returns | Description |
|---|---|---|---|
| `initialize` | `env: Env, admin: Address` | `Result<(), VerificationError>` | Initialise the verification registry with an admin address. |
| `publish_abi` | `env: Env, entries: Vec<AbiEntry>` | `Result<(), VerificationError>` | Publish the ABI for all public functions. |
| `mark_verified` | `env: Env` | `Result<(), VerificationError>` | Mark the contract as fully verified (metadata + build + ABI all present). |
| `get_metadata` | `env: Env` | `Result<ContractMetadata, VerificationError>` | — |
| `get_build_info` | `env: Env` | `Result<BuildInfo, VerificationError>` | — |
| `get_abi` | `env: Env` | `Result<Vec<AbiEntry>, VerificationError>` | — |
| `is_verified` | `env: Env` | `bool` | — |

### Types

#### `struct ContractMetadata`

| Field | Type | Description |
|---|---|---|
| `name` | `String` | — |
| `version` | `String` | — |
| `source_url` | `String` | — |
| `license` | `String` | — |
| `description` | `String` | — |
| `published_at` | `u64` | — |
| `publisher` | `Address` | — |

#### `struct BuildInfo`

| Field | Type | Description |
|---|---|---|
| `rust_version` | `String` | — |
| `sdk_version` | `String` | — |
| `build_profile` | `String` | — |
| `wasm_hash` | `BytesN<32>` | — |
| `commit_sha` | `String` | — |

#### `struct AbiEntry`

| Field | Type | Description |
|---|---|---|
| `name` | `String` | — |
| `params` | `String` | — |
| `returns` | `String` | — |
| `doc` | `String` | — |

#### `enum DataKey`

| Variant | Value | Description |
|---|---|---|
| `Admin` | — | — |
| `Metadata` | — | — |
| `BuildInfo` | — | — |
| `AbiEntries` | — | — |
| `IsVerified` | — | — |

#### `enum VerificationError`

| Variant | Value | Description |
|---|---|---|
| `NotInitialized` | 1 | — |
| `AlreadyInitialized` | 2 | — |
| `Unauthorized` | 3 | — |
| `MetadataNotFound` | 4 | — |

---

## credential_registry

### Functions

| Function | Parameters | Returns | Description |
|---|---|---|---|
| `initialize` | `env: Env, admin: Address` | `Result<(), Error>` | — |
| `get_issuer_admin` | `env: Env, issuer: Address` | `Option<Address>` | — |
| `get_active_root` | `env: Env, issuer: Address` | `Option<BytesN<32>>` | — |
| `get_active_version` | `env: Env, issuer: Address` | `u32` | — |
| `get_root` | `env: Env, issuer: Address, version: u32` | `Option<CredentialRootRecord>` | — |
| `get_revocation_root` | `env: Env, issuer: Address` | `Option<BytesN<32>>` | — |
| `is_root_revoked` | `env: Env, issuer: Address, root: BytesN<32>` | `bool` | — |
| `has_active_root` | `env: Env, issuer: Address` | `bool` | — |

### Types

#### `struct CredentialRootRecord`

| Field | Type | Description |
|---|---|---|
| `version` | `u32` | — |
| `root` | `BytesN<32>` | — |
| `metadata_hash` | `BytesN<32>` | — |
| `updated_at` | `u64` | — |
| `expiry` | `u64` | — |
| `signature` | `BytesN<64>` | — |
| `revoked` | `bool` | — |

#### `enum DataKey`

| Variant | Value | Description |
|---|---|---|
| `Initialized` | — | — |
| `Admin` | — | — |
| `IssuerAdmin` | — | — |
| `Address` | — | — |
| `ActiveVersion` | — | — |
| `Address` | — | — |
| `ActiveRoot` | — | — |
| `Address` | — | — |
| `RootRecord` | — | — |
| `Address` | — | — |
| `u32` | — | — |
| `RevocationRoot` | — | — |
| `Address` | — | — |
| `RootToVersion` | — | — |
| `Address` | — | — |
| `BytesN` | — | — |
| `32` | — | — |
| `index` | — | — |
| `root` | — | — |
| `bytes` | — | — |
| `version` | — | — |
| `number` | — | — |

#### `enum Error`

| Variant | Value | Description |
|---|---|---|
| `AlreadyInitialized` | 1 | — |
| `NotInitialized` | 2 | — |
| `NotAuthorized` | 3 | — |
| `IssuerNotFound` | 4 | — |
| `RootVersionNotFound` | 5 | — |
| `InvalidCredentialId` | 6 | — |
| `InvalidExpiry` | 7 | — |
| `InvalidMetadata` | 8 | — |
| `InvalidSignature` | 9 | — |

---

## cross_chain_access

### Functions

| Function | Parameters | Returns | Description |
|---|---|---|---|
| `revoke_access` | `env: Env, caller: Address, grant_id: u64` | `Result<bool, Error>` | — |
| `cancel_access_swap` | `env: Env, caller: Address, swap_id: u64` | `Result<bool, Error>` | Cancel a proposed swap (only initiator or after timelock expiry) |
| `get_grant` | `env: Env, grant_id: u64` | `Option<AccessGrant>` | — |
| `get_request` | `env: Env, request_id: u64` | `Option<AccessRequest>` | — |
| `get_delegation` | `env: Env, delegator: Address, delegate: Address` | `Option<Delegation>` | — |
| `get_emergency_config` | `env: Env, patient: Address` | `Option<EmergencyConfig>` | — |
| `get_audit_entry` | `env: Env, entry_id: u64` | `Option<AuditEntry>` | — |
| `get_swap` | `env: Env, swap_id: u64` | `Option<SwapProposal>` | — |
| `is_paused` | `env: Env` | `bool` | — |
| `pause` | `env: Env, caller: Address` | `Result<bool, Error>` | — |
| `unpause` | `env: Env, caller: Address` | `Result<bool, Error>` | — |

### Types

#### `enum PermissionLevel`

| Variant | Value | Description |
|---|---|---|
| `None` | — | — |
| `Read` | — | — |
| `ReadConfidential` | — | — |
| `Write` | — | — |
| `Admin` | — | — |

#### `enum ChainId`

| Variant | Value | Description |
|---|---|---|
| `None` | — | — |
| `Stellar` | — | — |
| `Ethereum` | — | — |
| `Polygon` | — | — |
| `Avalanche` | — | — |
| `BinanceSmartChain` | — | — |
| `Arbitrum` | — | — |
| `Optimism` | — | — |
| `Custom` | — | — |
| `u32` | — | — |

#### `struct AccessGrant`

| Field | Type | Description |
|---|---|---|
| `grant_id` | `u64` | — |
| `grantor` | `Address` | — |
| `grantee_chain` | `ChainId` | — |
| `grantee_address` | `String` | — |
| `permission_level` | `PermissionLevel` | — |
| `record_scope` | `AccessScope` | — |
| `granted_at` | `u64` | — |
| `expires_at` | `u64` | — |
| `is_active` | `bool` | — |
| `conditions` | `Vec<AccessCondition>` | — |

#### `enum AccessScope`

| Variant | Value | Description |
|---|---|---|
| `AllRecords` | — | — |
| `SpecificRecords` | — | — |
| `Vec` | — | — |
| `u64` | — | — |
| `CategoryBased` | — | — |
| `String` | — | — |
| `TimeRanged` | — | — |
| `u64` | — | — |
| `u64` | — | — |

#### `enum AccessCondition`

| Variant | Value | Description |
|---|---|---|
| `EmergencyOnly` | — | — |
| `RequireConsent` | — | — |
| `AuditRequired` | — | — |
| `SingleUse` | — | — |
| `TimeRestricted` | — | — |
| `u64` | — | — |
| `u64` | — | — |

#### `struct AccessRequest`

| Field | Type | Description |
|---|---|---|
| `request_id` | `u64` | — |
| `requester_chain` | `ChainId` | — |
| `requester_address` | `String` | — |
| `patient` | `Address` | — |
| `requested_records` | `Vec<u64>` | — |
| `purpose` | `String` | — |
| `is_emergency` | `bool` | — |
| `created_at` | `u64` | — |
| `status` | `RequestStatus` | — |
| `decision_by` | `Option<Address>` | — |
| `decision_at` | `Option<u64>` | — |

#### `enum RequestStatus`

| Variant | Value | Description |
|---|---|---|
| `Pending` | — | — |
| `Approved` | — | — |
| `Rejected` | — | — |
| `Expired` | — | — |
| `Revoked` | — | — |

#### `struct AuditEntry`

| Field | Type | Description |
|---|---|---|
| `entry_id` | `u64` | — |
| `accessor_chain` | `ChainId` | — |
| `accessor_address` | `String` | — |
| `patient` | `Address` | — |
| `record_id` | `u64` | — |
| `action` | `AccessAction` | — |
| `timestamp` | `u64` | — |
| `ip_hash` | `BytesN<32>` | — |
| `success` | `bool` | — |

#### `enum AccessAction`

| Variant | Value | Description |
|---|---|---|
| `View` | — | — |
| `Download` | — | — |
| `Share` | — | — |
| `Export` | — | — |
| `EmergencyAccess` | — | — |

#### `struct Delegation`

| Field | Type | Description |
|---|---|---|
| `delegator` | `Address` | — |
| `delegate` | `Address` | — |
| `delegate_chain` | `ChainId` | — |
| `delegate_address` | `String` | — |
| `can_grant` | `bool` | — |
| `can_revoke` | `bool` | — |
| `can_manage_emergency` | `bool` | — |
| `created_at` | `u64` | — |
| `expires_at` | `u64` | — |
| `is_active` | `bool` | — |

#### `struct EmergencyConfig`

| Field | Type | Description |
|---|---|---|
| `patient` | `Address` | — |
| `is_enabled` | `bool` | — |
| `auto_approve_duration` | `u64` | — |
| `required_attestations` | `u32` | — |
| `trusted_providers` | `Vec<String>` | — |

#### `struct SwapProposal`

| Field | Type | Description |
|---|---|---|
| `swap_id` | `u64` | — |
| `initiator` | `Address` | — |
| `counterpart_chain` | `ChainId` | — |
| `counterpart_address` | `String` | — |
| `offered_grant_id` | `u64` | — |
| `requested_permission` | `PermissionLevel` | — |
| `requested_scope` | `AccessScope` | — |
| `hash_lock` | `BytesN<32>` | — |
| `timelock` | `u64` | — |
| `created_at` | `u64` | — |
| `status` | `SwapStatus` | — |
| `accepted_grant_id` | `u64` | — |

#### `enum SwapStatus`

| Variant | Value | Description |
|---|---|---|
| `Proposed` | — | — |
| `Accepted` | — | — |
| `Completed` | — | — |
| `Cancelled` | — | — |
| `Expired` | — | — |

#### `enum DataKey`

| Variant | Value | Description |
|---|---|---|
| `Core` | — | — |
| `config` | — | — |
| `Admin` | — | — |
| `Bridge` | — | — |
| `Identity` | — | — |
| `Paused` | — | — |
| `GrantCount` | — | — |
| `RequestCount` | — | — |
| `AuditCount` | — | — |
| `SwapCount` | — | — |
| `Map` | — | — |
| `based` | — | — |
| `storage` | — | — |
| `sequential` | — | — |
| `ID` | — | — |
| `lookup` | — | — |
| `needed` | — | — |
| `for` | — | — |
| `verify_access` | — | — |
| `Grants` | — | — |
| `Requests` | — | — |
| `AuditLog` | — | — |
| `Per` | — | — |
| `item` | — | — |
| `storage` | — | — |
| `BUG` | — | — |
| `FIX` | — | — |
| `Delegation` | — | — |
| `Address` | — | — |
| `Address` | — | — |
| `delegator` | — | — |
| `delegate` | — | — |
| `was` | — | — |
| `deleg_key` | — | — |
| `EmergencyConfig` | — | — |
| `Address` | — | — |
| `patient` | — | — |
| `address` | — | — |
| `was` | — | — |
| `emerg_key` | — | — |
| `Swap` | — | — |
| `u64` | — | — |

#### `enum Error`

| Variant | Value | Description |
|---|---|---|
| `Existing` | — | — |
| `errors` | — | — |
| `NotAuthorized` | 1 | — |
| `ContractPaused` | 2 | — |
| `AlreadyInitialized` | 3 | — |
| `GrantNotFound` | 4 | — |
| `GrantExpired` | 5 | — |
| `GrantRevoked` | 6 | — |
| `RequestNotFound` | 7 | — |
| `RequestExpired` | 8 | — |
| `RequestAlreadyProcessed` | 9 | — |
| `DelegationNotFound` | 10 | — |
| `DelegationExpired` | 11 | — |
| `InsufficientPermissions` | 12 | — |
| `EmergencyNotEnabled` | 13 | — |
| `EmergencyNotAuthorized` | 14 | — |
| `InvalidScope` | 15 | — |
| `InvalidCondition` | 16 | — |
| `AuditRequired` | 17 | — |
| `SingleUseConsumed` | 18 | — |
| `TimeRestrictionViolated` | 19 | — |
| `Overflow` | 20 | — |
| `New` | — | — |
| `errors` | — | — |
| `SwapNotFound` | 21 | — |
| `SwapExpired` | 22 | — |
| `SwapAlreadyProcessed` | 23 | — |

### Examples

#### `test_initialize`

```rust
let env = Env::default();
    let (client, admin, bridge, identity) = create_contract(&env);

    initialize_contract(&env, &client, &admin, &bridge, &identity);

    assert!(!client.is_paused());
```

#### `test_initialize_twice_fails`

```rust
let env = Env::default();
    let (client, admin, bridge, identity) = create_contract(&env);

    env.mock_all_auths();
    client.initialize(&admin, &bridge, &identity);

    let result = client.try_initialize(&admin, &bridge, &identity);
    assert_eq!(result, Err(Ok(Error::AlreadyInitialized)));
```

#### `test_grant_access`

```rust
let env = Env::default();
    let (client, admin, bridge, identity) = create_contract(&env);
    initialize_contract(&env, &client, &admin, &bridge, &identity);

    let patient = Address::generate(&env);
    let grantee_address = String::from_str(&env, "0x1234567890abcdef1234567890abcdef12345678");

    env.mock_all_auths();
```

---

## cross_chain_enhancements

### Functions

| Function | Parameters | Returns | Description |
|---|---|---|---|
| `initialize` | `env: Env, admin: Address` | `Result<(), Error>` | — |
| `check_rate_limit` | `env: Env, caller: Address, amount: u64` | `Result<bool, Error>` | Check and update rate limit for an operation |
| `get_zk_proof` | `env: Env, proof_id: BytesN<32>` | `Option<ZKOwnershipProof>` | Get ZK proof status |
| `get_integrity_proof` | `env: Env, proof_id: BytesN<32>` | `Option<ZKDataIntegrityProof>` | Get integrity proof |

### Types

#### `struct ZKOwnershipProof`

| Field | Type | Description |
|---|---|---|
| `proof_id` | `BytesN<32>` | — |
| `record_id` | `u64` | — |
| `owner` | `Address` | — |
| `chain` | `ChainId` | — |
| `proof_data` | `BytesN<64>` | — |
| `statement_hash` | `BytesN<32>` | — |
| `verified` | `bool` | — |
| `verified_at` | `Option<u64>` | — |
| `verifier` | `Option<Address>` | — |

#### `struct ZKDataIntegrityProof`

| Field | Type | Description |
|---|---|---|
| `proof_id` | `BytesN<32>` | — |
| `data_hash` | `BytesN<32>` | — |
| `merkle_root` | `BytesN<32>` | — |
| `merkle_path` | `Vec<BytesN<32>>` | — |
| `leaf_index` | `u32` | — |
| `proven_at` | `u64` | — |
| `chain_id` | `ChainId` | — |

#### `enum ChainId`

| Variant | Value | Description |
|---|---|---|
| `Stellar` | — | — |
| `Ethereum` | — | — |
| `Polygon` | — | — |
| `Avalanche` | — | — |
| `BinanceSmartChain` | — | — |
| `Arbitrum` | — | — |
| `Optimism` | — | — |
| `Custom` | — | — |
| `u32` | — | — |

#### `struct ReplayProtection`

| Field | Type | Description |
|---|---|---|
| `message_hash` | `BytesN<32>` | — |
| `source_chain` | `ChainId` | — |
| `seen_at` | `u64` | — |
| `expires_at` | `u64` | — |

#### `struct RateLimit`

| Field | Type | Description |
|---|---|---|
| `address` | `Address` | — |
| `daily_limit` | `u64` | — |
| `used_today` | `u64` | — |
| `last_reset` | `u64` | — |
| `is_active` | `bool` | — |

#### `enum DataKey`

| Variant | Value | Description |
|---|---|---|
| `ZKProof` | — | — |
| `BytesN` | — | — |
| `32` | — | — |
| `IntegrityProof` | — | — |
| `BytesN` | — | — |
| `32` | — | — |
| `SeenMessage` | — | — |
| `BytesN` | — | — |
| `32` | — | — |
| `RateLimit` | — | — |
| `Address` | — | — |
| `Admin` | — | — |
| `Initialized` | — | — |
| `ZKCounter` | — | — |
| `IntegrityCounter` | — | — |

#### `enum Error`

| Variant | Value | Description |
|---|---|---|
| `NotAuthorized` | 1 | — |
| `NotInitialized` | 2 | — |
| `AlreadyInitialized` | 3 | — |
| `InvalidProof` | 4 | — |
| `ProofAlreadyVerified` | 5 | — |
| `ProofNotFound` | 6 | — |
| `ReplayDetected` | 7 | — |
| `RateLimitExceeded` | 8 | — |
| `ArithmeticOverflow` | 9 | — |
| `InvalidMerklePath` | 10 | — |
| `ExpiredMessage` | 11 | — |

---

## cross_chain_identity

### Functions

| Function | Parameters | Returns | Description |
|---|---|---|---|
| `initialize` | `env: Env, admin: Address, bridge_contract: Address` | `Result<bool, Error>` | — |
| `pause` | `env: Env, caller: Address` | `Result<bool, Error>` | — |
| `unpause` | `env: Env, caller: Address` | `Result<bool, Error>` | — |
| `verify_identity` | `env: Env, stellar_address: Address, external_chain: ChainId` | `bool` | — |
| `get_request` | `env: Env, request_id: u64` | `Option<VerificationRequest>` | — |
| `get_sync` | `env: Env, sync_id: u64` | `Option<IdentitySync>` | — |
| `get_validator` | `env: Env, validator_address: Address` | `Option<IdentityValidator>` | — |
| `get_attestation` | `env: Env, request_id: u64, validator: Address` | `Option<Attestation>` | — |
| `is_paused` | `env: Env` | `bool` | — |

### Types

#### `enum VerificationStatus`

| Variant | Value | Description |
|---|---|---|
| `Unverified` | — | — |
| `Pending` | — | — |
| `Verified` | — | — |
| `Revoked` | — | — |
| `Expired` | — | — |

#### `enum ChainId`

| Variant | Value | Description |
|---|---|---|
| `Stellar` | — | — |
| `Ethereum` | — | — |
| `Polygon` | — | — |
| `Avalanche` | — | — |
| `BinanceSmartChain` | — | — |
| `Arbitrum` | — | — |
| `Optimism` | — | — |
| `Custom` | — | — |
| `u32` | — | — |

#### `struct CrossChainIdentity`

| Field | Type | Description |
|---|---|---|
| `stellar_address` | `Address` | — |
| `external_chain` | `ChainId` | — |
| `external_address` | `String` | — |
| `verification_status` | `VerificationStatus` | — |
| `verified_at` | `u64` | — |
| `expires_at` | `u64` | — |
| `attestations` | `u32` | — |
| `metadata_hash` | `BytesN<32>` | — |

#### `struct VerificationRequest`

| Field | Type | Description |
|---|---|---|
| `request_id` | `u64` | — |
| `stellar_address` | `Address` | — |
| `external_chain` | `ChainId` | — |
| `external_address` | `String` | — |
| `proof` | `BytesN<64>` | — |
| `created_at` | `u64` | — |
| `status` | `RequestStatus` | — |
| `validator_attestations` | `Vec<Address>` | — |

#### `enum RequestStatus`

| Variant | Value | Description |
|---|---|---|
| `Pending` | — | — |
| `Approved` | — | — |
| `Rejected` | — | — |
| `Expired` | — | — |

#### `struct Attestation`

| Field | Type | Description |
|---|---|---|
| `validator` | `Address` | — |
| `stellar_address` | `Address` | — |
| `external_chain` | `ChainId` | — |
| `attested_at` | `u64` | — |
| `is_valid` | `bool` | — |
| `signature` | `BytesN<64>` | — |

#### `struct IdentityValidator`

| Field | Type | Description |
|---|---|---|
| `address` | `Address` | — |
| `name` | `String` | — |
| `public_key` | `BytesN<32>` | — |
| `is_active` | `bool` | — |
| `trust_score` | `u32` | — |
| `total_attestations` | `u64` | — |

#### `struct IdentitySync`

| Field | Type | Description |
|---|---|---|
| `stellar_address` | `Address` | — |
| `source_chain` | `ChainId` | — |
| `dest_chain` | `ChainId` | — |
| `sync_timestamp` | `u64` | — |
| `sync_status` | `SyncStatus` | — |
| `sync_proof` | `BytesN<32>` | — |

#### `enum SyncStatus`

| Variant | Value | Description |
|---|---|---|
| `Initiated` | — | — |
| `InProgress` | — | — |
| `Completed` | — | — |
| `Failed` | — | — |

#### `enum DataKey`

| Variant | Value | Description |
|---|---|---|
| `Core` | — | — |
| `config` | — | — |
| `Admin` | — | — |
| `Bridge` | — | — |
| `Paused` | — | — |
| `RequestCount` | — | — |
| `SyncCount` | — | — |
| `MinAttestations` | — | — |
| `IdentityTtl` | — | — |
| `Per` | — | — |
| `item` | — | — |
| `storage` | — | — |
| `BUG` | — | — |
| `FIX` | — | — |
| `Validator` | — | — |
| `Address` | — | — |
| `Request` | — | — |
| `u64` | — | — |
| `Identity` | — | — |
| `Address` | — | — |
| `ChainId` | — | — |
| `BUG` | — | — |
| `FIX` | — | — |
| `was` | — | — |
| `always` | — | — |
| `id_key` | — | — |
| `Attestation` | — | — |
| `u64` | — | — |
| `Address` | — | — |
| `BUG` | — | — |
| `FIX` | — | — |
| `was` | — | — |
| `always` | — | — |
| `att_key` | — | — |
| `Sync` | — | — |
| `u64` | — | — |

#### `enum Error`

| Variant | Value | Description |
|---|---|---|
| `NotAuthorized` | 1 | — |
| `ContractPaused` | 2 | — |
| `AlreadyInitialized` | 3 | — |
| `IdentityNotFound` | 4 | — |
| `IdentityAlreadyExists` | 5 | — |
| `IdentityExpired` | 6 | — |
| `IdentityRevoked` | 7 | — |
| `RequestNotFound` | 8 | — |
| `RequestExpired` | 9 | — |
| `RequestAlreadyProcessed` | 10 | — |
| `ValidatorNotFound` | 11 | — |
| `ValidatorNotActive` | 12 | — |
| `DuplicateAttestation` | 13 | — |
| `InsufficientAttestations` | 14 | — |
| `InvalidProof` | 15 | — |
| `InvalidChain` | 16 | — |
| `SyncNotFound` | 17 | — |
| `SyncFailed` | 18 | — |

### Examples

#### `test_initialize`

```rust
let env = Env::default();
    let (client, admin, bridge) = create_contract(&env);

    initialize_contract(&env, &client, &admin, &bridge);

    assert!(!client.is_paused());
```

#### `test_initialize_twice_fails`

```rust
let env = Env::default();
    let (client, admin, bridge) = create_contract(&env);

    env.mock_all_auths();
    client.initialize(&admin, &bridge);

    let result = client.try_initialize(&admin, &bridge);
    assert_eq!(result, Err(Ok(Error::AlreadyInitialized)));
```

#### `test_add_validator`

```rust
let env = Env::default();
    let (client, admin, bridge) = create_contract(&env);
    initialize_contract(&env, &client, &admin, &bridge);

    let validator = Address::generate(&env);
    let name = String::from_str(&env, "Validator1");
    let public_key = generate_public_key(&env);

    env.mock_all_auths();
```

---

## crypto_registry

### Functions

| Function | Parameters | Returns | Description |
|---|---|---|---|
| `initialize` | `env: Env, admin: Address` | `Result<(), Error>` | Initialize the registry with an admin address for policy upgrades. Key registration/rotation is always self-authorized by the account. |
| `revoke_key_bundle` | `env: Env, owner: Address, version: u32` | `Result<(), Error>` | Revoke a specific key bundle version. |
| `get_current_version` | `env: Env, owner: Address` | `Result<u32, Error>` | — |
| `get_current_key_bundle` | `env: Env, owner: Address` | `Result<Option<KeyBundle>, Error>` | — |
| `get_all_key_versions` | `env: Env, owner: Address` | `Result<Vec<u32>, Error>` | Get all key bundle versions for an owner (including revoked ones). |

### Types

#### `enum KeyAlgorithm`

| Variant | Value | Description |
|---|---|---|
| `Classical` | — | — |
| `X25519` | — | — |
| `Ed25519` | — | — |
| `Secp256k1` | — | — |
| `Post` | — | — |
| `quantum` | — | — |
| `preparations` | — | — |
| `Lattice` | — | — |
| `based` | — | — |
| `Kyber768` | — | — |
| `Kyber1024` | — | — |
| `Dilithium2` | — | — |
| `Dilithium3` | — | — |
| `Dilithium5` | — | — |
| `Falcon512` | — | — |
| `Falcon1024` | — | — |
| `Hash` | — | — |
| `based` | — | — |
| `signatures` | — | — |
| `XMSS` | — | — |
| `SphincsPlus` | — | — |
| `Code` | — | — |
| `based` | — | — |
| `cryptography` | — | — |
| `McEliece348864` | — | — |
| `McEliece460896` | — | — |
| `McEliece6688128` | — | — |
| `McEliece6960119` | — | — |
| `McEliece8192128` | — | — |
| `Multivariate` | — | — |
| `cryptography` | — | — |
| `Rainbow` | — | — |
| `GeMSS` | — | — |
| `Quantum` | — | — |
| `safe` | — | — |
| `KDF` | — | — |
| `HkdfSha3` | — | — |
| `For` | — | — |
| `forward` | — | — |
| `compatibility` | — | — |
| `Custom` | — | — |
| `u32` | — | — |

#### `struct PublicKey`

| Field | Type | Description |
|---|---|---|
| `algorithm` | `KeyAlgorithm` | — |
| `key` | `Bytes` | — |

#### `struct KeyBundle`

| Field | Type | Description |
|---|---|---|
| `version` | `u32` | — |
| `created_at` | `u64` | — |
| `revoked` | `bool` | — |
| `encryption_key` | `PublicKey` | — |
| `has_pq_encryption_key` | `bool` | — |
| `pq_encryption_key` | `PublicKey` | — |
| `has_signing_key` | `bool` | — |
| `signing_key` | `PublicKey` | — |
| `bundle_id` | `BytesN<32>` | — |

#### `enum DataKey`

| Variant | Value | Description |
|---|---|---|
| `Initialized` | — | — |
| `Admin` | — | — |
| `CurrentVersion` | — | — |
| `Address` | — | — |
| `Bundle` | — | — |
| `Address` | — | — |
| `u32` | — | — |

#### `enum Error`

| Variant | Value | Description |
|---|---|---|
| `AlreadyInitialized` | 1 | — |
| `NotInitialized` | 2 | — |
| `NotAuthorized` | 3 | — |
| `InvalidKey` | 4 | — |
| `KeyNotFound` | 5 | — |
| `KeyAlreadyRevoked` | 6 | — |
| `InvalidKeyLength` | 7 | — |

### Examples

#### `key_bundle_registration_and_rotation`

```rust
let env = Env::default();
    env.mock_all_auths();

    let (client, _id) = register_contract(&env);
    let admin = soroban_sdk::Address::generate(&env);
    client.initialize(&admin);

    let alice = soroban_sdk::Address::generate(&env);
    let enc_key = PublicKey {
```

#### `revoke_bundle_marks_revoked`

```rust
let env = Env::default();
    env.mock_all_auths();

    let (client, _id) = register_contract(&env);
    let admin = soroban_sdk::Address::generate(&env);
    client.initialize(&admin);

    let alice = soroban_sdk::Address::generate(&env);
    let enc_key = PublicKey {
```

#### `post_quantum_key_registration`

```rust
let env = Env::default();
    env.mock_all_auths();

    let (client, _id) = register_contract(&env);
    let admin = soroban_sdk::Address::generate(&env);
    client.initialize(&admin);

    let alice = soroban_sdk::Address::generate(&env);
```

---

## deprecation_framework

### Functions

| Function | Parameters | Returns | Description |
|---|---|---|---|
| `initialize` | `env: Env, admin: Address` | `Result<(), Error>` | Initialize the deprecation framework |
| `get_sunset_timeline` | `env: Env, contract_id: String` | `Result<SunsetTimeline, Error>` | Get sunset timeline |
| `get_migration_guide` | `env: Env, contract_id: String` | `Result<MigrationGuide, Error>` | Get migration guide |
| `is_deprecated` | `env: Env, contract_id: String` | `bool` | Check if contract is deprecated |

### Error Codes

| Variant | Code | Description |
|---|---|---|
| `NotInitialized` | 1 | — |
| `AlreadyInitialized` | 2 | — |
| `NotAuthorized` | 3 | — |
| `ContractNotFound` | 4 | — |
| `ContractAlreadyDeprecated` | 5 | — |
| `InvalidTimeline` | 6 | — |
| `InvalidPhaseTransition` | 7 | — |
| `TimelineNotFound` | 8 | — |
| `GuideNotFound` | 9 | — |
| `ChecklistNotFound` | 10 | — |
| `InvalidChecklistIndex` | 11 | — |

### Examples

#### `test_initialize`

```rust
let env = Env::default();
        env.mock_all_auths();

        let admin = Address::generate(&env);
        let contract_id = env.register_contract(None, DeprecationFramework);
        let client = DeprecationFrameworkClient::new(&env, &contract_id);

        client.initialize(&admin);
    }
```

---

## dispute_resolution

### Functions

| Function | Parameters | Returns | Description |
|---|---|---|---|
| `initialize` | `env: Env, arbiters: Vec<Address>` | `()` | — |
| `dispute` | `env: Env, proposal_id: u64, challenger: Address` | `()` | — |
| `is_disputed` | `env: Env, proposal_id: u64` | `bool` | — |

### Types

#### `enum Error`

| Variant | Value | Description |
|---|---|---|
| `NotInitialized` | 1 | — |
| `NotArbiter` | 2 | — |
| `DisputeNotFound` | 3 | — |

---

## emr_integration

### Functions

| Function | Parameters | Returns | Description |
|---|---|---|---|
| `initialize` | `env: Env, admin: Address, fhir_contract: Address` | `Result<bool, Error>` | — |
| `get_emr_system` | `env: Env, system_id: String` | `Result<EMRSystem, Error>` | — |
| `get_network_node` | `env: Env, node_id: String` | `Result<NetworkNode, Error>` | — |
| `get_interop_test` | `env: Env, test_id: String` | `Result<InteroperabilityTest, Error>` | — |
| `wrap_transport_payload` | `env: Env, message_id: String` | `Result<String, Error>` | — |
| `get_message` | `env: Env, message_id: String` | `Result<HealthcareMessage, Error>` | — |
| `get_supported_message_types` | `env: Env` | `Vec<String>` | — |
| `pause` | `env: Env, admin: Address` | `Result<bool, Error>` | — |
| `resume` | `env: Env, admin: Address` | `Result<bool, Error>` | — |

### Types

#### `enum EMRStatus`

| Variant | Value | Description |
|---|---|---|
| `Active` | — | — |
| `Inactive` | — | — |
| `Suspended` | — | — |
| `Decommissioned` | — | — |

#### `enum IntegrationStatus`

| Variant | Value | Description |
|---|---|---|
| `Pending` | — | — |
| `InProgress` | — | — |
| `Completed` | — | — |
| `Failed` | — | — |
| `Paused` | — | — |

#### `struct EMRSystem`

| Field | Type | Description |
|---|---|---|
| `system_id` | `String` | — |
| `vendor_name` | `String` | — |
| `vendor_contact` | `String` | — |
| `system_version` | `String` | — |
| `supported_standards` | `Vec<String>` | — |
| `api_endpoints` | `Vec<String>` | — |
| `status` | `EMRStatus` | — |
| `last_activity` | `u64` | — |
| `integration_date` | `u64` | — |

#### `struct ProviderOnboarding`

| Field | Type | Description |
|---|---|---|
| `onboarding_id` | `String` | — |
| `provider_id` | `String` | — |
| `provider_name` | `String` | — |
| `provider_email` | `String` | — |
| `facility_name` | `String` | — |
| `npi` | `String` | — |
| `emr_system_id` | `String` | — |
| `status` | `IntegrationStatus` | — |
| `created_at` | `u64` | — |
| `completed_at` | `u64` | — |
| `verification_document_hash` | `BytesN<32>` | — |
| `compliance_checklist` | `Vec<String>` | — |
| `notes` | `String` | — |

#### `struct ProviderVerification`

| Field | Type | Description |
|---|---|---|
| `verification_id` | `String` | — |
| `provider_id` | `String` | — |
| `verified_by` | `Address` | — |
| `verification_timestamp` | `u64` | — |
| `license_number` | `String` | — |
| `license_state` | `String` | — |
| `license_expiration` | `String` | — |
| `board_certification` | `Vec<String>` | — |
| `malpractice_insurance` | `String` | — |
| `background_check_id` | `String` | — |
| `verification_status` | `String` | — |

#### `struct NetworkNode`

| Field | Type | Description |
|---|---|---|
| `node_id` | `String` | — |
| `provider_id` | `String` | — |
| `node_type` | `String` | — |
| `network_name` | `String` | — |
| `geographic_region` | `String` | — |
| `specialties` | `Vec<String>` | — |
| `bed_capacity` | `u32` | — |
| `operating_hours` | `String` | — |
| `emergency_services` | `bool` | — |
| `telemedicine_enabled` | `bool` | — |
| `coordinates` | `String` | — |
| `connectivity_score` | `u32` | — |

#### `struct InteroperabilityAgreement`

| Field | Type | Description |
|---|---|---|
| `agreement_id` | `String` | — |
| `initiating_provider` | `String` | — |
| `receiving_provider` | `String` | — |
| `effective_date` | `String` | — |
| `expiration_date` | `String` | — |
| `supported_data_types` | `Vec<String>` | — |
| `access_level` | `String` | — |
| `audit_requirement` | `String` | — |
| `data_encryption` | `String` | — |
| `status` | `String` | — |

#### `struct InteroperabilityTest`

| Field | Type | Description |
|---|---|---|
| `test_id` | `String` | — |
| `test_date` | `u64` | — |
| `provider_a` | `String` | — |
| `provider_b` | `String` | — |
| `test_type` | `String` | — |
| `result_status` | `String` | — |
| `success_rate` | `u32` | — |
| `data_exchanged` | `u64` | — |
| `latency_ms` | `u32` | — |
| `error_details` | `String` | — |
| `tester_address` | `Address` | — |

#### `enum MessagingStandard`

| Variant | Value | Description |
|---|---|---|
| `HL7v2` | — | — |
| `HL7v3` | — | — |
| `CDA` | — | — |

#### `enum TransportProtocol`

| Variant | Value | Description |
|---|---|---|
| `MLLP` | — | — |
| `HTTP` | — | — |

#### `enum CharacterEncoding`

| Variant | Value | Description |
|---|---|---|
| `UTF8` | — | — |
| `UTF16` | — | — |
| `ASCII` | — | — |
| `ISO88591` | — | — |

#### `enum ValidationSeverity`

| Variant | Value | Description |
|---|---|---|
| `Info` | — | — |
| `Warning` | — | — |
| `Critical` | — | — |

#### `struct HealthcareMessage`

| Field | Type | Description |
|---|---|---|
| `message_id` | `String` | — |
| `source_system_id` | `String` | — |
| `standard` | `MessagingStandard` | — |
| `version` | `String` | — |
| `message_type` | `String` | — |
| `control_id` | `String` | — |
| `content_type` | `String` | — |
| `encoding` | `CharacterEncoding` | — |
| `transport` | `TransportProtocol` | — |
| `segment_count` | `u32` | — |
| `field_count` | `u32` | — |
| `metadata` | `Map<String` | — |
| `raw_payload` | `String` | — |
| `created_at` | `u64` | — |

#### `struct ValidationIssue`

| Field | Type | Description |
|---|---|---|
| `code` | `String` | — |
| `severity` | `ValidationSeverity` | — |
| `message` | `String` | — |
| `location` | `String` | — |

#### `struct MessageValidationReport`

| Field | Type | Description |
|---|---|---|
| `report_id` | `String` | — |
| `message_id` | `String` | — |
| `is_valid` | `bool` | — |
| `issues` | `Vec<ValidationIssue>` | — |
| `validated_at` | `u64` | — |

#### `struct MessageTransformation`

| Field | Type | Description |
|---|---|---|
| `transform_id` | `String` | — |
| `source_message_id` | `String` | — |
| `target_message_id` | `String` | — |
| `source_standard` | `MessagingStandard` | — |
| `target_standard` | `MessagingStandard` | — |
| `target_message_type` | `String` | — |
| `status` | `String` | — |
| `notes` | `String` | — |
| `transformed_at` | `u64` | — |

#### `struct ThroughputBenchmark`

| Field | Type | Description |
|---|---|---|
| `benchmark_id` | `String` | — |
| `batch_size` | `u32` | — |
| `message_type` | `String` | — |
| `encoding` | `CharacterEncoding` | — |
| `transport` | `TransportProtocol` | — |
| `elapsed_ms` | `u32` | — |
| `messages_per_second` | `u32` | — |

#### `enum Error`

| Variant | Value | Description |
|---|---|---|
| `NotAuthorized` | 1 | — |
| `ContractPaused` | 2 | — |
| `EMRSystemNotFound` | 3 | — |
| `EMRSystemAlreadyExists` | 4 | — |
| `OnboardingNotFound` | 5 | — |
| `OnboardingAlreadyExists` | 6 | — |
| `VerificationNotFound` | 7 | — |
| `NetworkNodeNotFound` | 8 | — |
| `AgreementNotFound` | 9 | — |
| `TestNotFound` | 10 | — |
| `InvalidStatus` | 11 | — |
| `InvalidEMRSystem` | 12 | — |
| `ProviderNotFound` | 13 | — |
| `InvalidNPI` | 14 | — |
| `InvalidLicenseNumber` | 15 | — |
| `LicenseExpired` | 16 | — |
| `InvalidAgreement` | 17 | — |
| `AgreementNotActive` | 18 | — |
| `TestFailed` | 19 | — |
| `InvalidTestType` | 20 | — |
| `DuplicateTest` | 21 | — |
| `FHIRContractNotSet` | 22 | — |
| `OperationFailed` | 23 | — |
| `UnsupportedMessageFormat` | 24 | — |
| `MessageParseFailed` | 25 | — |
| `UnsupportedMessageType` | 26 | — |
| `InvalidMessagePayload` | 27 | — |
| `MessageNotFound` | 28 | — |
| `ValidationReportNotFound` | 29 | — |
| `TransformationNotFound` | 30 | — |
| `UnsupportedEncoding` | 31 | — |

### Examples

#### `initialize_and_generate_hl7_v2_message`

```rust
let env = Env::default();
    let (client, _admin, _) = setup(&env);

    let generated = client.mock_all_auths().generate_message(
        &Address::generate(&env),
        &String::from_str(&env, "msg-1"),
        &String::from_str(&env, "epic-prod"),
        &MessagingStandard::HL7v2,
        &String::from_str(&env, "2.5.1"),
```

#### `parse_hl7_v2_message_extracts_header_fields`

```rust
let env = Env::default();
    let (client, _admin, _) = setup(&env);

    let payload = String::from_str(
        &env,
        "MSH|^~\\&|VitaStellar|Main|EMR|Receiving|20260328090000||ORU^R01|CTRL-99|P|2.5.1||||||UTF-8\rPID|1||PAT-001||DOE^JANE\rOBX|1|TX|NOTE||All good",
    );

    let parsed = client.mock_all_auths().parse_message(
```

#### `supports_hl7_v3_and_cda_documents`

```rust
let env = Env::default();
    let (client, _admin, _) = setup(&env);

    let v3 = client.mock_all_auths().generate_message(
        &Address::generate(&env),
        &String::from_str(&env, "msg-v3"),
        &String::from_str(&env, "epic-prod"),
        &MessagingStandard::HL7v3,
        &String::from_str(&env, "3.0"),
```

---

## escrow

### Functions

| Function | Parameters | Returns | Description |
|---|---|---|---|
| `initialize` | `env: Env, admin: Address` | `Result<(), Error>` | — |
| `get_fee_config` | `env: Env` | `Option<FeeConfig>` | — |
| `mark_disputed` | `env: Env, caller: Address, order_id: u64` | `Result<(), Error>` | — |
| `approve_release` | `env: Env, order_id: u64, approver: Address` | `Result<(), Error>` | — |
| `release_escrow` | `env: Env, order_id: u64` | `Result<bool, Error>` | — |
| `refund_escrow` | `env: Env, order_id: u64, reason: String` | `Result<bool, Error>` | — |
| `get_escrow` | `env: Env, order_id: u64` | `Option<Escrow>` | — |
| `get_credit` | `env: Env, addr: Address` | `i128` | — |
| `withdraw` | `env: Env, caller: Address, token: Address, to: Address` | `Result<i128, Error>` | — |
| `get_total_volume` | `env: Env` | `i128` | — |
| `get_total_escrows` | `env: Env` | `u64` | — |
| `get_settled_rate` | `env: Env` | `u32` | — |
| `get_refund_rate` | `env: Env` | `u32` | — |
| `get_dispute_rate` | `env: Env` | `u32` | — |
| `get_active_escrows_count` | `env: Env` | `u64` | — |
| `get_stats_summary` | `env: Env` | `PlatformStats` | — |
| `get_platform_health_score` | `env: Env` | `u32` | — |
| `get_token_volume` | `env: Env, token: Address` | `i128` | — |
| `get_donor_reputation` | `env: Env, donor: Address` | `u32` | — |
| `get_daily_stats` | `env: Env, day_id: u64` | `Option<DailyStats>` | — |
| `export_summary` | `env: Env, format: String` | `ExportMetadata` | — |

### Error Codes

| Variant | Code | Description |
|---|---|---|
| `Unauthorized` | 100 | — |
| `NotAdmin` | 102 | — |
| `InsufficientApprovals` | 120 | — |
| `InvalidAmount` | 205 | — |
| `InvalidFeeBps` | 260 | — |
| `FeeNotSet` | 380 | — |
| `ReentrancyRejected` | 381 | — |
| `InvalidStateTransition` | 382 | — |
| `EscrowExists` | 480 | — |
| `EscrowNotFound` | 481 | — |
| `AlreadySettled` | 482 | — |
| `NoBasisToRefund` | 560 | — |
| `NoCredit` | 561 | — |
| `Overflow` | 562 | — |

---

## explainable_ai

### Functions

| Function | Parameters | Returns | Description |
|---|---|---|---|
| `initialize` | `env: Env, admin: Address` | `bool` | — |
| `request_explanation` | `env: Env, caller: Address, ai_insight_id: u64` | `u64` | — |
| `get_explanation_request` | `env: Env, request_id: u64` | `Option<ExplanationRequest>` | — |
| `get_explanation` | `env: Env, explanation_id: u64` | `Option<ExplanationMetadata>` | — |
| `get_bias_audit` | `env: Env, model_id: BytesN<32>` | `Option<BiasAuditResult>` | — |
| `get_shap_explanation` | `env: Env, shap_id: u64` | `Option<ShapExplanation>` | Get SHAP explanation by ID |
| `get_counterfactual` | `env: Env, cf_id: u64` | `Option<CounterfactualExplanation>` | Get counterfactual explanation by ID |

### Types

#### `struct ShapValue`

| Field | Type | Description |
|---|---|---|
| `feature_name` | `String` | — |
| `shap_value` | `i128` | — |
| `absolute_shap` | `u128` | — |
| `feature_value` | `i128` | — |
| `baseline_value` | `i128` | — |

#### `struct ShapExplanation`

| Field | Type | Description |
|---|---|---|
| `explanation_id` | `u64` | — |
| `model_id` | `BytesN<32>` | — |
| `patient` | `Address` | — |
| `prediction_id` | `u64` | — |
| `base_value` | `i128` | — |
| `prediction` | `i128` | — |
| `shap_values` | `Vec<ShapValue>` | — |
| `method` | `String` | — |
| `computation_time_ms` | `u64` | — |
| `created_at` | `u64` | — |

#### `struct CounterfactualExplanation`

| Field | Type | Description |
|---|---|---|
| `cf_id` | `u64` | — |
| `original_prediction` | `i128` | — |
| `target_prediction` | `i128` | — |
| `minimal_changes` | `Vec<FeatureChange>` | — |
| `feasibility_score` | `u32` | — |
| `proximity_distance` | `u128` | — |
| `created_at` | `u64` | — |

#### `struct FeatureChange`

| Field | Type | Description |
|---|---|---|
| `feature_name` | `String` | — |
| `original_value` | `i128` | — |
| `counterfactual_value` | `i128` | — |
| `change_magnitude` | `u128` | — |

#### `struct ExplanationRequest`

| Field | Type | Description |
|---|---|---|
| `request_id` | `u64` | — |
| `patient` | `Address` | — |
| `ai_insight_id` | `u64` | — |
| `requested_at` | `u64` | — |
| `fulfilled_at` | `Option<u64>` | — |
| `explanation_ref` | `Option<String>` | — |
| `status` | `ExplanationStatus` | — |

#### `struct FeatureImportance`

| Field | Type | Description |
|---|---|---|
| `feature_name` | `String` | — |
| `importance_bps` | `u32` | — |
| `normalized_value` | `u32` | — |

#### `struct ExplanationMetadata`

| Field | Type | Description |
|---|---|---|
| `insight_id` | `u64` | — |
| `model_id` | `BytesN<32>` | — |
| `patient` | `Address` | — |
| `explanation_method` | `String` | — |
| `feature_importance` | `Vec<FeatureImportance>` | — |
| `primary_factors` | `Vec<String>` | — |
| `confidence_impact` | `u32` | — |
| `created_at` | `u64` | — |
| `explanation_ref` | `String` | — |

#### `enum ExplanationStatus`

| Variant | Value | Description |
|---|---|---|
| `Pending` | — | — |
| `Processing` | — | — |
| `Completed` | — | — |
| `Failed` | — | — |

#### `struct BiasAuditResult`

| Field | Type | Description |
|---|---|---|
| `model_id` | `BytesN<32>` | — |
| `audit_date` | `u64` | — |
| `demographic_fairness_metrics` | `Map<String` | — |
| `equalized_odds` | `bool` | — |
| `calibration_by_group` | `Map<String` | — |
| `audit_summary` | `String` | — |
| `recommendations` | `Vec<String>` | — |

#### `enum DataKey`

| Variant | Value | Description |
|---|---|---|
| `Admin` | — | — |
| `Request` | — | — |
| `u64` | — | — |
| `Explanation` | — | — |
| `u64` | — | — |
| `BiasAudit` | — | — |
| `BytesN` | — | — |
| `32` | — | — |
| `RequestCounter` | — | — |
| `ExplanationCounter` | — | — |
| `AuditCounter` | — | — |
| `ShapExplanation` | — | — |
| `u64` | — | — |
| `ShapCounter` | — | — |
| `Counterfactual` | — | — |
| `u64` | — | — |
| `CfCounter` | — | — |

#### `enum Error`

| Variant | Value | Description |
|---|---|---|
| `NotAuthorized` | 1 | — |
| `RequestNotFound` | 2 | — |
| `ExplanationNotFound` | 3 | — |
| `InvalidImportance` | 4 | — |
| `AuditNotFound` | 5 | — |
| `InvalidBPSValue` | 6 | — |

---

## fido2_authenticator

### Functions

| Function | Parameters | Returns | Description |
|---|---|---|---|
| `initialize` | `env: Env, admin: Address, rp_id_hash: BytesN<32>` | `Result<(), Error>` | Initializes the contract.  Must be called exactly once.  * `admin`      — address authorised to call administrative functions. * `rp_id_hash` — SHA-256 of the relying party identifier string (e.g., `sha256(b"vitastellar.health")`). |
| `set_zk_verifier` | `env: Env, caller: Address, contract_id: Address` | `Result<(), Error>` | Configures the ZK verifier contract used for ES256 (P-256) assertions. |
| `issue_registration_challenge` | `env: Env, user: Address` | `Result<BytesN<32>, Error>` | Issues a registration challenge for `user`.  The 32-byte challenge must be embedded in `clientDataJSON.challenge` during the FIDO2 attestation ceremony.  Valid for 5 minutes. |
| `issue_auth_challenge` | `env: Env, user: Address` | `Result<BytesN<32>, Error>` | Issues a one-time authentication challenge for `user`. |
| `get_device_count` | `env: Env, user: Address` | `u32` | Returns the total device count (active + revoked) for `user`. |
| `get_active_device_count` | `env: Env, user: Address` | `u32` | Returns the number of active (non-revoked) devices for `user`. |
| `is_device_registered` | `env: Env, user: Address, credential_id_hash: BytesN<32>` | `bool` | Returns `true` if `credential_id_hash` is registered and active for `user`. |

### Types

#### `enum Error`

| Variant | Value | Description |
|---|---|---|
| `AlreadyInitialized` | 1 | — |
| `NotInitialized` | 2 | — |
| `NotAuthorized` | 3 | — |
| `DeviceNotFound` | 4 | — |
| `DeviceAlreadyRegistered` | 5 | — |
| `MaxDevicesReached` | 6 | — |
| `DeviceInactive` | 7 | — |
| `InvalidPublicKey` | 8 | — |
| `Signature` | — | — |
| `or` | — | — |
| `ZK` | — | — |
| `proof` | — | — |
| `verification` | — | — |
| `failed` | — | — |
| `InvalidSignature` | 9 | — |
| `authenticatorData` | — | — |
| `is` | — | — |
| `malformed` | — | — |
| `or` | — | — |
| `too` | — | — |
| `short` | — | — |
| `InvalidAuthenticatorData` | 10 | — |
| `The` | — | — |
| `pending` | — | — |
| `challenge` | — | — |
| `has` | — | — |
| `expired` | — | — |
| `5` | — | — |
| `minutes` | — | — |
| `old` | — | — |
| `ChallengeExpired` | 11 | — |
| `Authentication` | — | — |
| `attempted` | — | — |
| `without` | — | — |
| `first` | — | — |
| `issuing` | — | — |
| `a` | — | — |
| `challenge` | — | — |
| `NoChallengeIssued` | 12 | — |
| `Sign` | — | — |
| `count` | — | — |
| `did` | — | — |
| `not` | — | — |
| `increase` | — | — |
| `possible` | — | — |
| `credential` | — | — |
| `clone` | — | — |
| `detected` | — | — |
| `SignCountRegression` | 13 | — |
| `InvalidDeviceName` | 14 | — |
| `InvalidCredentialIdHash` | 15 | — |
| `verify_zk_assertion` | — | — |
| `called` | — | — |
| `but` | — | — |
| `no` | — | — |
| `ZK` | — | — |
| `verifier` | — | — |
| `contract` | — | — |
| `is` | — | — |
| `configured` | — | — |
| `ZkVerifierNotSet` | 16 | — |
| `ZK` | — | — |
| `proof` | — | — |
| `nullifier` | — | — |
| `has` | — | — |
| `already` | — | — |
| `been` | — | — |
| `used` | — | — |
| `replay` | — | — |
| `attack` | — | — |
| `NullifierAlreadyUsed` | 17 | — |
| `authenticatorData` | — | — |
| `rpIdHash` | — | — |
| `does` | — | — |
| `not` | — | — |
| `match` | — | — |
| `the` | — | — |
| `contract` | — | — |
| `s` | — | — |
| `configured` | — | — |
| `RP` | — | — |
| `ID` | — | — |
| `RpIdMismatch` | 18 | — |
| `FIDO2` | — | — |
| `User` | — | — |
| `Presence` | — | — |
| `UP` | — | — |
| `flag` | — | — |
| `is` | — | — |
| `not` | — | — |
| `set` | — | — |
| `in` | — | — |
| `authenticatorData` | — | — |
| `UserPresenceNotVerified` | 19 | — |
| `InvalidRevocationReason` | 20 | — |
| `register_device` | — | — |
| `called` | — | — |
| `with` | — | — |
| `an` | — | — |
| `algorithm` | — | — |
| `mismatched` | — | — |
| `to` | — | — |
| `the` | — | — |
| `public` | — | — |
| `key` | — | — |
| `size` | — | — |
| `AlgorithmKeyMismatch` | 21 | — |

#### `enum PublicKeyAlgorithm`

| Variant | Value | Description |
|---|---|---|
| `Ed25519` | — | — |
| `COSE` | — | — |
| `algorithm` | — | — |
| `8` | — | — |
| `Verified` | — | — |
| `on` | — | — |
| `chain` | — | — |
| `EdDSA` | — | — |
| `ECDSA` | — | — |
| `P` | — | — |
| `256` | — | — |
| `COSE` | — | — |
| `algorithm` | — | — |
| `7` | — | — |
| `Verified` | — | — |
| `via` | — | — |
| `ZK` | — | — |
| `proof` | — | — |
| `ES256` | — | — |

#### `enum AuthenticatorTransport`

| Variant | Value | Description |
|---|---|---|
| `USB` | — | — |
| `hardware` | — | — |
| `security` | — | — |
| `key` | — | — |
| `e` | — | — |
| `g` | — | — |
| `YubiKey` | — | — |
| `5` | — | — |
| `series` | — | — |
| `Usb` | — | — |
| `NFC` | — | — |
| `capable` | — | — |
| `hardware` | — | — |
| `security` | — | — |
| `key` | — | — |
| `Nfc` | — | — |
| `Bluetooth` | — | — |
| `Low` | — | — |
| `Energy` | — | — |
| `hardware` | — | — |
| `security` | — | — |
| `key` | — | — |
| `Ble` | — | — |
| `Built` | — | — |
| `in` | — | — |
| `platform` | — | — |
| `authenticator` | — | — |
| `fingerprint` | — | — |
| `sensor` | — | — |
| `Face` | — | — |
| `ID` | — | — |
| `Windows` | — | — |
| `Hello` | — | — |
| `Internal` | — | — |
| `Hybrid` | — | — |
| `passkey` | — | — |
| `synced` | — | — |
| `credential` | — | — |
| `cross` | — | — |
| `device` | — | — |
| `authentication` | — | — |
| `via` | — | — |
| `phone` | — | — |
| `Hybrid` | — | — |

#### `enum AuthenticatorAttachment`

| Variant | Value | Description |
|---|---|---|
| `Built` | — | — |
| `in` | — | — |
| `authenticator` | — | — |
| `Touch` | — | — |
| `ID` | — | — |
| `Face` | — | — |
| `ID` | — | — |
| `Windows` | — | — |
| `Hello` | — | — |
| `Platform` | — | — |
| `External` | — | — |
| `roaming` | — | — |
| `hardware` | — | — |
| `security` | — | — |
| `key` | — | — |
| `CrossPlatform` | — | — |

#### `struct Fido2Device`

| Field | Type | Description |
|---|---|---|
| `credential_id_hash` | `BytesN<32>` | — |
| `public_key` | `Bytes` | — |
| `algorithm` | `PublicKeyAlgorithm` | — |
| `sign_count` | `u32` | — |
| `aaguid` | `BytesN<16>` | — |
| `device_name` | `String` | — |
| `attachment` | `AuthenticatorAttachment` | — |
| `transports` | `Vec<AuthenticatorTransport>` | — |
| `created_at` | `u64` | — |
| `last_used_at` | `u64` | — |
| `is_active` | `bool` | — |
| `backup_eligible` | `bool` | — |
| `backup_state` | `bool` | — |

#### `struct PendingChallenge`

| Field | Type | Description |
|---|---|---|
| `challenge` | `BytesN<32>` | — |
| `created_at` | `u64` | — |
| `expires_at` | `u64` | — |

#### `struct AssertionResult`

| Field | Type | Description |
|---|---|---|
| `credential_id_hash` | `BytesN<32>` | — |
| `new_sign_count` | `u32` | — |
| `device_name` | `String` | — |
| `attachment` | `AuthenticatorAttachment` | — |
| `verified_at` | `u64` | — |

#### `struct RevocationRecord`

| Field | Type | Description |
|---|---|---|
| `credential_id_hash` | `BytesN<32>` | — |
| `device_name` | `String` | — |
| `revoked_at` | `u64` | — |
| `revoked_by` | `Address` | — |
| `reason` | `String` | — |

#### `enum DataKey`

| Variant | Value | Description |
|---|---|---|
| `Admin` | — | — |
| `Initialized` | — | — |
| `Optional` | — | — |
| `address` | — | — |
| `of` | — | — |
| `the` | — | — |
| `identity_registry` | — | — |
| `contract` | — | — |
| `IdentityRegistry` | — | — |
| `Optional` | — | — |
| `address` | — | — |
| `of` | — | — |
| `the` | — | — |
| `ZK` | — | — |
| `verifier` | — | — |
| `contract` | — | — |
| `required` | — | — |
| `for` | — | — |
| `ES256` | — | — |
| `ZkVerifier` | — | — |
| `SHA` | — | — |
| `256` | — | — |
| `of` | — | — |
| `the` | — | — |
| `relying` | — | — |
| `party` | — | — |
| `ID` | — | — |
| `string` | — | — |
| `e` | — | — |
| `g` | — | — |
| `sha256` | — | — |
| `vitastellar` | — | — |
| `health` | — | — |
| `RpIdHash` | — | — |
| `All` | — | — |
| `registered` | — | — |
| `devices` | — | — |
| `for` | — | — |
| `a` | — | — |
| `user` | — | — |
| `active` | — | — |
| `revoked` | — | — |
| `UserDevices` | — | — |
| `Address` | — | — |
| `Outstanding` | — | — |
| `registration` | — | — |
| `or` | — | — |
| `authentication` | — | — |
| `challenge` | — | — |
| `for` | — | — |
| `a` | — | — |
| `user` | — | — |
| `PendingChallenge` | — | — |
| `Address` | — | — |
| `Nullifiers` | — | — |
| `consumed` | — | — |
| `by` | — | — |
| `ZK` | — | — |
| `assertions` | — | — |
| `replay` | — | — |
| `attack` | — | — |
| `prevention` | — | — |
| `UsedNullifier` | — | — |
| `BytesN` | — | — |
| `32` | — | — |
| `Revocation` | — | — |
| `audit` | — | — |
| `log` | — | — |
| `per` | — | — |
| `user` | — | — |
| `RevocationHistory` | — | — |
| `Address` | — | — |

---

## fp_math

### Functions

| Function | Parameters | Returns | Description |
|---|---|---|---|
| `mul_bps` | `amount: i128, bps: u32` | `Option<i128>` | Multiply `amount` by basis points (1 bps = 0.01%) using floor division.  Floor rounding ensures fees are never rounded up — the fee taker always receives ≤ the exact fractional amount. Callers can reconstruct the complementary side as `amount - fee` to guarantee `fee + remainder == amount`.  Returns `None` if the intermediate `amount * bps` overflows `i128`. |
| `mul_bps_round_half_up` | `amount: i128, bps: u32` | `Option<i128>` | Multiply `amount` by basis points with round-half-up rounding.  Returns `None` on overflow. |

---

## governor

### Functions

| Function | Parameters | Returns | Description |
|---|---|---|---|
| `state` | `env: Env, proposal_id: u64` | `Result<u32, Error>` | — |
| `queue` | `env: Env, proposal_id: u64` | `Result<(), Error>` | — |
| `execute` | `env: Env, proposal_id: u64` | `Result<(), Error>` | — |
| `balance_of` | `env: Env, user: Address` | `i128` | — |
| `set_bal` | `env: Env, user: Address, amount: i128` | `()` | — |

### Types

#### `struct GovernorConfig`

| Field | Type | Description |
|---|---|---|
| `voting_delay` | `u64` | — |
| `voting_period` | `u64` | — |
| `quorum_bps` | `u32` | — |
| `timelock` | `Address` | — |
| `token` | `Address` | — |
| `rep_contract` | `Option<Address>` | — |
| `dispute_contract` | `Option<Address>` | — |
| `prop_threshold` | `i128` | — |

#### `struct Proposal`

| Field | Type | Description |
|---|---|---|
| `id` | `u64` | — |
| `proposer` | `Address` | — |
| `desc_hash` | `Bytes` | — |
| `start_time` | `u64` | — |
| `end_time` | `u64` | — |
| `for_votes` | `i128` | — |
| `against_votes` | `i128` | — |
| `abstain_votes` | `i128` | — |
| `canceled` | `bool` | — |
| `queued` | `bool` | — |
| `executed` | `bool` | — |
| `exec_data` | `Bytes` | — |

### Error Codes

| Variant | Code | Description |
|---|---|---|
| `NotInitialized` | 300 | — |
| `AlreadyInitialized` | 301 | — |
| `InvalidState` | 304 | — |
| `VotingClosed` | 370 | — |
| `AlreadyVoted` | 371 | — |
| `NotQueued` | 372 | — |
| `ProposalDisputed` | 373 | — |
| `ProposalNotFound` | 450 | — |
| `ProposalNotSuccessful` | 451 | — |
| `AlreadyExecuted` | 452 | — |
| `ProposalThresholdNotMet` | 530 | — |
| `NoVotingPower` | 531 | — |
| `Overflow` | 580 | — |
| `InvalidVoteType` | 280 | — |

---

## healthcare_data_conversion

### Functions

| Function | Parameters | Returns | Description |
|---|---|---|---|
| `initialize` | `env: Env, admin: Address` | `Result<bool, Error>` | Initialize the healthcare data conversion contract |
| `get_conversion_rule` | `env: Env, rule_id: String` | `Result<ConversionRule, Error>` | Get conversion rule |
| `get_coding_mapping` | `env: Env, mapping_id: String` | `Result<CodingMapping, Error>` | Get coding mapping |
| `get_conversion_request` | `env: Env, request_id: u64` | `Result<ConversionRequest, Error>` | Get conversion request details |
| `pause` | `env: Env, admin: Address` | `Result<bool, Error>` | Pause contract operations |
| `resume` | `env: Env, admin: Address` | `Result<bool, Error>` | Resume contract operations |

### Types

#### `enum DataFormat`

| Variant | Value | Description |
|---|---|---|
| `FHIRJSON` | 0 | — |
| `FHIRXML` | 1 | — |
| `HL7v2` | 2 | — |
| `CDA` | 3 | — |
| `HL7v3` | 4 | — |
| `CCD` | 5 | — |
| `Continuity` | — | — |
| `of` | — | — |
| `Care` | — | — |
| `Document` | — | — |
| `C32` | 6 | — |
| `Consolidated` | — | — |
| `CDA` | — | — |
| `PDF` | 7 | — |
| `CSV` | 8 | — |

#### `enum FieldType`

| Variant | Value | Description |
|---|---|---|
| `String` | — | — |
| `Integer` | — | — |
| `Decimal` | — | — |
| `DateTime` | — | — |
| `Boolean` | — | — |
| `Code` | — | — |
| `Array` | — | — |
| `Object` | — | — |

#### `struct ConversionRule`

| Field | Type | Description |
|---|---|---|
| `rule_id` | `String` | — |
| `source_format` | `DataFormat` | — |
| `target_format` | `DataFormat` | — |
| `source_path` | `String` | — |
| `target_path` | `String` | — |
| `transformation_type` | `String` | — |
| `field_type` | `FieldType` | — |
| `mapping_table_ref` | `String` | — |
| `validation_rules` | `Vec<String>` | — |
| `is_active` | `bool` | — |

#### `struct CodingMapping`

| Field | Type | Description |
|---|---|---|
| `mapping_id` | `String` | — |
| `source_code_system` | `String` | — |
| `target_code_system` | `String` | — |
| `source_code` | `String` | — |
| `target_code` | `String` | — |
| `source_description` | `String` | — |
| `target_description` | `String` | — |
| `confidence_score` | `u32` | — |
| `backward_mapping` | `Option<String>` | — |
| `effective_date` | `String` | — |
| `end_date` | `String` | — |

#### `struct FormatSpecification`

| Field | Type | Description |
|---|---|---|
| `format` | `DataFormat` | — |
| `version` | `String` | — |
| `mime_type` | `String` | — |
| `encoding` | `String` | — |
| `character_set` | `String` | — |
| `supported_resources` | `Vec<String>` | — |
| `description` | `String` | — |
| `standard_url` | `String` | — |

#### `struct ConversionRequest`

| Field | Type | Description |
|---|---|---|
| `request_id` | `u64` | — |
| `source_format` | `DataFormat` | — |
| `target_format` | `DataFormat` | — |
| `source_data_hash` | `BytesN<32>` | — |
| `target_data_hash` | `BytesN<32>` | — |
| `conversion_timestamp` | `u64` | — |
| `requester` | `Address` | — |
| `status` | `String` | — |
| `error_details` | `String` | — |

#### `struct ValidationResult`

| Field | Type | Description |
|---|---|---|
| `validation_id` | `u64` | — |
| `source_format` | `DataFormat` | — |
| `target_format` | `DataFormat` | — |
| `is_valid` | `bool` | — |
| `validation_errors` | `Vec<String>` | — |
| `validation_warnings` | `Vec<String>` | — |
| `validated_at` | `u64` | — |

#### `struct LossyConversionWarning`

| Field | Type | Description |
|---|---|---|
| `warning_id` | `String` | — |
| `conversion_request_id` | `u64` | — |
| `lost_fields` | `Vec<String>` | — |
| `data_loss_percentage` | `u32` | — |
| `mitigation_recommendation` | `String` | — |

#### `enum Error`

| Variant | Value | Description |
|---|---|---|
| `NotAuthorized` | 1 | — |
| `ContractPaused` | 2 | — |
| `RuleNotFound` | 3 | — |
| `CodingMappingNotFound` | 4 | — |
| `FormatNotSupported` | 5 | — |
| `ConversionFailed` | 6 | — |
| `ValidationFailed` | 7 | — |
| `InvalidConversionRequest` | 8 | — |
| `SourceFormatNotSupported` | 9 | — |
| `TargetFormatNotSupported` | 10 | — |
| `MappingTableNotFound` | 11 | — |
| `DuplicateRule` | 12 | — |
| `IncompatibleFormats` | 13 | — |
| `DataLossWarning` | 14 | — |
| `InvalidMappingData` | 15 | — |
| `OperationFailed` | 16 | — |

### Examples

#### `test_initialize`

```rust
let env = Env::default();
    env.mock_all_auths();

    let (client, _id) = setup(&env);
    let admin = Address::generate(&env);
    assert!(client.initialize(&admin));

    // Double initialization should fail
    let result = client.try_initialize(&admin);
```

#### `test_initialize_unauthorized`

```rust
let env = Env::default();
    env.mock_all_auths();

    let (client, _id) = setup(&env);
    let admin = Address::generate(&env);
    let other = Address::generate(&env);

    // First init as admin
    assert!(client.initialize(&admin));
```

#### `test_register_and_get_conversion_rule`

```rust
let env = Env::default();
    env.mock_all_auths();

    let (client, _id) = setup(&env);
    let admin = Address::generate(&env);
    client.initialize(&admin);

    let rule = ConversionRule {
        rule_id: String::from_str(&env, "rule-001"),
```

---

## healthcare_reputation

### Functions

| Function | Parameters | Returns | Description |
|---|---|---|---|
| `initialize` | `env: Env, admin: Address` | `Result<(), Error>` | — |
| `get_reputation_score` | `env: Env, provider: Address` | `Result<u32, Error>` | — |

### Types

#### `enum Error`

| Variant | Value | Description |
|---|---|---|
| `AlreadyInitialized` | 1 | — |
| `NotInitialized` | 2 | — |
| `NotAuthorized` | 3 | — |
| `ProviderNotFound` | 4 | — |
| `CredentialNotFound` | 5 | — |
| `InvalidCredentialType` | 6 | — |
| `CredentialExpired` | 7 | — |
| `CredentialRevoked` | 8 | — |
| `DuplicateCredential` | 9 | — |
| `InvalidRating` | 10 | — |
| `FeedbackNotFound` | 11 | — |
| `DisputeNotFound` | 12 | — |
| `InsufficientReputation` | 13 | — |
| `NotVerifiedProvider` | 14 | — |
| `InvalidConductEntry` | 15 | — |
| `ConductEntryNotFound` | 16 | — |

#### `enum CredentialType`

| Variant | Value | Description |
|---|---|---|
| `MedicalLicense` | 0 | — |
| `BoardCertification` | 1 | — |
| `Specialization` | 2 | — |
| `DEARegistration` | 3 | — |
| `StateLicense` | 4 | — |
| `HospitalPrivileges` | 5 | — |
| `InsuranceCredentials` | 6 | — |
| `ContinuingEducation` | 7 | — |

#### `struct ProviderCredential`

| Field | Type | Description |
|---|---|---|
| `credential_id` | `BytesN<32>` | — |
| `provider` | `Address` | — |
| `credential_type` | `CredentialType` | — |
| `issuer` | `Address` | — |
| `issue_date` | `u64` | — |
| `expiration_date` | `u64` | — |
| `credential_hash` | `BytesN<32>` | — |
| `is_active` | `bool` | — |
| `verification_status` | `VerificationStatus` | — |

#### `enum VerificationStatus`

| Variant | Value | Description |
|---|---|---|
| `Pending` | 0 | — |
| `Verified` | 1 | — |
| `Rejected` | 2 | — |
| `Expired` | 3 | — |
| `Revoked` | 4 | — |

#### `struct PatientFeedback`

| Field | Type | Description |
|---|---|---|
| `feedback_id` | `BytesN<32>` | — |
| `provider` | `Address` | — |
| `patient` | `Address` | — |
| `rating` | `u32` | — |
| `comment` | `String` | — |
| `timestamp` | `u64` | — |
| `is_verified` | `bool` | — |
| `feedback_type` | `FeedbackType` | — |

#### `enum FeedbackType`

| Variant | Value | Description |
|---|---|---|
| `General` | 0 | — |
| `Treatment` | 1 | — |
| `Communication` | 2 | — |
| `BedsideManner` | 3 | — |
| `WaitTime` | 4 | — |
| `Facility` | 5 | — |

#### `struct ConductEntry`

| Field | Type | Description |
|---|---|---|
| `entry_id` | `BytesN<32>` | — |
| `provider` | `Address` | — |
| `conduct_type` | `ConductType` | — |
| `description` | `String` | — |
| `severity` | `u32` | — |
| `reporter` | `Address` | — |
| `timestamp` | `u64` | — |
| `is_verified` | `bool` | — |
| `action_taken` | `String` | — |

#### `enum ConductType`

| Variant | Value | Description |
|---|---|---|
| `Positive` | 0 | — |
| `Complaint` | 1 | — |
| `Malpractice` | 2 | — |
| `EthicsViolation` | 3 | — |
| `ProfessionalAchievement` | 4 | — |
| `CommunityService` | 5 | — |

#### `struct ReputationDispute`

| Field | Type | Description |
|---|---|---|
| `dispute_id` | `BytesN<32>` | — |
| `provider` | `Address` | — |
| `challenger` | `Address` | — |
| `dispute_type` | `DisputeType` | — |
| `description` | `String` | — |
| `evidence` | `Vec<String>` | — |
| `timestamp` | `u64` | — |
| `status` | `DisputeStatus` | — |
| `resolution` | `String` | — |

#### `enum DisputeType`

| Variant | Value | Description |
|---|---|---|
| `Credential` | 0 | — |
| `Feedback` | 1 | — |
| `Conduct` | 2 | — |
| `Score` | 3 | — |

#### `enum DisputeStatus`

| Variant | Value | Description |
|---|---|---|
| `Pending` | 0 | — |
| `UnderReview` | 1 | — |
| `Resolved` | 2 | — |
| `Rejected` | 3 | — |

#### `struct ReputationComponents`

| Field | Type | Description |
|---|---|---|
| `credential_score` | `u32` | — |
| `feedback_score` | `u32` | — |
| `conduct_score` | `u32` | — |
| `experience_score` | `u32` | — |
| `total_score` | `u32` | — |

#### `enum DataKey`

| Variant | Value | Description |
|---|---|---|
| `Admin` | — | — |
| `Initialized` | — | — |
| `ProviderCredential` | — | — |
| `Address` | — | — |
| `BytesN` | — | — |
| `32` | — | — |
| `ProviderCredentials` | — | — |
| `Address` | — | — |
| `Vec` | — | — |
| `BytesN` | — | — |
| `32` | — | — |
| `PatientFeedback` | — | — |
| `BytesN` | — | — |
| `32` | — | — |
| `ProviderFeedback` | — | — |
| `Address` | — | — |
| `Vec` | — | — |
| `BytesN` | — | — |
| `32` | — | — |
| `ConductEntry` | — | — |
| `BytesN` | — | — |
| `32` | — | — |
| `ProviderConduct` | — | — |
| `Address` | — | — |
| `Vec` | — | — |
| `BytesN` | — | — |
| `32` | — | — |
| `ReputationDispute` | — | — |
| `BytesN` | — | — |
| `32` | — | — |
| `ProviderDisputes` | — | — |
| `Address` | — | — |
| `Vec` | — | — |
| `BytesN` | — | — |
| `32` | — | — |
| `ReputationScore` | — | — |
| `Address` | — | — |
| `ReputationComponents` | — | — |
| `Address` | — | — |
| `CredentialVerification` | — | — |
| `BytesN` | — | — |
| `32` | — | — |
| `ExpirationNotification` | — | — |
| `Address` | — | — |
| `u64` | — | — |

---

## homomorphic_registry

### Functions

| Function | Parameters | Returns | Description |
|---|---|---|---|
| `initialize` | `env: Env, admin: Address` | `Result<(), Error>` | — |
| `get_context` | `env: Env, context_id: BytesN<32>` | `Result<Option<HEContext>, Error>` | — |

### Types

#### `enum HEScheme`

| Variant | Value | Description |
|---|---|---|
| `Paillier` | — | — |
| `BFV` | — | — |
| `BGV` | — | — |
| `CKKS` | — | — |
| `TFHE` | — | — |
| `Custom` | — | — |
| `u32` | — | — |

#### `struct HEContext`

| Field | Type | Description |
|---|---|---|
| `context_id` | `BytesN<32>` | — |
| `scheme` | `HEScheme` | — |
| `params_ref` | `String` | — |
| `params_hash` | `BytesN<32>` | — |
| `created_at` | `u64` | — |
| `is_active` | `bool` | — |

#### `struct EncryptedComputation`

| Field | Type | Description |
|---|---|---|
| `computation_id` | `BytesN<32>` | — |
| `context_id` | `BytesN<32>` | — |
| `submitter` | `Address` | — |
| `ciphertext_ref` | `String` | — |
| `ciphertext_hash` | `BytesN<32>` | — |
| `proof_ref` | `String` | — |
| `proof_hash` | `BytesN<32>` | — |
| `submitted_at` | `u64` | — |

#### `struct FHEKeyBundle`

| Field | Type | Description |
|---|---|---|
| `key_id` | `BytesN<32>` | — |
| `context_id` | `BytesN<32>` | — |
| `version` | `u32` | — |
| `public_key_ref` | `String` | — |
| `eval_key_ref` | `String` | — |
| `relin_key_ref` | `String` | — |
| `galois_key_ref` | `String` | — |
| `key_hash` | `BytesN<32>` | — |
| `created_at` | `u64` | — |
| `is_active` | `bool` | — |

#### `struct PerformanceProfile`

| Field | Type | Description |
|---|---|---|
| `context_id` | `BytesN<32>` | — |
| `batching_enabled` | `bool` | — |
| `max_batch_size` | `u32` | — |
| `lazy_relinearization` | `bool` | — |
| `auto_bootstrap` | `bool` | — |
| `bootstrap_threshold` | `u32` | — |
| `created_at` | `u64` | — |
| `updated_at` | `u64` | — |

#### `struct EncryptedVector`

| Field | Type | Description |
|---|---|---|
| `ciphertext_id` | `BytesN<32>` | — |
| `context_id` | `BytesN<32>` | — |
| `owner` | `Address` | — |
| `scheme` | `HEScheme` | — |
| `scale` | `u32` | — |
| `noise_budget` | `u32` | — |
| `multiplicative_depth` | `u32` | — |
| `slots` | `Vec<i128>` | — |
| `created_at` | `u64` | — |
| `last_bootstrapped_at` | `u64` | — |

#### `struct EncryptedStats`

| Field | Type | Description |
|---|---|---|
| `ciphertext_id` | `BytesN<32>` | — |
| `count` | `u32` | — |
| `sum` | `i128` | — |
| `mean_scaled` | `i128` | — |
| `variance_scaled` | `i128` | — |
| `min` | `i128` | — |
| `max` | `i128` | — |

#### `enum DataKey`

| Variant | Value | Description |
|---|---|---|
| `Initialized` | — | — |
| `Admin` | — | — |
| `Context` | — | — |
| `BytesN` | — | — |
| `32` | — | — |
| `Computation` | — | — |
| `BytesN` | — | — |
| `32` | — | — |
| `KeyBundle` | — | — |
| `BytesN` | — | — |
| `32` | — | — |
| `ActiveKey` | — | — |
| `BytesN` | — | — |
| `32` | — | — |
| `Ciphertext` | — | — |
| `BytesN` | — | — |
| `32` | — | — |
| `Profile` | — | — |
| `BytesN` | — | — |
| `32` | — | — |

#### `enum Error`

| Variant | Value | Description |
|---|---|---|
| `AlreadyInitialized` | 1 | — |
| `NotInitialized` | 2 | — |
| `NotAuthorized` | 3 | — |
| `ContextNotFound` | 4 | — |
| `ContextInactive` | 5 | — |
| `InvalidInput` | 6 | — |
| `ComputationAlreadyExists` | 7 | — |
| `CiphertextNotFound` | 8 | — |
| `CiphertextAlreadyExists` | 9 | — |
| `SchemeMismatch` | 10 | — |
| `IncompatibleDimensions` | 11 | — |
| `NoiseBudgetExhausted` | 12 | — |
| `ArithmeticOverflow` | 13 | — |
| `KeyNotFound` | 14 | — |

### Examples

#### `context_and_submission_flow`

```rust
let env = Env::default();
    env.mock_all_auths();

    let (client, _id) = setup(&env);
    let admin = Address::generate(&env);
    client.initialize(&admin);

    let ctx_id = BytesN::from_array(&env, &[7u8; 32]);
    let params_ref = String::from_str(&env, "ipfs://he-params");
```

#### `ckks_secure_stats_and_ml_inference_flow`

```rust
let env = Env::default();
    env.mock_all_auths();
    env.ledger().set_timestamp(1_000);
    let (client, _id) = setup(&env);

    let admin = Address::generate(&env);
    let analyst = Address::generate(&env);
    client.initialize(&admin);
```

#### `bgv_exact_computation_and_noise_bootstrap`

```rust
let env = Env::default();
    env.mock_all_auths();
    env.ledger().set_timestamp(2_000);
    let (client, _id) = setup(&env);

    let admin = Address::generate(&env);
    let submitter = Address::generate(&env);
    client.initialize(&admin);
```

---

## identity_registry

### Functions

| Function | Parameters | Returns | Description |
|---|---|---|---|
| `health_check` | `env: Env` | `(Symbol, u32, u64)` | Perform a health check on the contract. Returns (status, version, timestamp) with standardized status values: "OK", "PAUSED", "NOT_INIT", "DEGRADED". |
| `is_paused` | `env: Env` | `bool` | Returns true if the contract is currently paused. |
| `pause` | `env: Env, caller: Address` | `Result<bool, Error>` | — |
| `unpause` | `env: Env, caller: Address` | `Result<bool, Error>` | — |
| `initialize_legacy` | `env: Env, owner: Address, rbac_contract: Address` | `()` | — |
| `resolve_did` | `env: Env, subject: Address` | `Result<DIDDocument, Error>` | Resolve a DID Document by subject address |
| `resolve_did_by_string` | `env: Env, did_string: String` | `Result<DIDDocument, Error>` | Resolve a DID Document by DID string |
| `deactivate_did` | `env: Env, subject: Address` | `Result<(), Error>` | Deactivate a DID (soft delete) |
| `get_subject_credentials` | `env: Env, subject: Address` | `Vec<VerifiableCredential>` | Get all credentials for a subject |
| `set_recovery_threshold` | `env: Env, subject: Address, threshold: u32` | `Result<(), Error>` | Set recovery threshold |
| `approve_recovery` | `env: Env, guardian: Address, request_id: u64` | `Result<(), Error>` | Approve a recovery request |
| `execute_recovery` | `env: Env, request_id: u64` | `Result<(), Error>` | Execute recovery after timelock and threshold met |
| `cancel_recovery` | `env: Env, subject: Address` | `Result<(), Error>` | Cancel a recovery request (only subject with existing key) |
| `remove_service` | `env: Env, subject: Address, service_id: String` | `Result<(), Error>` | Remove/deactivate a service endpoint |
| `add_verifier` | `env: Env, verifier: Address` | `Result<(), Error>` | Add a verifier (only owner can do this).  SECURITY (issue #43): the previously-blind `assign_role(Staff)` call has been guarded so that a verifier who already holds a higher-privileged role (Admin, Doctor, Researcher) keeps that role untouched. Only verifiers without any of those higher roles receive the `Staff` marker. Either way, the local `Verifier(addr) -> true` flag is set so the contract-level verifier registry stays consistent.  Trade-off (intentional): if a verifier was originally added while only holding `Staff` and is later promoted to a higher role (Admin/Doctor /Researcher) without an intervening `remove_verifier`, the `Staff` row will remain in RBAC. `remove_verifier` will then leave it alone because of the higher-role guard. Operators that need the row removed should call `remove_verifier` before the promotion. |
| `remove_verifier` | `env: Env, verifier: Address` | `Result<(), Error>` | Remove a verifier (only owner can do this).  SECURITY (issue #43): as with `add_verifier`, the `remove_role(Staff)` call is now skipped whenever the target already holds a higher-privileged role (Admin, Doctor, Researcher). Stripping `Staff` from those users could be misinterpreted as a privilege revocation and risks disturbing the higher-privileged role state, so the call is intentionally a no-op in that case. The local `Verifier(addr)` flag is always cleared.  Trade-off (intentional, mirrors `add_verifier`): if a verifier was originally added while only holding `Staff` and was later promoted to a higher role, the pre-existing `Staff` row is preserved by this function alongside the higher role. To clear `Staff` from such an address, demote it back to non-staff roles first. |
| `is_verifier` | `env: Env, account: Address` | `bool` | Check if an address is a verifier |
| `get_owner` | `env: Env` | `Result<Address, Error>` | Get the contract owner |
| `get_identity_hash` | `env: Env, subject: Address` | `Option<BytesN<32>>` | Get identity hash for a subject (legacy) |
| `get_identity_meta` | `env: Env, subject: Address` | `Option<String>` | Get identity metadata for a subject (legacy) |
| `is_attested` | `env: Env, subject: Address, claim_hash: BytesN<32>` | `bool` | Check if a specific attestation is active (legacy) |
| `get_attestations` | `env: Env, subject: Address` | `Vec<BytesN<32>>` | Get all active attestations for a subject (legacy) |
| `withdraw_stake` | `env: Env, provider: Address` | `Result<i128, Error>` | Withdraw stake after lock period if not slashed and in good standing. |
| `has_role` | `env: Env, address: Address, role: RbacRole` | `Result<bool, RbacError>` | — |
| `assign_role` | `env: Env, address: Address, role: RbacRole` | `Result<bool, RbacError>` | — |
| `remove_role` | `env: Env, address: Address, role: RbacRole` | `Result<bool, RbacError>` | — |

### Types

#### `enum RbacRole`

| Variant | Value | Description |
|---|---|---|
| `Admin` | 0 | — |
| `Doctor` | 1 | — |
| `Patient` | 2 | — |
| `Staff` | 3 | — |
| `Insurer` | 4 | — |
| `Researcher` | 5 | — |
| `Auditor` | 6 | — |
| `Service` | 7 | — |

#### `enum RbacError`

| Variant | Value | Description |
|---|---|---|
| `Unauthorized` | 100 | — |
| `NotInitialized` | 300 | — |
| `AlreadyInitialized` | 301 | — |

#### `enum VerificationMethodType`

| Variant | Value | Description |
|---|---|---|
| `Ed25519VerificationKey2020` | — | — |
| `EcdsaSecp256k1VerifKey2019` | — | — |
| `X25519KeyAgreementKey2020` | — | — |
| `JsonWebKey2020` | — | — |
| `FIDO2` | — | — |
| `WebAuthn` | — | — |
| `Ed25519` | — | — |
| `EdDSA` | — | — |
| `authenticator` | — | — |
| `key` | — | — |
| `algorithm` | — | — |
| `tag` | 1 | — |
| `Fido2EdDsa2024` | — | — |
| `FIDO2` | — | — |
| `WebAuthn` | — | — |
| `P` | — | — |
| `256` | — | — |
| `ES256` | — | — |
| `authenticator` | — | — |
| `key` | — | — |
| `algorithm` | — | — |
| `tag` | 2 | — |
| `Fido2Es2562024` | — | — |

#### `enum VerificationRelationship`

| Variant | Value | Description |
|---|---|---|
| `Authentication` | — | — |
| `AssertionMethod` | — | — |
| `KeyAgreement` | — | — |
| `CapabilityInvocation` | — | — |
| `CapabilityDelegation` | — | — |

#### `struct VerificationMethod`

| Field | Type | Description |
|---|---|---|
| `id` | `String` | — |
| `method_type` | `VerificationMethodType` | — |
| `controller` | `Address` | — |
| `public_key` | `BytesN<32>` | — |
| `is_active` | `bool` | — |
| `created` | `u64` | — |
| `last_rotated` | `u64` | — |

#### `struct ServiceEndpoint`

| Field | Type | Description |
|---|---|---|
| `id` | `String` | — |
| `service_type` | `String` | — |
| `endpoint` | `String` | — |
| `is_active` | `bool` | — |

#### `enum DIDStatus`

| Variant | Value | Description |
|---|---|---|
| `Active` | — | — |
| `Deactivated` | — | — |
| `RecoveryPending` | — | — |

#### `struct DIDDocument`

| Field | Type | Description |
|---|---|---|
| `id` | `String` | — |
| `controller` | `Address` | — |
| `also_known_as` | `Vec<String>` | — |
| `verification_methods` | `Vec<VerificationMethod>` | — |
| `authentication` | `Vec<String>` | — |
| `assertion_method` | `Vec<String>` | — |
| `key_agreement` | `Vec<String>` | — |
| `capability_invocation` | `Vec<String>` | — |
| `capability_delegation` | `Vec<String>` | — |
| `services` | `Vec<ServiceEndpoint>` | — |
| `status` | `DIDStatus` | — |
| `created` | `u64` | — |
| `updated` | `u64` | — |
| `version` | `u32` | — |
| `previous_hash` | `BytesN<32>` | — |

#### `enum CredentialType`

| Variant | Value | Description |
|---|---|---|
| `MedicalLicense` | — | — |
| `SpecialistCertification` | — | — |
| `HospitalAffiliation` | — | — |
| `ResearchAuthorization` | — | — |
| `PatientConsent` | — | — |
| `EmergencyAccess` | — | — |
| `DataAccessPermission` | — | — |

#### `struct VerifiableCredential`

| Field | Type | Description |
|---|---|---|
| `id` | `BytesN<32>` | — |
| `credential_type` | `CredentialType` | — |
| `issuer` | `Address` | — |
| `subject` | `Address` | — |
| `issuance_date` | `u64` | — |
| `expiration_date` | `u64` | — |
| `credential_hash` | `BytesN<32>` | — |
| `credential_uri` | `String` | — |
| `is_revoked` | `bool` | — |
| `revoked_at` | `u64` | — |
| `revocation_reason` | `String` | — |

#### `enum CredentialStatus`

| Variant | Value | Description |
|---|---|---|
| `Valid` | — | — |
| `Revoked` | — | — |
| `Expired` | — | — |
| `NotFound` | — | — |

#### `struct RecoveryGuardian`

| Field | Type | Description |
|---|---|---|
| `address` | `Address` | — |
| `weight` | `u32` | — |
| `added_at` | `u64` | — |

#### `struct RecoveryRequest`

| Field | Type | Description |
|---|---|---|
| `request_id` | `u64` | — |
| `subject` | `Address` | — |
| `new_controller` | `Address` | — |
| `new_primary_key` | `BytesN<32>` | — |
| `created_at` | `u64` | — |
| `approvals` | `Vec<Address>` | — |
| `total_weight` | `u32` | — |
| `executed` | `bool` | — |

#### `struct IdentityRecord`

| Field | Type | Description |
|---|---|---|
| `hash` | `BytesN<32>` | — |
| `meta` | `String` | — |
| `registered_by` | `Address` | — |

#### `struct Attestation`

| Field | Type | Description |
|---|---|---|
| `claim_hash` | `BytesN<32>` | — |
| `verifier` | `Address` | — |
| `is_active` | `bool` | — |

#### `struct ProviderStake`

| Field | Type | Description |
|---|---|---|
| `provider` | `Address` | — |
| `token_address` | `Address` | — |
| `amount` | `i128` | — |
| `locked_until` | `u64` | — |
| `slashed` | `bool` | — |
| `deposited_at` | `u64` | — |

#### `enum DataKey`

| Variant | Value | Description |
|---|---|---|
| `Contract` | — | — |
| `State` | — | — |
| `Owner` | — | — |
| `Initialized` | — | — |
| `NetworkId` | — | — |
| `RbacContract` | — | — |
| `Paused` | — | — |
| `Verifier` | — | — |
| `Management` | — | — |
| `Verifier` | — | — |
| `Address` | — | — |
| `Legacy` | — | — |
| `Identity` | — | — |
| `backward` | — | — |
| `compatibility` | — | — |
| `IdentityHash` | — | — |
| `Address` | — | — |
| `Attestation` | — | — |
| `Address` | — | — |
| `BytesN` | — | — |
| `32` | — | — |
| `SubjectAttestations` | — | — |
| `Address` | — | — |
| `DID` | — | — |
| `Document` | — | — |
| `Storage` | — | — |
| `DIDDocument` | — | — |
| `Address` | — | — |
| `DIDByString` | — | — |
| `String` | — | — |
| `Verification` | — | — |
| `Methods` | — | — |
| `VerificationMethod` | — | — |
| `Address` | — | — |
| `String` | — | — |
| `Verifiable` | — | — |
| `Credentials` | — | — |
| `Credential` | — | — |
| `BytesN` | — | — |
| `32` | — | — |
| `SubjectCredentials` | — | — |
| `Address` | — | — |
| `IssuerCredentials` | — | — |
| `Address` | — | — |
| `CredentialsByType` | — | — |
| `Address` | — | — |
| `CredentialType` | — | — |
| `Recovery` | — | — |
| `System` | — | — |
| `RecoveryGuardians` | — | — |
| `Address` | — | — |
| `RecoveryThreshold` | — | — |
| `Address` | — | — |
| `RecoveryRequest` | — | — |
| `u64` | — | — |
| `ActiveRecovery` | — | — |
| `Address` | — | — |
| `RecoveryCounter` | — | — |
| `SubjectRecoveryId` | — | — |
| `Address` | — | — |
| `Key` | — | — |
| `Rotation` | — | — |
| `LastKeyRotation` | — | — |
| `Address` | — | — |
| `KeyRotationCooldown` | — | — |
| `Provider` | — | — |
| `Staking` | — | — |
| `StakeInfo` | — | — |
| `Address` | — | — |

#### `enum MockRbacKey`

| Variant | Value | Description |
|---|---|---|
| `Role` | — | — |
| `Address` | — | — |
| `RbacRole` | — | — |

### Error Codes

| Variant | Code | Description |
|---|---|---|
| `Unauthorized` | 100 | — |
| `NotVerifier` | 110 | — |
| `CannotRemoveOwner` | 111 | — |
| `InvalidRecoveryGuardian` | 120 | — |
| `InsufficientGuardianApprovals` | 121 | — |
| `GuardianWeightTooHigh` | 122 | — |
| `InvalidRecoveryThreshold` | 123 | — |
| `InvalidInput` | 200 | — |
| `InputTooLong` | 201 | — |
| `InvalidVerificationMethod` | 250 | — |
| `InvalidCredentialType` | 251 | — |
| `InvalidServiceEndpoint` | 252 | — |
| `NotInitialized` | 300 | — |
| `AlreadyInitialized` | 301 | — |
| `ContractPaused` | 302 | — |
| `RecoveryNotInitiated` | 360 | — |
| `RecoveryAlreadyPending` | 361 | — |
| `RecoveryTimelockNotElapsed` | 362 | — |
| `RecoveryAlreadyExecuted` | 363 | — |
| `VerificationMethodNotFound` | 450 | — |
| `ArithmeticOverflow` | 500 | — |
| `CredentialNotFound` | 460 | — |
| `AttestationNotFound` | 461 | — |
| `ServiceNotFound` | 462 | — |
| `DIDNotFound` | 470 | — |
| `DIDAlreadyExists` | 471 | — |
| `DIDDeactivated` | 472 | — |
| `CredentialExpired` | 605 | — |
| `CredentialRevoked` | 606 | — |
| `KeyRotationCooldown` | 603 | — |

---

## ihe_integration

### Functions

| Function | Parameters | Returns | Description |
|---|---|---|---|
| `initialize` | `env: Env, admin: Address` | `Result<(), Error>` | — |
| `atna_get_event` | `env: Env, event_id: u64` | `Result<ATNAAuditEvent, Error>` | Retrieve an ATNA audit event by ID |
| `bppc_revoke_consent` | `env: Env, author: Address, consent_id: u64` | `Result<(), Error>` | Revoke a privacy consent |
| `bppc_verify_consent` | `env: Env, consent_id: u64` | `Result<BPPCConsent, Error>` | Verify consent is active and not expired |
| `dsg_verify_signature` | `env: Env, signature_id: u64` | `Result<DSGSignature, Error>` | Verify a document signature by signature ID |
| `hpd_get_provider` | `env: Env, provider_id: u64` | `Result<HPDProvider, Error>` | Query a provider by ID |
| `svs_get_value_set_by_oid` | `env: Env, oid: String` | `Result<SVSValueSet, Error>` | Retrieve a value set by OID |
| `connectathon_is_compliant` | `env: Env, profile: IHEProfile` | `bool` | Check if a profile passes all recorded Connectathon tests |

### Types

#### `enum IHEProfile`

| Variant | Value | Description |
|---|---|---|
| `XDS` | — | — |
| `Cross` | — | — |
| `Enterprise` | — | — |
| `Document` | — | — |
| `Sharing` | — | — |
| `PIX` | — | — |
| `Patient` | — | — |
| `Identifier` | — | — |
| `Cross` | — | — |
| `referencing` | — | — |
| `PDQ` | — | — |
| `Patient` | — | — |
| `Demographics` | — | — |
| `Query` | — | — |
| `ATNA` | — | — |
| `Audit` | — | — |
| `Trail` | — | — |
| `and` | — | — |
| `Node` | — | — |
| `Authentication` | — | — |
| `XCA` | — | — |
| `Cross` | — | — |
| `Community` | — | — |
| `Access` | — | — |
| `MPI` | — | — |
| `Master` | — | — |
| `Patient` | — | — |
| `Index` | — | — |
| `XDR` | — | — |
| `Cross` | — | — |
| `Enterprise` | — | — |
| `Document` | — | — |
| `Reliable` | — | — |
| `Interchange` | — | — |
| `XDM` | — | — |
| `Cross` | — | — |
| `Enterprise` | — | — |
| `Document` | — | — |
| `Media` | — | — |
| `Interchange` | — | — |
| `CT` | — | — |
| `Consistent` | — | — |
| `Time` | — | — |
| `BPPC` | — | — |
| `Basic` | — | — |
| `Patient` | — | — |
| `Privacy` | — | — |
| `Consents` | — | — |
| `DSG` | — | — |
| `Document` | — | — |
| `Digital` | — | — |
| `Signature` | — | — |
| `HPD` | — | — |
| `Healthcare` | — | — |
| `Provider` | — | — |
| `Directory` | — | — |
| `SVS` | — | — |
| `Sharing` | — | — |
| `Value` | — | — |
| `Sets` | — | — |

#### `enum HL7MessageType`

| Variant | Value | Description |
|---|---|---|
| `HL7` | — | — |
| `v2` | — | — |
| `V2ADT` | — | — |
| `Admit` | — | — |
| `Discharge` | — | — |
| `Transfer` | — | — |
| `V2ORM` | — | — |
| `Order` | — | — |
| `Message` | — | — |
| `V2ORU` | — | — |
| `Observation` | — | — |
| `Result` | — | — |
| `V2MFN` | — | — |
| `Master` | — | — |
| `File` | — | — |
| `Notification` | — | — |
| `V2QBP` | — | — |
| `Query` | — | — |
| `By` | — | — |
| `Parameter` | — | — |
| `V2RSP` | — | — |
| `Segment` | — | — |
| `Pattern` | — | — |
| `Response` | — | — |
| `V2ACK` | — | — |
| `General` | — | — |
| `Acknowledgment` | — | — |
| `HL7` | — | — |
| `v3` | — | — |
| `V3ClinicalDocument` | — | — |
| `V3PatientQuery` | — | — |
| `V3PatientResponse` | — | — |
| `V3DeviceQuery` | — | — |

#### `enum DocumentStatus`

| Variant | Value | Description |
|---|---|---|
| `Approved` | — | — |
| `Deprecated` | — | — |
| `Submitted` | — | — |

#### `struct XDSDocumentEntry`

| Field | Type | Description |
|---|---|---|
| `document_id` | `String` | — |
| `patient_id` | `String` | — |
| `content_hash` | `BytesN<32>` | — |
| `document_class_code` | `String` | — |
| `document_type_code` | `String` | — |
| `format_code` | `String` | — |
| `healthcare_facility_type` | `String` | — |
| `practice_setting_code` | `String` | — |
| `creation_time` | `u64` | — |
| `author` | `Address` | — |
| `confidentiality_code` | `String` | — |
| `language_code` | `String` | — |
| `hl7_message_type` | `HL7MessageType` | — |
| `status` | `DocumentStatus` | — |
| `repository_unique_id` | `String` | — |
| `submission_set_id` | `String` | — |
| `mime_type` | `String` | — |

#### `struct XDSSubmissionSet`

| Field | Type | Description |
|---|---|---|
| `submission_set_id` | `String` | — |
| `patient_id` | `String` | — |
| `submission_time` | `u64` | — |
| `source_id` | `String` | — |
| `author` | `Address` | — |
| `content_type_code` | `String` | — |
| `document_ids` | `Vec<String>` | — |
| `intended_recipient` | `String` | — |

#### `struct PatientIdentifier`

| Field | Type | Description |
|---|---|---|
| `id_value` | `String` | — |
| `assigning_authority` | `String` | — |
| `identifier_type_code` | `String` | — |

#### `struct PIXCrossReference`

| Field | Type | Description |
|---|---|---|
| `reference_id` | `u64` | — |
| `local_id` | `PatientIdentifier` | — |
| `cross_referenced_ids` | `Vec<PatientIdentifier>` | — |
| `created_at` | `u64` | — |
| `updated_at` | `u64` | — |
| `is_merged` | `bool` | — |

#### `struct PatientDemographics`

| Field | Type | Description |
|---|---|---|
| `patient_id` | `String` | — |
| `given_name` | `String` | — |
| `family_name` | `String` | — |
| `date_of_birth` | `String` | — |
| `administrative_gender` | `String` | — |
| `street_address` | `String` | — |
| `city` | `String` | — |
| `state` | `String` | — |
| `postal_code` | `String` | — |
| `country_code` | `String` | — |
| `phone_home` | `String` | — |
| `phone_mobile` | `String` | — |
| `mother_maiden_name` | `String` | — |
| `marital_status` | `String` | — |
| `race` | `String` | — |
| `ethnicity` | `String` | — |
| `primary_language` | `String` | — |
| `last_updated` | `u64` | — |
| `assigning_authority` | `String` | — |

#### `struct PDQQuery`

| Field | Type | Description |
|---|---|---|
| `query_id` | `u64` | — |
| `query_parameters` | `Map<String` | — |
| `requesting_system` | `String` | — |
| `query_time` | `u64` | — |
| `hl7_message_type` | `HL7MessageType` | — |
| `domain_filter` | `String` | — |

#### `enum ATNAEventType`

| Variant | Value | Description |
|---|---|---|
| `PatientRecordAccess` | — | — |
| `PatientRecordUpdate` | — | — |
| `UserAuthentication` | — | — |
| `NodeAuthentication` | — | — |
| `DocumentExport` | — | — |
| `DocumentImport` | — | — |
| `QueryRequest` | — | — |
| `QueryResponse` | — | — |
| `SecurityAlert` | — | — |
| `OrderMessage` | — | — |
| `ProcedureRecord` | — | — |

#### `enum ATNAEventOutcome`

| Variant | Value | Description |
|---|---|---|
| `Success` | 0 | — |
| `MinorFailure` | 4 | — |
| `SeriousFailure` | 8 | — |
| `MajorFailure` | 12 | — |

#### `struct ATNAParticipant`

| Field | Type | Description |
|---|---|---|
| `user_id` | `String` | — |
| `user_name` | `String` | — |
| `role_id_code` | `String` | — |
| `is_requestor` | `bool` | — |
| `network_access_point` | `String` | — |

#### `struct ATNAParticipantObject`

| Field | Type | Description |
|---|---|---|
| `object_id_type_code` | `String` | — |
| `object_id` | `String` | — |
| `object_type_code` | `u32` | — |
| `object_sensitivity` | `String` | — |
| `object_query` | `String` | — |

#### `struct ATNAAuditEvent`

| Field | Type | Description |
|---|---|---|
| `event_id` | `u64` | — |
| `event_type` | `ATNAEventType` | — |
| `event_action_code` | `String` | — |
| `event_date_time` | `u64` | — |
| `event_outcome` | `ATNAEventOutcome` | — |
| `source_id` | `String` | — |
| `source_type` | `String` | — |
| `active_participants` | `Vec<ATNAParticipant>` | — |
| `participant_objects` | `Vec<ATNAParticipantObject>` | — |
| `hl7_message_id` | `String` | — |
| `profile` | `IHEProfile` | — |

#### `struct XCAGateway`

| Field | Type | Description |
|---|---|---|
| `gateway_id` | `String` | — |
| `community_id` | `String` | — |
| `gateway_address` | `String` | — |
| `supported_profiles` | `Vec<IHEProfile>` | — |
| `registered_by` | `Address` | — |
| `registration_time` | `u64` | — |
| `is_active` | `bool` | — |

#### `struct MPIMasterPatient`

| Field | Type | Description |
|---|---|---|
| `master_id` | `u64` | — |
| `global_patient_id` | `String` | — |
| `linked_identifiers` | `Vec<PatientIdentifier>` | — |
| `demographics` | `PatientDemographics` | — |
| `created_at` | `u64` | — |
| `updated_at` | `u64` | — |
| `confidence_score` | `u32` | — |

#### `enum ConsentStatus`

| Variant | Value | Description |
|---|---|---|
| `Active` | — | — |
| `Revoked` | — | — |
| `Expired` | — | — |

#### `struct BPPCConsent`

| Field | Type | Description |
|---|---|---|
| `consent_id` | `u64` | — |
| `patient_id` | `String` | — |
| `policy_id` | `String` | — |
| `consent_status` | `ConsentStatus` | — |
| `access_consent_list` | `Vec<String>` | — |
| `date_of_consent` | `u64` | — |
| `expiry_time` | `u64` | — |
| `author` | `Address` | — |
| `document_ref` | `String` | — |

#### `struct DSGSignature`

| Field | Type | Description |
|---|---|---|
| `signature_id` | `u64` | — |
| `document_id` | `String` | — |
| `signer` | `Address` | — |
| `signature_hash` | `BytesN<32>` | — |
| `signature_algorithm` | `String` | — |
| `signing_time` | `u64` | — |
| `certificate_ref` | `String` | — |
| `signature_purpose` | `String` | — |
| `is_valid` | `bool` | — |

#### `enum ProviderType`

| Variant | Value | Description |
|---|---|---|
| `Individual` | — | — |
| `Organization` | — | — |
| `Department` | — | — |

#### `struct HPDProvider`

| Field | Type | Description |
|---|---|---|
| `provider_id` | `u64` | — |
| `provider_type` | `ProviderType` | — |
| `given_name` | `String` | — |
| `family_name` | `String` | — |
| `organization_name` | `String` | — |
| `specialty_code` | `String` | — |
| `license_number` | `String` | — |
| `npi` | `String` | — |
| `address` | `String` | — |
| `electronic_service_info` | `String` | — |
| `registered_by` | `Address` | — |
| `registration_time` | `u64` | — |
| `is_active` | `bool` | — |

#### `struct SVSConcept`

| Field | Type | Description |
|---|---|---|
| `code` | `String` | — |
| `code_system` | `String` | — |
| `code_system_name` | `String` | — |
| `display_name` | `String` | — |
| `level` | `u32` | — |
| `type_code` | `String` | — |

#### `struct SVSValueSet`

| Field | Type | Description |
|---|---|---|
| `value_set_id` | `u64` | — |
| `oid` | `String` | — |
| `name` | `String` | — |
| `version` | `String` | — |
| `status` | `String` | — |
| `description` | `String` | — |
| `concepts` | `Vec<SVSConcept>` | — |
| `effective_date` | `u64` | — |
| `source_url` | `String` | — |
| `registered_by` | `Address` | — |

#### `struct ConnectathonTestResult`

| Field | Type | Description |
|---|---|---|
| `test_id` | `u64` | — |
| `profile` | `IHEProfile` | — |
| `actor_name` | `String` | — |
| `test_name` | `String` | — |
| `passed` | `bool` | — |
| `tested_at` | `u64` | — |
| `tested_by` | `Address` | — |
| `notes` | `String` | — |

#### `enum DataKey`

| Variant | Value | Description |
|---|---|---|
| `Admin` | — | — |
| `Counters` | — | — |
| `NextDocumentId` | — | — |
| `NextPixRefId` | — | — |
| `NextPdqQueryId` | — | — |
| `NextAtnaEventId` | — | — |
| `NextMasterPatientId` | — | — |
| `NextConsentId` | — | — |
| `NextSignatureId` | — | — |
| `NextProviderId` | — | — |
| `NextValueSetId` | — | — |
| `NextTestResultId` | — | — |
| `XDS` | — | — |
| `XDSDocument` | — | — |
| `String` | — | — |
| `document_id` | — | — |
| `XDSDocumentEntry` | — | — |
| `XDSSubmissionSet` | — | — |
| `String` | — | — |
| `submission_set_id` | — | — |
| `XDSSubmissionSet` | — | — |
| `PatientDocuments` | — | — |
| `String` | — | — |
| `patient_id` | — | — |
| `Vec` | — | — |
| `String` | — | — |
| `document` | — | — |
| `IDs` | — | — |
| `PIX` | — | — |
| `PIXCrossRef` | — | — |
| `u64` | — | — |
| `reference_id` | — | — |
| `PIXCrossReference` | — | — |
| `PIXPatientRefs` | — | — |
| `String` | — | — |
| `patient_id` | — | — |
| `Vec` | — | — |
| `u64` | — | — |
| `reference` | — | — |
| `IDs` | — | — |
| `PDQ` | — | — |
| `PatientDemographics` | — | — |
| `String` | — | — |
| `patient_id` | — | — |
| `PatientDemographics` | — | — |
| `PDQQuery` | — | — |
| `u64` | — | — |
| `query_id` | — | — |
| `PDQQuery` | — | — |
| `ATNA` | — | — |
| `ATNAEvent` | — | — |
| `u64` | — | — |
| `event_id` | — | — |
| `ATNAAuditEvent` | — | — |
| `XCA` | — | — |
| `XCAGateway` | — | — |
| `String` | — | — |
| `gateway_id` | — | — |
| `XCAGateway` | — | — |
| `MPI` | — | — |
| `MPIMasterPatient` | — | — |
| `u64` | — | — |
| `master_id` | — | — |
| `MPIMasterPatient` | — | — |
| `MPIGlobalIndex` | — | — |
| `String` | — | — |
| `global_patient_id` | — | — |
| `master_id` | — | — |
| `BPPC` | — | — |
| `BPPCConsent` | — | — |
| `u64` | — | — |
| `consent_id` | — | — |
| `BPPCConsent` | — | — |
| `PatientConsents` | — | — |
| `String` | — | — |
| `patient_id` | — | — |
| `Vec` | — | — |
| `u64` | — | — |
| `consent` | — | — |
| `IDs` | — | — |
| `DSG` | — | — |
| `DSGSignature` | — | — |
| `u64` | — | — |
| `signature_id` | — | — |
| `DSGSignature` | — | — |
| `DocumentSignatures` | — | — |
| `String` | — | — |
| `document_id` | — | — |
| `Vec` | — | — |
| `u64` | — | — |
| `signature` | — | — |
| `IDs` | — | — |
| `HPD` | — | — |
| `HPDProvider` | — | — |
| `u64` | — | — |
| `provider_id` | — | — |
| `HPDProvider` | — | — |
| `SVS` | — | — |
| `SVSValueSet` | — | — |
| `u64` | — | — |
| `value_set_id` | — | — |
| `SVSValueSet` | — | — |
| `SVSValueSetByOid` | — | — |
| `String` | — | — |
| `oid` | — | — |
| `value_set_id` | — | — |
| `Connectathon` | — | — |
| `ConnectathonResult` | — | — |
| `u64` | — | — |
| `test_id` | — | — |
| `ConnectathonTestResult` | — | — |
| `ProfileTestIds` | — | — |
| `IHEProfile` | — | — |
| `profile` | — | — |
| `Vec` | — | — |
| `u64` | — | — |
| `test` | — | — |
| `IDs` | — | — |

#### `enum Error`

| Variant | Value | Description |
|---|---|---|
| `NotInitialized` | 1 | — |
| `AlreadyInitialized` | 2 | — |
| `NotAuthorized` | 3 | — |
| `DocumentNotFound` | 4 | — |
| `DocumentAlreadyExists` | 5 | — |
| `DocumentDeprecated` | 6 | — |
| `PatientNotFound` | 7 | — |
| `CrossReferenceNotFound` | 8 | — |
| `DemographicsNotFound` | 9 | — |
| `AuditEventNotFound` | 10 | — |
| `GatewayNotFound` | 11 | — |
| `GatewayAlreadyExists` | 12 | — |
| `MasterPatientNotFound` | 13 | — |
| `ConsentNotFound` | 14 | — |
| `ConsentRevoked` | 15 | — |
| `ConsentExpired` | 16 | — |
| `SignatureNotFound` | 17 | — |
| `SignatureInvalid` | 18 | — |
| `ProviderNotFound` | 19 | — |
| `ValueSetNotFound` | 20 | — |
| `ValueSetOidExists` | 21 | — |
| `InvalidHL7Message` | 22 | — |
| `ConnectathonTestNotFound` | 23 | — |
| `EmptyPatientId` | 24 | — |
| `EmptyDocumentId` | 25 | — |

### Examples

#### `test_initialize`

```rust
let (_, _, _) = setup();
```

#### `test_double_initialize_fails`

```rust
let (env, admin, client) = setup();
    let _ = env;
    let result = client.try_initialize(&admin);
    assert!(result.is_err());
```

#### `test_xds_register_and_retrieve_document`

```rust
let (env, _, client) = setup();
    let author = Address::generate(&env);
    let entry = make_xds_entry(&env, &author);

    client.xds_register_document(&author, &entry);

    let retrieved = client.xds_retrieve_document(&author, &String::from_str(&env, "DOC-001"));
    assert_eq!(retrieved.document_id, entry.document_id);
    assert_eq!(retrieved.patient_id, entry.patient_id);
```

---

## iot_device_management

### Functions

| Function | Parameters | Returns | Description |
|---|---|---|---|
| `initialize` | `env: Env, admin: Address` | `Result<(), Error>` | — |
| `pause` | `env: Env, admin: Address` | `Result<(), Error>` | — |
| `unpause` | `env: Env, admin: Address` | `Result<(), Error>` | — |
| `set_role` | `env: Env, admin: Address, user: Address, role: Role` | `Result<(), Error>` | — |
| `get_role` | `env: Env, user: Address` | `Role` | — |
| `get_manufacturer` | `env: Env, manufacturer_id: BytesN<32>` | `Result<Manufacturer, Error>` | — |
| `get_device` | `env: Env, device_id: BytesN<32>` | `Result<Device, Error>` | — |
| `get_device_count` | `env: Env` | `u64` | — |
| `get_devices_by_operator` | `env: Env, operator: Address` | `Vec<BytesN<32>>` | — |
| `activate_device` | `env: Env, caller: Address, device_id: BytesN<32>` | `Result<(), Error>` | — |
| `suspend_device` | `env: Env, caller: Address, device_id: BytesN<32>` | `Result<(), Error>` | — |
| `get_device_heartbeats` | `env: Env, device_id: BytesN<32>` | `Result<Vec<Heartbeat>, Error>` | — |
| `get_device_uptime_bps` | `env: Env, device_id: BytesN<32>` | `Result<u32, Error>` | — |
| `get_active_device_count` | `env: Env` | `u64` | — |
| `get_comm_channel` | `env: Env, channel_id: BytesN<32>` | `Result<CommChannel, Error>` | — |
| `get_devices_by_manufacturer` | `env: Env, manufacturer_id: BytesN<32>` | `Vec<BytesN<32>>` | — |
| `get_manufacturer_count` | `env: Env` | `u32` | — |
| `get_crypto_registry_contract` | `env: Env` | `Option<Address>` | Return the configured crypto_registry contract address, if any. |
| `get_manufacturer_crypto_owner` | `env: Env, manufacturer_id: BytesN<32>` | `Option<Address>` | Return the linked crypto_registry owner address for a manufacturer (if any). |

### Types

#### `enum DeviceStatus`

| Variant | Value | Description |
|---|---|---|
| `Provisioning` | 0 | — |
| `Active` | 1 | — |
| `Suspended` | 2 | — |
| `Maintenance` | 3 | — |
| `Decommissioned` | 4 | — |

#### `enum DeviceType`

| Variant | Value | Description |
|---|---|---|
| `VitalSignsMonitor` | 0 | — |
| `BloodPressureMonitor` | 1 | — |
| `GlucoseMonitor` | 2 | — |
| `PulseOximeter` | 3 | — |
| `ECGMonitor` | 4 | — |
| `TemperatureSensor` | 5 | — |
| `InfusionPump` | 6 | — |
| `Ventilator` | 7 | — |
| `WearableSensor` | 8 | — |
| `ImagingDevice` | 9 | — |
| `LabAnalyzer` | 10 | — |
| `Other` | 99 | — |

#### `enum FirmwareStatus`

| Variant | Value | Description |
|---|---|---|
| `Pending` | 0 | — |
| `Approved` | 1 | — |
| `Rejected` | 2 | — |
| `Deprecated` | 3 | — |

#### `enum HealthStatus`

| Variant | Value | Description |
|---|---|---|
| `Healthy` | 0 | — |
| `Degraded` | 1 | — |
| `Critical` | 2 | — |
| `Offline` | 3 | — |

#### `enum Role`

| Variant | Value | Description |
|---|---|---|
| `Admin` | 0 | — |
| `Manufacturer` | 1 | — |
| `Operator` | 2 | — |
| `Viewer` | 3 | — |

#### `struct Manufacturer`

| Field | Type | Description |
|---|---|---|
| `manufacturer_id` | `BytesN<32>` | — |
| `address` | `Address` | — |
| `name` | `String` | — |
| `certification_hash` | `BytesN<32>` | — |
| `is_active` | `bool` | — |
| `registered_at` | `u64` | — |
| `device_count` | `u32` | — |

#### `struct Device`

| Field | Type | Description |
|---|---|---|
| `device_id` | `BytesN<32>` | — |
| `manufacturer_id` | `BytesN<32>` | — |
| `device_type` | `DeviceType` | — |
| `model` | `String` | — |
| `serial_number` | `String` | — |
| `firmware_version` | `u32` | — |
| `firmware_hash` | `BytesN<32>` | — |
| `status` | `DeviceStatus` | — |
| `operator` | `Address` | — |
| `location` | `String` | — |
| `registered_at` | `u64` | — |
| `last_heartbeat` | `u64` | — |
| `health_status` | `HealthStatus` | — |
| `uptime_start` | `u64` | — |
| `total_uptime_secs` | `u64` | — |
| `total_downtime_secs` | `u64` | — |
| `encryption_key_hash` | `BytesN<32>` | — |
| `metadata_ref` | `String` | — |

#### `struct DeviceRegistrationInfo`

| Field | Type | Description |
|---|---|---|
| `model` | `String` | — |
| `serial_number` | `String` | — |
| `location` | `String` | — |
| `encryption_key_hash` | `BytesN<32>` | — |
| `metadata_ref` | `String` | — |

#### `struct FirmwareAttestation`

| Field | Type | Description |
|---|---|---|
| `firmware_hash` | `BytesN<32>` | — |
| `firmware_signature` | `BytesN<64>` | — |

#### `struct FirmwareVersion`

| Field | Type | Description |
|---|---|---|
| `version` | `u32` | — |
| `manufacturer_id` | `BytesN<32>` | — |
| `device_type` | `DeviceType` | — |
| `binary_hash` | `BytesN<32>` | — |
| `release_notes_ref` | `String` | — |
| `status` | `FirmwareStatus` | — |
| `min_version` | `u32` | — |
| `published_at` | `u64` | — |
| `approved_by` | `Address` | — |
| `size_bytes` | `u64` | — |

#### `struct FirmwareUpdateRecord`

| Field | Type | Description |
|---|---|---|
| `update_id` | `u64` | — |
| `device_id` | `BytesN<32>` | — |
| `from_version` | `u32` | — |
| `to_version` | `u32` | — |
| `initiated_by` | `Address` | — |
| `initiated_at` | `u64` | — |
| `completed_at` | `u64` | — |
| `success` | `bool` | — |
| `error_ref` | `String` | — |

#### `struct Heartbeat`

| Field | Type | Description |
|---|---|---|
| `device_id` | `BytesN<32>` | — |
| `timestamp` | `u64` | — |
| `health_status` | `HealthStatus` | — |
| `battery_pct` | `u32` | — |
| `signal_strength` | `u32` | — |
| `error_count` | `u32` | — |
| `metrics_ref` | `String` | — |

#### `struct CommChannel`

| Field | Type | Description |
|---|---|---|
| `channel_id` | `BytesN<32>` | — |
| `device_id` | `BytesN<32>` | — |
| `encryption_key_hash` | `BytesN<32>` | — |
| `protocol` | `String` | — |
| `created_at` | `u64` | — |
| `last_rotated` | `u64` | — |
| `rotation_count` | `u32` | — |

#### `enum DataKey`

| Variant | Value | Description |
|---|---|---|
| `System` | — | — |
| `Initialized` | — | — |
| `Admin` | — | — |
| `Paused` | — | — |
| `RBAC` | — | — |
| `UserRole` | — | — |
| `Address` | — | — |
| `Manufacturers` | — | — |
| `Manufacturer` | — | — |
| `BytesN` | — | — |
| `32` | — | — |
| `ManufacturerByAddr` | — | — |
| `Address` | — | — |
| `ManufacturerCount` | — | — |
| `Devices` | — | — |
| `Device` | — | — |
| `BytesN` | — | — |
| `32` | — | — |
| `DevicesByOperator` | — | — |
| `Address` | — | — |
| `DevicesByManufacturer` | — | — |
| `BytesN` | — | — |
| `32` | — | — |
| `DevicesByType` | — | — |
| `u32` | — | — |
| `DeviceCount` | — | — |
| `ActiveDeviceCount` | — | — |
| `Firmware` | — | — |
| `Firmware` | — | — |
| `BytesN` | — | — |
| `32` | — | — |
| `u32` | — | — |
| `manufacturer_id` | — | — |
| `version` | — | — |
| `LatestFirmware` | — | — |
| `BytesN` | — | — |
| `32` | — | — |
| `u32` | — | — |
| `manufacturer_id` | — | — |
| `device_type` | — | — |
| `version` | — | — |
| `FirmwareUpdateRecord` | — | — |
| `u64` | — | — |
| `FirmwareUpdateCount` | — | — |
| `DeviceFirmwareUpdates` | — | — |
| `BytesN` | — | — |
| `32` | — | — |
| `device_id` | — | — |
| `Vec` | — | — |
| `u64` | — | — |
| `Health` | — | — |
| `DeviceHeartbeats` | — | — |
| `BytesN` | — | — |
| `32` | — | — |
| `device_id` | — | — |
| `Vec` | — | — |
| `Heartbeat` | — | — |
| `last` | — | — |
| `N` | — | — |
| `HeartbeatMinInterval` | — | — |
| `u64` | — | — |
| `seconds` | — | — |
| `Communication` | — | — |
| `CommChannel` | — | — |
| `BytesN` | — | — |
| `32` | — | — |
| `channel_id` | — | — |
| `CommChannel` | — | — |
| `DeviceChannel` | — | — |
| `BytesN` | — | — |
| `32` | — | — |
| `device_id` | — | — |
| `channel_id` | — | — |
| `KeyRotationMinInterval` | — | — |
| `u64` | — | — |
| `seconds` | — | — |
| `Issue` | — | — |
| `194` | — | — |
| `manufacturer` | — | — |
| `signed` | — | — |
| `firmware` | — | — |
| `verification` | — | — |
| `Address` | — | — |
| `of` | — | — |
| `the` | — | — |
| `deployed` | — | — |
| `crypto_registry` | — | — |
| `contract` | — | — |
| `used` | — | — |
| `to` | — | — |
| `look` | — | — |
| `up` | — | — |
| `the` | — | — |
| `manufacturer` | — | — |
| `signing` | — | — |
| `public` | — | — |
| `key` | — | — |
| `for` | — | — |
| `firmware` | — | — |
| `authentication` | — | — |
| `CryptoRegistryContract` | — | — |
| `The` | — | — |
| `on` | — | — |
| `chain` | — | — |
| `identity` | — | — |
| `crypto_registry` | — | — |
| `owner` | — | — |
| `whose` | — | — |
| `key` | — | — |
| `bundle` | — | — |
| `belongs` | — | — |
| `to` | — | — |
| `manufacturer` | — | — |
| `BytesN` | — | — |
| `32` | — | — |
| `Only` | — | — |
| `set` | — | — |
| `after` | — | — |
| `admin` | — | — |
| `explicitly` | — | — |
| `links` | — | — |
| `the` | — | — |
| `manufacturer` | — | — |
| `to` | — | — |
| `its` | — | — |
| `key` | — | — |
| `bundle` | — | — |
| `ManufacturerCryptoOwner` | — | — |
| `BytesN` | — | — |
| `32` | — | — |

### Error Codes

| Variant | Code | Description |
|---|---|---|
| `Unauthorized` | 100 | — |
| `NotAdmin` | 102 | — |
| `NotDeviceOperator` | 115 | — |
| `NotManufacturer` | 116 | — |
| `InputTooLong` | 201 | — |
| `InputTooShort` | 202 | — |
| `InvalidDeviceType` | 240 | — |
| `InvalidFirmwareHash` | 250 | — |
| `InvalidMetricValue` | 260 | — |
| `InvalidTimestamp` | 270 | — |
| `NotInitialized` | 300 | — |
| `AlreadyInitialized` | 301 | — |
| `ContractPaused` | 302 | — |
| `NotPaused` | 303 | — |
| `DeviceNotFound` | 405 | — |
| `DeviceAlreadyRegistered` | 420 | — |
| `ManufacturerNotRegistered` | 425 | — |
| `ManufacturerAlreadyRegistered` | 426 | — |
| `FirmwareVersionNotFound` | 430 | — |
| `FirmwareAlreadyExists` | 431 | — |
| `ChannelNotFound` | 440 | — |
| `InvalidEncryptionKey` | 602 | — |
| `KeyRotationTooFrequent` | 603 | — |
| `CryptoRegistryNotConfigured` | 604 | — |
| `ManufacturerCryptoOwnerNotSet` | 605 | — |
| `InvalidFirmwareSignature` | 606 | — |
| `CryptoKeyBundleNotFound` | 607 | — |
| `InvalidSigningKeyAlgorithm` | 608 | — |
| `DeviceDecommissioned` | 820 | — |
| `FirmwareNotApproved` | 821 | — |
| `HeartbeatTooFrequent` | 822 | — |
| `DeviceNotActive` | 823 | — |
| `DeviceSuspended` | 824 | — |
| `DowngradeNotAllowed` | 825 | — |
| `DeviceOffline` | 826 | — |

### Examples

#### `test_initialize`

```rust
let env = Env::default();
    let (client, admin) = setup(&env);
    client.initialize(&admin);
    let result = client.try_initialize(&admin);
    assert_eq!(result, Err(Ok(Error::AlreadyInitialized)));
```

#### `test_pause_unpause`

```rust
let env = Env::default();
    let (client, admin) = setup(&env);
    client.initialize(&admin);
    client.pause(&admin);
    let user = Address::generate(&env);
    let result = client.try_set_role(&admin, &user, &Role::Operator);
    assert_eq!(result, Err(Ok(Error::ContractPaused)));
    client.unpause(&admin);
    client.set_role(&admin, &user, &Role::Operator);
```

#### `test_pause_not_admin`

```rust
let env = Env::default();
    let (client, admin) = setup(&env);
    client.initialize(&admin);
    let non_admin = Address::generate(&env);
    let result = client.try_pause(&non_admin);
    assert_eq!(result, Err(Ok(Error::NotAdmin)));
```

---

## load_testing

### Functions

| Function | Parameters | Returns | Description |
|---|---|---|---|
| `run` | `env: Env, config: LoadTestConfig` | `LoadTestResult` | Execute a load-test run and persist the result.  Each "operation" is a lightweight storage read/write that exercises the contract's execution path.  Latency is measured in ledger sequence units. |
| `last_result` | `env: Env` | `Option<LoadTestResult>` | Return the result of the most recent run. |
| `run_count` | `env: Env` | `u32` | Return the total number of runs executed. |

### Types

#### `struct LoadTestConfig`

| Field | Type | Description |
|---|---|---|
| `num_requests` | `u32` | — |
| `concurrency` | `u32` | — |
| `max_avg_latency` | `u64` | — |
| `min_success_rate` | `u32` | — |

#### `struct LoadTestResult`

| Field | Type | Description |
|---|---|---|
| `total_requests` | `u32` | — |
| `successful` | `u32` | — |
| `failed` | `u32` | — |
| `success_rate` | `u32` | — |
| `min_latency` | `u64` | — |
| `max_latency` | `u64` | — |
| `avg_latency` | `u64` | — |
| `p95_latency` | `u64` | — |
| `p99_latency` | `u64` | — |
| `passed` | `bool` | — |

#### `enum DataKey`

| Variant | Value | Description |
|---|---|---|
| `LastResult` | — | — |
| `RunCount` | — | — |

---

## medical_consent_nft

### Functions

| Function | Parameters | Returns | Description |
|---|---|---|---|
| `initialize` | `env: Env, admin: Address` | `Result<(), ContractError>` | Initialize the contract with an admin |
| `add_issuer` | `env: Env, issuer: Address` | `()` | Add an authorized issuer (clinic/healthcare provider) |
| `remove_issuer` | `env: Env, issuer: Address` | `()` | Remove an authorized issuer |
| `is_issuer` | `env: Env, address: Address` | `bool` | Check if address is an authorized issuer |
| `revoke_consent` | `env: Env, token_id: u64` | `Result<(), ContractError>` | Revoke consent (marks as revoked, prevents transfers) - Patient authorizes via require_auth on their address from metadata |
| `owner_of` | `env: Env, token_id: u64` | `Address` | Get token owner |
| `get_metadata` | `env: Env, token_id: u64` | `ConsentMetadata` | Get consent metadata |
| `is_revoked` | `env: Env, token_id: u64` | `bool` | Check if consent is revoked |
| `get_history` | `env: Env, token_id: u64` | `Vec<ConsentHistoryEntry>` | Get consent history (audit trail) |
| `tokens_of_owner` | `env: Env, owner: Address` | `Vec<u64>` | Get all tokens owned by an address |
| `has_consent` | `env: Env, patient: Address, doctor: Address, consent_type: String` | `bool` | Check if doctor has valid consent for patient and type (for cross-contract access control) |
| `is_valid` | `env: Env, token_id: u64` | `bool` | Check if consent is valid (not revoked and not expired) |
| `get_delegations` | `env: Env, token_id: u64` | `Vec<Delegation>` | Get active delegations for a token |
| `add_emergency_authority` | `env: Env, authority: Address` | `Result<(), ContractError>` | Add emergency authority |
| `set_marketplace_enabled` | `env: Env, enabled: bool` | `Result<(), ContractError>` | Enable/disable marketplace |
| `get_version_history` | `env: Env, token_id: u64` | `Vec<VersionHistoryEntry>` | Get version history |
| `enable_dynamic_updates` | `env: Env, token_id: u64` | `Result<(), ContractError>` | Enable dynamic updates for a consent |
| `get_analytics` | `env: Env` | `AnalyticsData` | Get analytics data |
| `generate_consent_report` | `env: Env, patient: Address` | `Vec<u64>` | Generate consent report for a patient |

### Types

#### `enum DataKey`

| Variant | Value | Description |
|---|---|---|
| `Admin` | — | — |
| `Issuers` | — | — |
| `TokenCounter` | — | — |
| `TokenOwner` | — | — |
| `u64` | — | — |
| `TokenMetadata` | — | — |
| `u64` | — | — |
| `TokenRevoked` | — | — |
| `u64` | — | — |
| `OwnerTokens` | — | — |
| `Address` | — | — |
| `ConsentHistory` | — | — |
| `u64` | — | — |
| `PatientConsents` | — | — |
| `Address` | — | — |
| `Track` | — | — |
| `tokens` | — | — |
| `issued` | — | — |
| `for` | — | — |
| `a` | — | — |
| `patient` | — | — |
| `for` | — | — |
| `revoke` | — | — |
| `access` | — | — |
| `Advanced` | — | — |
| `features` | — | — |
| `storage` | — | — |
| `keys` | — | — |
| `GranularPermissions` | — | — |
| `u64` | — | — |
| `Granular` | — | — |
| `permissions` | — | — |
| `per` | — | — |
| `token` | — | — |
| `AccessControls` | — | — |
| `u64` | — | — |
| `Time` | — | — |
| `based` | — | — |
| `and` | — | — |
| `condition` | — | — |
| `based` | — | — |
| `access` | — | — |
| `controls` | — | — |
| `ConsentDelegations` | — | — |
| `u64` | — | — |
| `Delegation` | — | — |
| `mappings` | — | — |
| `ConsentInheritance` | — | — |
| `u64` | — | — |
| `Parent` | — | — |
| `child` | — | — |
| `consent` | — | — |
| `relationships` | — | — |
| `EmergencyOverrides` | — | — |
| `u64` | — | — |
| `Emergency` | — | — |
| `override` | — | — |
| `records` | — | — |
| `MarketplaceListings` | — | — |
| `u64` | — | — |
| `Research` | — | — |
| `marketplace` | — | — |
| `listings` | — | — |
| `VersionHistory` | — | — |
| `u64` | — | — |
| `Full` | — | — |
| `version` | — | — |
| `history` | — | — |
| `for` | — | — |
| `dynamic` | — | — |
| `updates` | — | — |
| `AnalyticsData` | — | — |
| `Aggregated` | — | — |
| `analytics` | — | — |
| `data` | — | — |
| `EmergencyAuthorities` | — | — |
| `Authorized` | — | — |
| `emergency` | — | — |
| `override` | — | — |
| `addresses` | — | — |
| `MarketplaceEnabled` | — | — |
| `Marketplace` | — | — |
| `feature` | — | — |
| `flag` | — | — |

#### `enum ContractError`

| Variant | Value | Description |
|---|---|---|
| `NotAuthorized` | 1 | — |
| `TokenNotFound` | 2 | — |
| `ConsentRevoked` | 3 | — |
| `AlreadyInitialized` | 4 | — |
| `NotTokenOwner` | 5 | — |
| `InvalidPermission` | 6 | — |
| `AccessDenied` | 7 | — |
| `InvalidDelegation` | 8 | — |
| `EmergencyOverrideFailed` | 9 | — |
| `MarketplaceNotEnabled` | 10 | — |
| `InvalidCondition` | 11 | — |
| `InheritanceCycle` | 12 | — |

#### `enum DataType`

| Variant | Value | Description |
|---|---|---|
| `Demographics` | — | — |
| `MedicalHistory` | — | — |
| `LabResults` | — | — |
| `Imaging` | — | — |
| `Medications` | — | — |
| `Procedures` | — | — |
| `Allergies` | — | — |
| `Research` | — | — |
| `Financial` | — | — |

#### `enum PermissionLevel`

| Variant | Value | Description |
|---|---|---|
| `None` | — | — |
| `No` | — | — |
| `access` | — | — |
| `Read` | — | — |
| `Read` | — | — |
| `only` | — | — |
| `access` | — | — |
| `Write` | — | — |
| `Read` | — | — |
| `and` | — | — |
| `write` | — | — |
| `access` | — | — |
| `Full` | — | — |
| `Full` | — | — |
| `access` | — | — |
| `including` | — | — |
| `deletion` | — | — |

#### `struct GranularPermissions`

| Field | Type | Description |
|---|---|---|
| `permissions` | `Map<DataType` | — |

#### `enum AccessCondition`

| Variant | Value | Description |
|---|---|---|
| `TimeWindow` | — | — |
| `u64` | — | — |
| `u64` | — | — |
| `start` | — | — |
| `end` | — | — |
| `Time` | — | — |
| `based` | — | — |
| `access` | — | — |
| `window` | — | — |
| `DayOfWeek` | — | — |
| `Vec` | — | — |
| `u32` | — | — |
| `Specific` | — | — |
| `days` | — | — |
| `of` | — | — |
| `week` | — | — |
| `0` | — | — |
| `6` | — | — |
| `TimeOfDay` | — | — |
| `u32` | — | — |
| `u32` | — | — |
| `start_hour` | — | — |
| `end_hour` | — | — |
| `Time` | — | — |
| `of` | — | — |
| `day` | — | — |
| `restrictions` | — | — |
| `LocationBased` | — | — |
| `Vec` | — | — |
| `String` | — | — |
| `Location` | — | — |
| `based` | — | — |
| `access` | — | — |
| `PurposeBased` | — | — |
| `Vec` | — | — |
| `String` | — | — |
| `Purpose` | — | — |
| `based` | — | — |
| `restrictions` | — | — |
| `EmergencyOnly` | — | — |
| `Emergency` | — | — |
| `access` | — | — |
| `only` | — | — |

#### `struct AccessControl`

| Field | Type | Description |
|---|---|---|
| `conditions` | `Vec<AccessCondition>` | — |
| `max_access_count` | `u32` | — |
| `current_access_count` | `u32` | — |
| `last_access_timestamp` | `u64` | — |

#### `struct Delegation`

| Field | Type | Description |
|---|---|---|
| `delegate` | `Address` | — |
| `permissions` | `GranularPermissions` | — |
| `expiry_timestamp` | `u64` | — |
| `created_timestamp` | `u64` | — |

#### `struct Inheritance`

| Field | Type | Description |
|---|---|---|
| `parent_token_id` | `u64` | — |
| `inherited_permissions` | `GranularPermissions` | — |

#### `struct EmergencyOverride`

| Field | Type | Description |
|---|---|---|
| `override_id` | `u64` | — |
| `authorized_by` | `Address` | — |
| `reason` | `String` | — |
| `timestamp` | `u64` | — |
| `duration` | `u64` | — |
| `used` | `bool` | — |

#### `struct MarketplaceListing`

| Field | Type | Description |
|---|---|---|
| `token_id` | `u64` | — |
| `price` | `i128` | — |
| `data_types` | `Vec<DataType>` | — |
| `research_purpose` | `String` | — |
| `duration` | `u64` | — |
| `listed_by` | `Address` | — |
| `listed_timestamp` | `u64` | — |
| `active` | `bool` | — |

#### `struct VersionHistoryEntry`

| Field | Type | Description |
|---|---|---|
| `version` | `u32` | — |
| `metadata_uri` | `String` | — |
| `updated_by` | `Address` | — |
| `timestamp` | `u64` | — |
| `change_summary` | `String` | — |

#### `struct ConsentMetadata`

| Field | Type | Description |
|---|---|---|
| `metadata_uri` | `String` | — |
| `consent_type` | `String` | — |
| `issued_timestamp` | `u64` | — |
| `expiry_timestamp` | `u64` | — |
| `issuer` | `Address` | — |
| `patient` | `Address` | — |
| `version` | `u32` | — |
| `dynamic_updates_enabled` | `bool` | — |

#### `struct ConsentHistoryEntry`

| Field | Type | Description |
|---|---|---|
| `action` | `String` | — |
| `timestamp` | `u64` | — |
| `actor` | `Address` | — |
| `metadata_uri` | `String` | — |
| `details` | `String` | — |

#### `struct AnalyticsData`

| Field | Type | Description |
|---|---|---|
| `total_consents` | `u64` | — |
| `active_consents` | `u64` | — |
| `revoked_consents` | `u64` | — |
| `total_delegations` | `u64` | — |
| `total_emergency_overrides` | `u64` | — |
| `marketplace_listings` | `u64` | — |
| `total_access_count` | `u64` | — |

### Examples

#### `test_initialize_and_add_issuer`

```rust
let env = Env::default();
        let contract_id = env.register_contract(None, PatientConsentToken);
        let client = PatientConsentTokenClient::new(&env, &contract_id);

        let admin = Address::generate(&env);
        let issuer = Address::generate(&env);

        client.initialize(&admin);
        client.add_issuer(&issuer);
```

---

## medical_record_backup

### Functions

| Function | Parameters | Returns | Description |
|---|---|---|---|
| `initialize` | `env: Env, admin: Address` | `Result<(), Error>` | — |
| `set_paused` | `env: Env, caller: Address, paused: bool` | `Result<bool, Error>` | — |
| `set_policy` | `env: Env, caller: Address, policy: BackupPolicy` | `Result<bool, Error>` | — |
| `get_policy` | `env: Env` | `Result<BackupPolicy, Error>` | — |
| `get_target` | `env: Env, target_id: u32` | `Option<BackupTarget>` | — |
| `list_targets` | `env: Env` | `Vec<BackupTarget>` | — |
| `approve_restore` | `env: Env, caller: Address, request_id: u64` | `Result<bool, Error>` | — |
| `execute_restore` | `env: Env, caller: Address, request_id: u64` | `Result<String, Error>` | — |
| `optimize_and_cleanup` | `env: Env, caller: Address` | `Result<CleanupReport, Error>` | — |
| `resolve_alert` | `env: Env, caller: Address, alert_id: u64` | `Result<bool, Error>` | — |
| `list_alerts` | `env: Env, open_only: bool` | `Vec<AlertEntry>` | — |
| `list_artifacts` | `env: Env, include_archived: bool` | `Vec<BackupArtifact>` | — |
| `get_artifact` | `env: Env, artifact_id: u64` | `Option<BackupArtifact>` | — |
| `get_execution` | `env: Env, execution_id: u64` | `Option<BackupExecution>` | — |
| `get_restore_request` | `env: Env, request_id: u64` | `Option<RestoreRequest>` | — |
| `get_recovery_test` | `env: Env, test_id: u64` | `Option<RecoveryTest>` | — |
| `get_health` | `env: Env` | `BackupHealth` | — |
| `get_schedule` | `env: Env` | `(u64, u64)` | — |

### Types

#### `enum BackupNetwork`

| Variant | Value | Description |
|---|---|---|
| `Stellar` | — | — |
| `Ethereum` | — | — |
| `Polygon` | — | — |
| `Arbitrum` | — | — |
| `Optimism` | — | — |
| `Avalanche` | — | — |
| `BinanceSmartChain` | — | — |
| `Ipfs` | — | — |
| `Filecoin` | — | — |
| `Arweave` | — | — |
| `AwsS3` | — | — |
| `AzureBlob` | — | — |
| `GcpStorage` | — | — |
| `Custom` | — | — |
| `u32` | — | — |

#### `enum GeoRegion`

| Variant | Value | Description |
|---|---|---|
| `UsEast` | — | — |
| `UsWest` | — | — |
| `EuCentral` | — | — |
| `EuWest` | — | — |
| `ApSouth` | — | — |
| `ApNorth` | — | — |
| `SaEast` | — | — |
| `AfSouth` | — | — |
| `Custom` | — | — |
| `u32` | — | — |

#### `enum BackupStatus`

| Variant | Value | Description |
|---|---|---|
| `Completed` | — | — |
| `Verified` | — | — |
| `Failed` | — | — |
| `Archived` | — | — |
| `Restored` | — | — |

#### `enum ReplicaStatus`

| Variant | Value | Description |
|---|---|---|
| `Synced` | — | — |
| `Verified` | — | — |
| `Failed` | — | — |
| `Archived` | — | — |

#### `enum AlertSeverity`

| Variant | Value | Description |
|---|---|---|
| `Low` | — | — |
| `Medium` | — | — |
| `High` | — | — |
| `Critical` | — | — |

#### `enum AlertKind`

| Variant | Value | Description |
|---|---|---|
| `BackupFailure` | — | — |
| `TargetFailure` | — | — |
| `GeoRedundancyRisk` | — | — |
| `IntegrityCheckFailed` | — | — |
| `RestoreFailure` | — | — |
| `CostThresholdExceeded` | — | — |
| `ScheduleMissed` | — | — |
| `RecoveryDrillFailed` | — | — |

#### `enum RestoreStatus`

| Variant | Value | Description |
|---|---|---|
| `Pending` | — | — |
| `Approved` | — | — |
| `Executed` | — | — |
| `Rejected` | — | — |

#### `struct BackupPolicy`

| Field | Type | Description |
|---|---|---|
| `interval_seconds` | `u64` | — |
| `retention_seconds` | `u64` | — |
| `max_active_backups` | `u32` | — |
| `min_targets_per_backup` | `u32` | — |
| `min_region_count` | `u32` | — |
| `max_total_cost_weight` | `u32` | — |
| `verify_on_write` | `bool` | — |
| `encryption_required` | `bool` | — |
| `auto_cleanup` | `bool` | — |
| `min_restore_approvals` | `u32` | — |

#### `struct BackupTarget`

| Field | Type | Description |
|---|---|---|
| `target_id` | `u32` | — |
| `network` | `BackupNetwork` | — |
| `region` | `GeoRegion` | — |
| `endpoint_hash` | `BytesN<32>` | — |
| `is_active` | `bool` | — |
| `encrypted_only` | `bool` | — |
| `cost_weight` | `u32` | — |
| `max_capacity_units` | `u64` | — |
| `failure_count` | `u32` | — |

#### `struct BackupArtifact`

| Field | Type | Description |
|---|---|---|
| `artifact_id` | `u64` | — |
| `source_root` | `BytesN<32>` | — |
| `checksum` | `BytesN<32>` | — |
| `snapshot_ref` | `String` | — |
| `encryption_key_version` | `u32` | — |
| `encrypted` | `bool` | — |
| `created_at` | `u64` | — |
| `expires_at` | `u64` | — |
| `target_ids` | `Vec<u32>` | — |
| `region_count` | `u32` | — |
| `total_cost_weight` | `u32` | — |
| `status` | `BackupStatus` | — |
| `last_verified_at` | `u64` | — |
| `last_restored_at` | `u64` | — |
| `restore_drill_passed` | `bool` | — |

#### `struct BackupReplica`

| Field | Type | Description |
|---|---|---|
| `artifact_id` | `u64` | — |
| `target_id` | `u32` | — |
| `checksum` | `BytesN<32>` | — |
| `synced_at` | `u64` | — |
| `status` | `ReplicaStatus` | — |

#### `struct BackupExecution`

| Field | Type | Description |
|---|---|---|
| `execution_id` | `u64` | — |
| `triggered_by` | `Address` | — |
| `started_at` | `u64` | — |
| `completed_at` | `u64` | — |
| `scheduled` | `bool` | — |
| `success_targets` | `u32` | — |
| `failed_targets` | `u32` | — |
| `artifact_id` | `Option<u64>` | — |
| `error_code` | `Option<u32>` | — |

#### `struct AlertEntry`

| Field | Type | Description |
|---|---|---|
| `alert_id` | `u64` | — |
| `kind` | `AlertKind` | — |
| `severity` | `AlertSeverity` | — |
| `created_at` | `u64` | — |
| `details_hash` | `BytesN<32>` | — |
| `resolved` | `bool` | — |
| `resolved_at` | `u64` | — |

#### `struct RecoveryTest`

| Field | Type | Description |
|---|---|---|
| `test_id` | `u64` | — |
| `artifact_id` | `u64` | — |
| `started_by` | `Address` | — |
| `executed_at` | `u64` | — |
| `validation_hash` | `BytesN<32>` | — |
| `passed` | `bool` | — |

#### `struct RestoreRequest`

| Field | Type | Description |
|---|---|---|
| `request_id` | `u64` | — |
| `artifact_id` | `u64` | — |
| `requested_by` | `Address` | — |
| `reason_hash` | `BytesN<32>` | — |
| `requested_at` | `u64` | — |
| `approvals` | `Vec<Address>` | — |
| `status` | `RestoreStatus` | — |
| `executed_at` | `u64` | — |

#### `struct BackupHealth`

| Field | Type | Description |
|---|---|---|
| `total_runs` | `u64` | — |
| `successful_runs` | `u64` | — |
| `failed_runs` | `u64` | — |
| `consecutive_failures` | `u32` | — |
| `last_success_at` | `u64` | — |
| `last_failure_at` | `u64` | — |
| `last_error_code` | `u32` | — |

#### `struct CleanupReport`

| Field | Type | Description |
|---|---|---|
| `archived_backups` | `u32` | — |
| `reclaimed_cost_weight` | `u32` | — |
| `remaining_active_backups` | `u32` | — |

#### `enum DataKey`

| Variant | Value | Description |
|---|---|---|
| `Policy` | — | — |
| `Roles` | — | — |
| `Address` | — | — |
| `Target` | — | — |
| `u32` | — | — |
| `TargetIds` | — | — |
| `Artifact` | — | — |
| `u64` | — | — |
| `ArtifactIds` | — | — |
| `Replica` | — | — |
| `u64` | — | — |
| `u32` | — | — |
| `Execution` | — | — |
| `u64` | — | — |
| `Alert` | — | — |
| `u64` | — | — |
| `AlertIds` | — | — |
| `RecoveryTest` | — | — |
| `u64` | — | — |
| `RestoreRequest` | — | — |
| `u64` | — | — |
| `Health` | — | — |

#### `enum Error`

| Variant | Value | Description |
|---|---|---|
| `AlreadyInitialized` | 1 | — |
| `NotInitialized` | 2 | — |
| `NotAuthorized` | 3 | — |
| `ContractPaused` | 4 | — |
| `InvalidInput` | 5 | — |
| `TargetNotFound` | 6 | — |
| `BackupNotFound` | 7 | — |
| `RestoreRequestNotFound` | 8 | — |
| `RecoveryTestNotFound` | 9 | — |
| `ScheduleNotDue` | 10 | — |
| `InsufficientTargets` | 11 | — |
| `GeoRedundancyNotMet` | 12 | — |
| `EncryptionRequired` | 13 | — |
| `IntegrityMismatch` | 14 | — |
| `RestoreNotApproved` | 15 | — |
| `AlreadyExecuted` | 16 | — |
| `DuplicateApproval` | 17 | — |
| `CostLimitExceeded` | 18 | — |

### Examples

#### `backup_run_creates_geo_redundant_artifact`

```rust
let env = Env::default();
    env.mock_all_auths();
    let (client, admin) = setup(&env);
    register_two_targets(&client, &admin, &env);

    let id = client.run_backup_now(
        &admin,
        &sample_hash(&env, 7),
        &String::from_str(&env, "ipfs://snapshot-a"),
```

#### `scheduled_backup_respects_interval`

```rust
let env = Env::default();
    env.mock_all_auths();
    let (client, admin) = setup(&env);
    register_two_targets(&client, &admin, &env);

    let policy = BackupPolicy {
        interval_seconds: 1_000,
        retention_seconds: 10_000,
        max_active_backups: 10,
```

#### `integrity_mismatch_creates_alert`

```rust
let env = Env::default();
    env.mock_all_auths();
    let (client, admin) = setup(&env);
    register_two_targets(&client, &admin, &env);

    let id = client.run_backup_now(
        &admin,
        &sample_hash(&env, 3),
        &String::from_str(&env, "ipfs://snapshot-d"),
```

---

## medical_record_search

### Functions

| Function | Parameters | Returns | Description |
|---|---|---|---|
| `initialize` | `env: Env, admin: Address` | `Result<(), Error>` | — |
| `set_paused` | `env: Env, caller: Address, paused: bool` | `Result<bool, Error>` | — |
| `set_cache_policy` | `env: Env, caller: Address, policy: CachePolicy` | `Result<bool, Error>` | — |
| `set_ranking` | `env: Env, caller: Address, cfg: RankingConfig` | `Result<bool, Error>` | — |
| `index_record` | `env: Env, caller: Address, input: IndexInput` | `Result<bool, Error>` | — |
| `get_cache_entry` | `env: Env, query_hash: BytesN<32>` | `Result<QueryCacheEntry, Error>` | — |
| `invalidate_cache` | `env: Env, caller: Address` | `Result<bool, Error>` | — |
| `get_audit` | `env: Env, caller: Address, query_id: u64` | `Result<SearchAuditEntry, Error>` | — |
| `preview_query_hash` | `env: Env, query: SearchQuery` | `BytesN<32>` | — |
| `get_indexed_entry` | `env: Env, record_id: u64` | `Result<SearchIndexEntry, Error>` | — |

### Types

#### `enum ChainId`

| Variant | Value | Description |
|---|---|---|
| `Stellar` | — | — |
| `Ethereum` | — | — |
| `Polygon` | — | — |
| `Avalanche` | — | — |
| `Arbitrum` | — | — |
| `Optimism` | — | — |
| `Custom` | — | — |
| `u32` | — | — |

#### `struct IndexInput`

| Field | Type | Description |
|---|---|---|
| `record_id` | `u64` | — |
| `patient` | `Address` | — |
| `network` | `ChainId` | — |
| `created_at` | `u64` | — |
| `is_confidential` | `bool` | — |
| `category_hash` | `BytesN<32>` | — |
| `token_hashes` | `Vec<BytesN<32>>` | — |
| `attribute_hashes` | `Vec<BytesN<32>>` | — |
| `encrypted_ref_hash` | `BytesN<32>` | — |
| `quality_score_bps` | `u32` | — |

#### `struct SearchIndexEntry`

| Field | Type | Description |
|---|---|---|
| `record_id` | `u64` | — |
| `indexed_by` | `Address` | — |
| `patient` | `Address` | — |
| `network` | `ChainId` | — |
| `created_at` | `u64` | — |
| `is_confidential` | `bool` | — |
| `category_hash` | `BytesN<32>` | — |
| `token_hashes` | `Vec<BytesN<32>>` | — |
| `attribute_hashes` | `Vec<BytesN<32>>` | — |
| `encrypted_ref_hash` | `BytesN<32>` | — |
| `quality_score_bps` | `u32` | — |

#### `struct SearchQuery`

| Field | Type | Description |
|---|---|---|
| `required_tokens` | `Vec<BytesN<32>>` | — |
| `optional_tokens` | `Vec<BytesN<32>>` | — |
| `category_filters` | `Vec<BytesN<32>>` | — |
| `attribute_filters` | `Vec<BytesN<32>>` | — |
| `network_filters` | `Vec<ChainId>` | — |
| `patient_filter` | `Option<Address>` | — |
| `from_timestamp` | `u64` | — |
| `to_timestamp` | `u64` | — |
| `include_confidential` | `bool` | — |
| `min_quality_bps` | `u32` | — |

#### `struct SearchResult`

| Field | Type | Description |
|---|---|---|
| `record_id` | `u64` | — |
| `patient` | `Address` | — |
| `network` | `ChainId` | — |
| `created_at` | `u64` | — |
| `encrypted_ref_hash` | `BytesN<32>` | — |
| `is_confidential` | `bool` | — |
| `score_bps` | `u32` | — |

#### `struct QueryCacheEntry`

| Field | Type | Description |
|---|---|---|
| `query_hash` | `BytesN<32>` | — |
| `created_at` | `u64` | — |
| `expires_at` | `u64` | — |
| `hit_count` | `u32` | — |
| `results` | `Vec<SearchResult>` | — |

#### `struct CachePolicy`

| Field | Type | Description |
|---|---|---|
| `ttl_seconds` | `u64` | — |
| `max_entries` | `u32` | — |

#### `struct RankingConfig`

| Field | Type | Description |
|---|---|---|
| `required_weight_bps` | `u32` | — |
| `optional_weight_bps` | `u32` | — |
| `recency_weight_bps` | `u32` | — |
| `quality_weight_bps` | `u32` | — |

#### `struct SearchAuditEntry`

| Field | Type | Description |
|---|---|---|
| `query_id` | `u64` | — |
| `caller` | `Address` | — |
| `query_hash` | `BytesN<32>` | — |
| `timestamp` | `u64` | — |
| `result_count` | `u32` | — |
| `from_cache` | `bool` | — |
| `granted` | `bool` | — |

#### `enum DataKey`

| Variant | Value | Description |
|---|---|---|
| `Roles` | — | — |
| `Address` | — | — |
| `Index` | — | — |
| `u64` | — | — |
| `IndexedIds` | — | — |
| `TokenPosting` | — | — |
| `BytesN` | — | — |
| `32` | — | — |
| `CategoryPosting` | — | — |
| `BytesN` | — | — |
| `32` | — | — |
| `AttributePosting` | — | — |
| `BytesN` | — | — |
| `32` | — | — |
| `Cache` | — | — |
| `BytesN` | — | — |
| `32` | — | — |
| `CacheOrder` | — | — |
| `CachePolicy` | — | — |
| `Ranking` | — | — |
| `Audit` | — | — |
| `u64` | — | — |

#### `enum Error`

| Variant | Value | Description |
|---|---|---|
| `AlreadyInitialized` | 1 | — |
| `NotInitialized` | 2 | — |
| `NotAuthorized` | 3 | — |
| `ContractPaused` | 4 | — |
| `InvalidInput` | 5 | — |
| `RecordNotIndexed` | 6 | — |
| `QueryTooLarge` | 7 | — |
| `CacheMiss` | 8 | — |

### Examples

#### `search_ranking_and_cache_work`

```rust
let env = Env::default();
    env.mock_all_auths();
    let (client, admin) = setup(&env);

    let indexer = Address::generate(&env);
    let searcher = Address::generate(&env);
    client.assign_role(
        &admin,
        &indexer,
```

#### `search_requires_role`

```rust
let env = Env::default();
    env.mock_all_auths();
    let (client, admin) = setup(&env);
    let indexer = Address::generate(&env);
    let unauthorized = Address::generate(&env);
    client.assign_role(&admin, &indexer, &ROLE_INDEXER);

    index_entry(
        &env,
```

#### `confidential_records_require_confidential_role`

```rust
let env = Env::default();
    env.mock_all_auths();
    let (client, admin) = setup(&env);
    let indexer = Address::generate(&env);
    let basic_searcher = Address::generate(&env);
    let privileged_searcher = Address::generate(&env);
    client.assign_role(&admin, &indexer, &(ROLE_INDEXER | ROLE_SEARCHER));
    client.assign_role(&admin, &basic_searcher, &ROLE_SEARCHER);
    client.assign_role(
```

---

## medical_records

### Functions

| Function | Parameters | Returns | Description |
|---|---|---|---|
| `initialize` | `env: Env, admin: Address` | `Result<(), RecordError>` | — |
| `get_admin` | `env: Env` | `Option<Address>` | — |
| `get_record` | `env: Env, record_id: u64, caller: Address` | `Result<Record, RecordError>` | — |

### Types

#### `struct Record`

| Field | Type | Description |
|---|---|---|
| `id` | `u64` | — |
| `patient_id` | `String` | — |
| `record_type` | `String` | — |
| `content` | `String` | — |
| `timestamp` | `u64` | — |
| `owner` | `Address` | — |

#### `enum RecordError`

| Variant | Value | Description |
|---|---|---|
| `InvalidInput` | 1 | — |
| `Unauthorized` | 2 | — |
| `RecordNotFound` | 3 | — |
| `EncryptionFailed` | 4 | — |
| `NotInitialized` | 5 | — |
| `AlreadyInitialized` | 6 | — |

#### `enum DataKey`

| Variant | Value | Description |
|---|---|---|
| `Record` | — | — |
| `u64` | — | — |

---

## mpc_manager

### Functions

| Function | Parameters | Returns | Description |
|---|---|---|---|
| `initialize` | `env: Env, admin: Address` | `Result<(), Error>` | — |
| `get_session` | `env: Env, session_id: BytesN<32>` | `Result<Option<MPCSession>, Error>` | — |
| `get_audit_trail` | `env: Env, session_id: BytesN<32>` | `Result<Vec<AuditEntry>, Error>` | Get audit trail for a session |
| `get_gas_stats` | `env: Env, session_id: BytesN<32>` | `Result<u64, Error>` | Get gas usage statistics for a session |

### Types

#### `struct SecretShare`

| Field | Type | Description |
|---|---|---|
| `share_id` | `u32` | — |
| `share_value` | `Bytes` | — |
| `commitment` | `BytesN<32>` | — |
| `created_at` | `u64` | — |

#### `struct ComputationProof`

| Field | Type | Description |
|---|---|---|
| `computation_type` | `String` | — |
| `input_commitment` | `BytesN<32>` | — |
| `output_hash` | `BytesN<32>` | — |
| `proof_data` | `Bytes` | — |
| `verification_key_hash` | `BytesN<32>` | — |
| `gas_used` | `u64` | — |
| `created_at` | `u64` | — |

#### `enum ComputationType`

| Variant | Value | Description |
|---|---|---|
| `StatisticalAnalysis` | — | — |
| `SecureAggregation` | — | — |
| `PrivacyPreservingML` | — | — |
| `DiagnosticAnalysis` | — | — |
| `DrugDiscovery` | — | — |

#### `struct AuditEntry`

| Field | Type | Description |
|---|---|---|
| `participant` | `Address` | — |
| `operation` | `String` | — |
| `session_id` | `BytesN<32>` | — |
| `timestamp` | `u64` | — |
| `gas_used` | `u64` | — |
| `metadata` | `Bytes` | — |

#### `enum SessionStatus`

| Variant | Value | Description |
|---|---|---|
| `Initiated` | — | — |
| `CommitPhase` | — | — |
| `RevealPhase` | — | — |
| `Finalized` | — | — |
| `Aborted` | — | — |
| `Expired` | — | — |

#### `struct ShareReveal`

| Field | Type | Description |
|---|---|---|
| `share_ref` | `String` | — |
| `share_hash` | `BytesN<32>` | — |
| `revealed_at` | `u64` | — |

#### `struct MPCSession`

| Field | Type | Description |
|---|---|---|
| `session_id` | `BytesN<32>` | — |
| `initiator` | `Address` | — |
| `participants` | `Vec<Address>` | — |
| `threshold` | `u32` | — |
| `purpose` | `String` | — |
| `created_at` | `u64` | — |
| `expires_at` | `u64` | — |
| `status` | `SessionStatus` | — |
| `commits` | `u32` | — |
| `reveals` | `u32` | — |
| `result_ref` | `String` | — |
| `result_hash` | `BytesN<32>` | — |
| `proof_ref` | `String` | — |
| `proof_hash` | `BytesN<32>` | — |
| `computation_type` | `ComputationType` | — |
| `total_gas_used` | `u64` | — |
| `audit_entries` | `u32` | — |

#### `enum DataKey`

| Variant | Value | Description |
|---|---|---|
| `Initialized` | — | — |
| `Admin` | — | — |
| `Session` | — | — |
| `BytesN` | — | — |
| `32` | — | — |
| `Commit` | — | — |
| `BytesN` | — | — |
| `32` | — | — |
| `Address` | — | — |
| `Reveal` | — | — |
| `BytesN` | — | — |
| `32` | — | — |
| `Address` | — | — |
| `SecretShare` | — | — |
| `BytesN` | — | — |
| `32` | — | — |
| `Address` | — | — |
| `u32` | — | — |
| `ComputationProof` | — | — |
| `BytesN` | — | — |
| `32` | — | — |
| `AuditEntry` | — | — |
| `u32` | — | — |
| `AuditCounter` | — | — |
| `GasTracker` | — | — |
| `BytesN` | — | — |
| `32` | — | — |
| `Address` | — | — |

#### `enum Error`

| Variant | Value | Description |
|---|---|---|
| `AlreadyInitialized` | 1 | — |
| `NotInitialized` | 2 | — |
| `NotAuthorized` | 3 | — |
| `InvalidInput` | 4 | — |
| `SessionNotFound` | 5 | — |
| `SessionExpired` | 6 | — |
| `InvalidState` | 7 | — |
| `DuplicateCommit` | 8 | — |
| `DuplicateReveal` | 9 | — |
| `ThresholdNotMet` | 10 | — |
| `InvalidShare` | 11 | — |
| `ComputationFailed` | 12 | — |
| `ProofVerificationFailed` | 13 | — |
| `GasLimitExceeded` | 14 | — |
| `InsufficientParticipants` | 15 | — |

### Examples

#### `mpc_session_lifecycle`

```rust
let env = Env::default();
    env.mock_all_auths();

    let (client, _id) = setup(&env);
    let admin = Address::generate(&env);
    client.initialize(&admin);

    let initiator = Address::generate(&env);
    let p1 = Address::generate(&env);
```

#### `test_shamir_secret_sharing`

```rust
let env = Env::default();
    env.mock_all_auths();

    let (client, _id) = setup(&env);
    let admin = Address::generate(&env);
    client.initialize(&admin);

    let initiator = Address::generate(&env);
    let p1 = Address::generate(&env);
```

#### `test_statistical_analysis`

```rust
let env = Env::default();
    env.mock_all_auths();

    let (client, _id) = setup(&env);
    let admin = Address::generate(&env);
    client.initialize(&admin);

    let initiator = Address::generate(&env);
    let p1 = Address::generate(&env);
```

---

## notification_system

### Functions

| Function | Parameters | Returns | Description |
|---|---|---|---|
| `initialize` | `env: Env, admin: Address` | `Result<(), Error>` | Initialise the contract. Must be called exactly once. |
| `get_admin` | `env: Env` | `Result<Address, Error>` | Returns the current admin address. |
| `add_authorized_sender` | `env: Env, caller: Address, sender: Address` | `Result<(), Error>` | Authorise `sender` to create notifications on behalf of integrated contracts. |
| `get_authorized_senders` | `env: Env` | `Result<Vec<Address>, Error>` | Returns the list of all currently authorised sender addresses. |
| `get_unread_count` | `env: Env, user: Address` | `Result<u32, Error>` | Returns the number of unread (Pending + Delivered) notifications for a user. |
| `mark_read` | `env: Env, caller: Address, notif_id: u64` | `Result<(), Error>` | Mark a single notification as Read. Only the recipient may call this. |
| `mark_all_read` | `env: Env, caller: Address` | `Result<u32, Error>` | Mark all Pending / Delivered notifications for the caller as Read. Returns the count of newly-read notifications. |
| `archive_notification` | `env: Env, caller: Address, notif_id: u64` | `Result<(), Error>` | Archive a notification so it no longer appears in default queries. Caller must be the recipient or admin. |
| `delete_alert_rule` | `env: Env, caller: Address, rule_id: u64` | `Result<(), Error>` | Permanently delete an alert rule. |
| `get_alert_rules` | `env: Env, caller: Address` | `Result<Vec<AlertRule>, Error>` | Returns all non-deleted alert rules. Admin only. |
| `get_analytics` | `env: Env, caller: Address` | `Result<NotificationAnalytics, Error>` | Returns aggregated send/read/pending counters. Admin only. |

### Types

#### `enum DataKey`

| Variant | Value | Description |
|---|---|---|
| `Singleton` | — | — |
| `lifecycle` | — | — |
| `stored` | — | — |
| `in` | — | — |
| `instance` | — | — |
| `storage` | — | — |
| `Initialized` | — | — |
| `Admin` | — | — |
| `Sender` | — | — |
| `authorization` | — | — |
| `stored` | — | — |
| `in` | — | — |
| `instance` | — | — |
| `storage` | — | — |
| `AuthorizedSenders` | — | — |
| `Vec` | — | — |
| `Address` | — | — |
| `bounded` | — | — |
| `by` | — | — |
| `MAX_SENDERS` | — | — |
| `Per` | — | — |
| `sender` | — | — |
| `rate` | — | — |
| `limiting` | — | — |
| `persistent` | — | — |
| `SenderRate` | — | — |
| `Address` | — | — |
| `SenderRateLimit` | — | — |
| `User` | — | — |
| `preferences` | — | — |
| `persistent` | — | — |
| `UserPrefs` | — | — |
| `Address` | — | — |
| `NotificationPreferences` | — | — |
| `Notification` | — | — |
| `records` | — | — |
| `persistent` | — | — |
| `NotifCount` | — | — |
| `u64` | — | — |
| `monotonic` | — | — |
| `ID` | — | — |
| `counter` | — | — |
| `Notif` | — | — |
| `u64` | — | — |
| `Notification` | — | — |
| `UserNotifIds` | — | — |
| `Address` | — | — |
| `Vec` | — | — |
| `u64` | — | — |
| `ordered` | — | — |
| `by` | — | — |
| `insertion` | — | — |
| `oldest` | — | — |
| `first` | — | — |
| `UserUnreadCount` | — | — |
| `Address` | — | — |
| `u32` | — | — |
| `Alert` | — | — |
| `rules` | — | — |
| `persistent` | — | — |
| `AlertRuleCount` | — | — |
| `u64` | — | — |
| `monotonic` | — | — |
| `ID` | — | — |
| `counter` | — | — |
| `AlertRule` | — | — |
| `u64` | — | — |
| `AlertRule` | — | — |
| `ActiveAlertRuleIds` | — | — |
| `Vec` | — | — |
| `u64` | — | — |
| `IDs` | — | — |
| `of` | — | — |
| `all` | — | — |
| `non` | — | — |
| `deleted` | — | — |
| `rules` | — | — |
| `Localised` | — | — |
| `templates` | — | — |
| `persistent` | — | — |
| `Key` | — | — |
| `notif_type_repr` | — | — |
| `locale` | — | — |
| `NotificationTemplate` | — | — |
| `Template` | — | — |
| `u32` | — | — |
| `String` | — | — |
| `Analytics` | — | — |
| `counters` | — | — |
| `persistent` | — | — |
| `TotalSent` | — | — |
| `u64` | — | — |
| `TotalRead` | — | — |
| `u64` | — | — |
| `TotalPending` | — | — |
| `u64` | — | — |
| `ByTypeSent` | — | — |
| `u32` | — | — |
| `u64` | — | — |
| `keyed` | — | — |
| `by` | — | — |
| `NotificationType` | — | — |
| `repr` | — | — |
| `ByPrioritySent` | — | — |
| `u32` | — | — |
| `u64` | — | — |
| `keyed` | — | — |
| `by` | — | — |
| `AlertPriority` | — | — |
| `repr` | — | — |

### Error Codes

| Variant | Code | Description |
|---|---|---|
| `Unauthorized` | 100 | — |
| `SenderNotAuthorized` | 120 | — |
| `BatchTooLarge` | 208 | — |
| `RecipientsEmpty` | 209 | — |
| `TitleTooLong` | 221 | — |
| `MessageTooLong` | 222 | — |
| `NameTooLong` | 223 | — |
| `LocaleTooLong` | 224 | — |
| `InvalidNotifType` | 241 | — |
| `TooManyEnabledTypes` | 242 | — |
| `NotInitialized` | 300 | — |
| `AlreadyInitialized` | 301 | — |
| `RateLimitExceeded` | 307 | — |
| `AlreadyRead` | 330 | — |
| `AlreadyArchived` | 331 | — |
| `NotificationNotFound` | 450 | — |
| `AlertRuleNotFound` | 451 | — |
| `TemplateNotFound` | 452 | — |
| `SenderNotFound` | 453 | — |
| `MaxSendersReached` | 510 | — |
| `MaxRulesReached` | 511 | — |
| `MaxNotificationsReached` | 512 | — |
| `MaxTemplatesReached` | 513 | — |

### Examples

#### `test_initialize_stores_admin`

```rust
let env = Env::default();
    let (client, admin) = setup(&env);
    assert_eq!(client.get_admin(), admin);
```

#### `test_double_initialize_fails`

```rust
let env = Env::default();
    let (client, admin) = setup(&env);
    env.mock_all_auths();
    assert!(matches!(
        client.try_initialize(&admin),
        Err(Ok(Error::AlreadyInitialized))
    ));
```

#### `test_get_admin_before_init_fails`

```rust
let env = Env::default();
    let contract_id = Address::generate(&env);
    env.register_contract(&contract_id, NotificationContract);
    let client = NotificationContractClient::new(&env, &contract_id);
    env.mock_all_auths();
    assert!(client.try_get_admin().is_err());
```

---

## patient_risk_stratification

### Functions

| Function | Parameters | Returns | Description |
|---|---|---|---|
| `initialize` | `env: Env, admin: Address` | `bool` | — |
| `get_risk_assessment` | `env: Env, assessment_id: u64` | `Option<RiskAssessment>` | — |
| `get_patient_risk_profile` | `env: Env, patient: Address` | `Option<PatientRiskProfile>` | — |
| `get_risk_model` | `env: Env, model_id: BytesN<32>` | `Option<RiskModel>` | — |

### Types

#### `enum RiskModelType`

| Variant | Value | Description |
|---|---|---|
| `Readmission` | — | — |
| `Mortality` | — | — |
| `Complications` | — | — |

#### `struct RiskModel`

| Field | Type | Description |
|---|---|---|
| `model_id` | `BytesN<32>` | — |
| `model_type` | `RiskModelType` | — |
| `specialty` | `String` | — |
| `version` | `String` | — |
| `min_confidence_bps` | `u32` | — |
| `enabled` | `bool` | — |
| `description` | `String` | — |

#### `struct RiskAssessment`

| Field | Type | Description |
|---|---|---|
| `assessment_id` | `u64` | — |
| `patient` | `Address` | — |
| `model_id` | `BytesN<32>` | — |
| `risk_score_bps` | `u32` | — |
| `confidence_bps` | `u32` | — |
| `assessment_date` | `u64` | — |
| `prediction_horizon_days` | `u32` | — |
| `risk_factors` | `Vec<RiskFactor>` | — |
| `interventions` | `Vec<InterventionRecommendation>` | — |
| `specialty` | `String` | — |
| `auc_score_bps` | `u32` | — |

#### `struct RiskFactor`

| Field | Type | Description |
|---|---|---|
| `factor_name` | `String` | — |
| `contribution_bps` | `i32` | — |
| `importance_bps` | `u32` | — |
| `category` | `String` | — |
| `explanation` | `String` | — |

#### `struct InterventionRecommendation`

| Field | Type | Description |
|---|---|---|
| `intervention_type` | `String` | — |
| `priority` | `u32` | — |
| `description` | `String` | — |
| `expected_impact_bps` | `u32` | — |
| `timeframe_days` | `u32` | — |
| `resources_needed` | `Vec<String>` | — |

#### `struct PatientRiskProfile`

| Field | Type | Description |
|---|---|---|
| `patient` | `Address` | — |
| `latest_assessment_id` | `u64` | — |
| `current_risk_level` | `String` | — |
| `risk_trend` | `String` | — |
| `last_updated` | `u64` | — |
| `total_assessments` | `u32` | — |
| `specialty_profiles` | `Map<String` | — |

#### `struct SpecialtyRiskSummary`

| Field | Type | Description |
|---|---|---|
| `specialty` | `String` | — |
| `avg_risk_score_bps` | `u32` | — |
| `high_risk_count` | `u32` | — |
| `last_assessment_date` | `u64` | — |

#### `enum DataKey`

| Variant | Value | Description |
|---|---|---|
| `Config` | — | — |
| `RiskModel` | — | — |
| `BytesN` | — | — |
| `32` | — | — |
| `Assessment` | — | — |
| `u64` | — | — |
| `PatientProfile` | — | — |
| `Address` | — | — |
| `AssessmentCounter` | — | — |
| `ModelRegistry` | — | — |
| `RiskModelType` | — | — |

#### `enum Error`

| Variant | Value | Description |
|---|---|---|
| `NotAuthorized` | 1 | — |
| `ConfigNotSet` | 2 | — |
| `ModelNotFound` | 3 | — |
| `InvalidScore` | 4 | — |
| `LowConfidence` | 5 | — |
| `AssessmentNotFound` | 6 | — |
| `InvalidModel` | 7 | — |
| `DuplicateModel` | 8 | — |

---

## payment_router

### Functions

| Function | Parameters | Returns | Description |
|---|---|---|---|
| `get_fee_config` | `env: Env` | `Option<RouterFeeConfig>` | — |
| `get_nonce` | `env: Env, caller: Address` | `u64` | — |
| `compute_split` | `env: Env, amount: i128` | `Result<(i128, i128), Error>` | — |

### Types

#### `enum Error`

| Variant | Value | Description |
|---|---|---|
| `InvalidFeeBps` | 1 | — |
| `FeeNotSet` | 2 | — |
| `Overflow` | 3 | — |
| `InsufficientFunds` | 10 | — |
| `DeadlineExceeded` | 11 | — |
| `InvalidSignature` | 12 | — |
| `UnauthorizedCaller` | 13 | — |
| `ContractPaused` | 14 | — |
| `StorageFull` | 15 | — |
| `CrossChainTimeout` | 16 | — |
| `ReplayDetected` | 17 | — |

#### `struct RouterFeeConfig`

| Field | Type | Description |
|---|---|---|
| `platform_fee_bps` | `u32` | — |
| `fee_receiver` | `Address` | — |

---

## pharma_supply_chain

### Functions

| Function | Parameters | Returns | Description |
|---|---|---|---|
| `initialize` | `env: Env, admin: Address` | `Result<(), Error>` | — |
| `run_compliance_check` | `env: Env, batch_id: u64` | `Result<bool, Error>` | — |
| `get_inventory_snapshot` | `env: Env, owner: Address` | `InventorySnapshot` | — |
| `get_batch` | `env: Env, batch_id: u64` | `Result<Batch, Error>` | — |
| `get_shipment` | `env: Env, shipment_id: u64` | `Result<Shipment, Error>` | — |

### Types

#### `enum Error`

| Variant | Value | Description |
|---|---|---|
| `AlreadyInitialized` | 1 | — |
| `NotInitialized` | 2 | — |
| `Unauthorized` | 3 | — |
| `ManufacturerNotFound` | 4 | — |
| `MedicationNotFound` | 5 | — |
| `BatchNotFound` | 6 | — |
| `ShipmentNotFound` | 7 | — |
| `InvalidInput` | 8 | — |
| `BatchAlreadyExists` | 9 | — |

#### `enum BatchStatus`

| Variant | Value | Description |
|---|---|---|
| `Manufactured` | 0 | — |
| `InTransit` | 1 | — |
| `Delivered` | 2 | — |
| `Recalled` | 3 | — |

#### `enum ShipmentStatus`

| Variant | Value | Description |
|---|---|---|
| `Created` | 0 | — |
| `InTransit` | 1 | — |
| `Delivered` | 2 | — |
| `Flagged` | 3 | — |

#### `struct Manufacturer`

| Field | Type | Description |
|---|---|---|
| `id` | `u64` | — |
| `operator` | `Address` | — |
| `name` | `String` | — |
| `license_number` | `String` | — |
| `active` | `bool` | — |

#### `struct Medication`

| Field | Type | Description |
|---|---|---|
| `id` | `u64` | — |
| `manufacturer_id` | `u64` | — |
| `name` | `String` | — |
| `ndc` | `String` | — |
| `requires_cold_chain` | `bool` | — |
| `min_temp_c` | `i32` | — |
| `max_temp_c` | `i32` | — |
| `regulatory_region` | `String` | — |

#### `struct Batch`

| Field | Type | Description |
|---|---|---|
| `id` | `u64` | — |
| `medication_id` | `u64` | — |
| `lot_number` | `String` | — |
| `quantity` | `u32` | — |
| `auth_hash` | `BytesN<32>` | — |
| `manufactured_at` | `u64` | — |
| `expires_at` | `u64` | — |
| `current_owner` | `Address` | — |
| `status` | `BatchStatus` | — |
| `compliance_ok` | `bool` | — |

#### `struct Shipment`

| Field | Type | Description |
|---|---|---|
| `id` | `u64` | — |
| `batch_id` | `u64` | — |
| `from` | `Address` | — |
| `to` | `Address` | — |
| `carrier_ref` | `String` | — |
| `status` | `ShipmentStatus` | — |
| `latest_temp_c` | `i32` | — |
| `latest_humidity_bps` | `u32` | — |
| `latitude_e6` | `i32` | — |
| `longitude_e6` | `i32` | — |
| `compliance_ok` | `bool` | — |
| `created_at` | `u64` | — |
| `delivered_at` | `u64` | — |

#### `struct InventorySnapshot`

| Field | Type | Description |
|---|---|---|
| `owner` | `Address` | — |
| `batch_count` | `u32` | — |
| `total_units` | `u32` | — |
| `cold_chain_violations` | `u32` | — |
| `last_updated` | `u64` | — |

#### `struct InventoryRecommendation`

| Field | Type | Description |
|---|---|---|
| `owner` | `Address` | — |
| `available_units` | `u32` | — |
| `forecast_units` | `u32` | — |
| `reorder_needed` | `bool` | — |
| `recommended_reorder_units` | `u32` | — |

#### `enum DataKey`

| Variant | Value | Description |
|---|---|---|
| `Admin` | — | — |
| `ManufacturerCount` | — | — |
| `Manufacturer` | — | — |
| `u64` | — | — |
| `MedicationCount` | — | — |
| `Medication` | — | — |
| `u64` | — | — |
| `BatchCount` | — | — |
| `Batch` | — | — |
| `u64` | — | — |
| `BatchByLotNumber` | — | — |
| `String` | — | — |
| `ShipmentCount` | — | — |
| `Shipment` | — | — |
| `u64` | — | — |

---

## predictive_analytics

### Functions

| Function | Parameters | Returns | Description |
|---|---|---|---|
| `get_prediction` | `env: Env, prediction_id: u64` | `Option<HealthPrediction>` | — |
| `get_config` | `env: Env` | `Option<PredictionConfig>` | — |
| `get_patient_summary` | `env: Env, patient: Address` | `Option<PatientPredictionsSummary>` | — |
| `get_model_metrics` | `env: Env, model_id: BytesN<32>` | `Option<PredictionMetrics>` | — |
| `has_high_risk_prediction` | `env: Env, patient: Address` | `bool` | — |
| `is_whitelisted_predictor` | `env: Env, predictor_addr: Address` | `bool` | — |

### Examples

#### `test_prediction_flow`

```rust
let env = Env::default();
    let contract_id = env.register_contract(None, PredictiveAnalyticsContract);
    let client = PredictiveAnalyticsContractClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    let predictor = Address::generate(&env);
    let patient = Address::generate(&env);

    client
```

#### `test_low_confidence_rejection`

```rust
let env = Env::default();
    let contract_id = env.register_contract(None, PredictiveAnalyticsContract);
    let client = PredictiveAnalyticsContractClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    let predictor = Address::generate(&env);
    let patient = Address::generate(&env);

    client
```

#### `test_config_updates`

```rust
let env = Env::default();
    let contract_id = env.register_contract(None, PredictiveAnalyticsContract);
    let client = PredictiveAnalyticsContractClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    let predictor = Address::generate(&env);

    client
        .mock_all_auths()
```

---

## provider_directory

### Functions

| Function | Parameters | Returns | Description |
|---|---|---|---|
| `initialize` | `env: Env, admin: Address` | `Result<(), Error>` | — |

### Types

#### `enum Error`

| Variant | Value | Description |
|---|---|---|
| `NotInitialized` | 1 | — |
| `AlreadyInitialized` | 2 | — |
| `RateLimitExceeded` | 3 | — |
| `NotAuthorized` | 4 | — |

#### `enum DataKey`

| Variant | Value | Description |
|---|---|---|
| `Admin` | — | — |
| `RateLimitConfig` | — | — |
| `SearchRateLimit` | — | — |
| `Address` | — | — |
| `ExemptInstitution` | — | — |
| `Address` | — | — |

#### `struct RateLimitConfig`

| Field | Type | Description |
|---|---|---|
| `max_searches` | `u32` | — |
| `window_secs` | `u64` | — |

#### `struct Provider`

| Field | Type | Description |
|---|---|---|
| `id` | `Address` | — |
| `name` | `String` | — |
| `specialty` | `String` | — |

### Examples

#### `test_initialize`

```rust
let env = Env::default();
    let admin = Address::generate(&env);
    let identity_registry = Address::generate(&env);
    
    let contract_id = env.register_contract(None, ProviderDirectoryContract);
    let client = ProviderDirectoryContractClient::new(&env, &contract_id);

    client.initialize(&admin, &identity_registry);
```

#### `test_profile_management`

```rust
let env = Env::default();
    env.mock_all_auths();
    
    let admin = Address::generate(&env);
    let identity_registry = Address::generate(&env);
    let provider = Address::generate(&env);
    
    let contract_id = env.register_contract(None, ProviderDirectoryContract);
    let client = ProviderDirectoryContractClient::new(&env, &contract_id);
```

#### `test_search_by_specialty`

```rust
let env = Env::default();
    env.mock_all_auths();
    
    let admin = Address::generate(&env);
    let identity_registry = Address::generate(&env);
    let p1 = Address::generate(&env);
    let p2 = Address::generate(&env);
    
    let contract_id = env.register_contract(None, ProviderDirectoryContract);
```

---

## public_health_surveillance

### Functions

| Function | Parameters | Returns | Description |
|---|---|---|---|
| `initialize` | `env: Env, admin: Address` | `Result<(), Error>` | Initialize the public health surveillance platform |
| `get_outbreak_data` | `env: Env, data_id: BytesN<32>` | `Result<OutbreakData, Error>` | Get outbreak data |
| `get_epidemic_model` | `env: Env, model_id: BytesN<32>` | `Result<EpidemicModel, Error>` | Get epidemic model |
| `get_public_health_alert` | `env: Env, alert_id: u64` | `Result<PublicHealthAlert, Error>` | Get public health alert |
| `get_privacy_budget` | `env: Env, user: Address` | `Result<u64, Error>` | Get privacy budget for address |

### Types

#### `enum DiseaseSeverity`

| Variant | Value | Description |
|---|---|---|
| `Low` | 1 | — |
| `Medium` | 2 | — |
| `High` | 3 | — |
| `Critical` | 4 | — |

#### `enum AlertType`

| Variant | Value | Description |
|---|---|---|
| `DiseaseOutbreak` | — | — |
| `EnvironmentalHazard` | — | — |
| `VaccineShortage` | — | — |
| `AntimicrobialResistance` | — | — |
| `SupplyChainDisruption` | — | — |
| `EmergingPathogen` | — | — |
| `SeasonalEpidemic` | — | — |

#### `enum OutbreakStatus`

| Variant | Value | Description |
|---|---|---|
| `Monitoring` | — | — |
| `Detected` | — | — |
| `Investigating` | — | — |
| `Confirmed` | — | — |
| `Contained` | — | — |
| `Resolved` | — | — |

#### `enum AggregationMethod`

| Variant | Value | Description |
|---|---|---|
| `DifferentialPrivacy` | — | — |
| `SecureMultipartyComputation` | — | — |
| `HomomorphicEncryption` | — | — |
| `ZeroKnowledgeProofs` | — | — |
| `FederatedLearning` | — | — |

#### `struct OutbreakData`

| Field | Type | Description |
|---|---|---|
| `data_id` | `BytesN<32>` | — |
| `encrypted_region` | `Bytes` | — |
| `disease_code` | `String` | — |
| `aggregated_cases` | `u64` | — |
| `time_period_start` | `u64` | — |
| `time_period_end` | `u64` | — |
| `aggregation_method` | `AggregationMethod` | — |
| `privacy_epsilon` | `u64` | — |
| `confidence_bps` | `u32` | — |
| `provider` | `Address` | — |
| `reported_at` | `u64` | — |

#### `struct EpidemicModel`

| Field | Type | Description |
|---|---|---|
| `model_id` | `BytesN<32>` | — |
| `disease_code` | `String` | — |
| `encrypted_scope` | `Bytes` | — |
| `model_type` | `String` | — |
| `r0_estimate` | `u64` | — |
| `incubation_days` | `u32` | — |
| `infectious_days` | `u32` | — |
| `case_fatality_bps` | `u32` | — |
| `encrypted_params` | `Bytes` | — |
| `prediction_horizon` | `u32` | — |
| `confidence_bps` | `u32` | — |
| `last_updated` | `u64` | — |
| `creator` | `Address` | — |

#### `struct PublicHealthAlert`

| Field | Type | Description |
|---|---|---|
| `alert_id` | `u64` | — |
| `alert_type` | `AlertType` | — |
| `severity` | `DiseaseSeverity` | — |
| `encrypted_affected_regions` | `Bytes` | — |
| `message` | `String` | — |
| `recommended_actions` | `Vec<String>` | — |
| `source` | `Address` | — |
| `created_at` | `u64` | — |
| `expires_at` | `u64` | — |
| `is_active` | `bool` | — |
| `acknowledgment_count` | `u32` | — |

#### `struct VaccinationCoverage`

| Field | Type | Description |
|---|---|---|
| `coverage_id` | `BytesN<32>` | — |
| `encrypted_region` | `Bytes` | — |
| `vaccine_type` | `String` | — |
| `encrypted_target_population` | `u64` | — |
| `coverage_bps` | `u32` | — |
| `aggregation_method` | `AggregationMethod` | — |
| `privacy_epsilon` | `u64` | — |
| `reporting_period_start` | `u64` | — |
| `reporting_period_end` | `u64` | — |
| `provider` | `Address` | — |
| `reported_at` | `u64` | — |

#### `struct EnvironmentalHealth`

| Field | Type | Description |
|---|---|---|
| `env_data_id` | `BytesN<32>` | — |
| `encrypted_location` | `Bytes` | — |
| `metric_type` | `String` | — |
| `aggregated_value` | `u64` | — |
| `risk_bps` | `u32` | — |
| `measurement_period_start` | `u64` | — |
| `measurement_period_end` | `u64` | — |
| `aggregation_method` | `AggregationMethod` | — |
| `privacy_epsilon` | `u64` | — |
| `monitoring_station` | `Address` | — |
| `measured_at` | `u64` | — |

#### `struct AntimicrobialResistance`

| Field | Type | Description |
|---|---|---|
| `amr_data_id` | `BytesN<32>` | — |
| `encrypted_region` | `Bytes` | — |
| `pathogen_code` | `String` | — |
| `antibiotic_class` | `String` | — |
| `resistance_bps` | `u32` | — |
| `aggregation_method` | `AggregationMethod` | — |
| `privacy_epsilon` | `u64` | — |
| `testing_lab` | `Address` | — |
| `tested_at` | `u64` | — |

#### `struct SocialHealthDeterminants`

| Field | Type | Description |
|---|---|---|
| `sdoh_data_id` | `BytesN<32>` | — |
| `encrypted_region` | `Bytes` | — |
| `determinant_type` | `String` | — |
| `aggregated_metric` | `u64` | — |
| `impact_bps` | `u32` | — |
| `aggregation_method` | `AggregationMethod` | — |
| `privacy_epsilon` | `u64` | — |
| `data_source` | `Address` | — |
| `collected_at` | `u64` | — |

#### `struct PublicHealthIntervention`

| Field | Type | Description |
|---|---|---|
| `intervention_id` | `BytesN<32>` | — |
| `intervention_type` | `String` | — |
| `encrypted_target_population` | `Bytes` | — |
| `encrypted_scope` | `Bytes` | — |
| `start_date` | `u64` | — |
| `end_date` | `u64` | — |
| `implementation_cost` | `u64` | — |
| `expected_outcomes` | `Vec<String>` | — |
| `effectiveness_bps` | `u32` | — |
| `aggregation_method` | `AggregationMethod` | — |
| `coordinator` | `Address` | — |
| `created_at` | `u64` | — |

#### `struct GlobalHealthCollaboration`

| Field | Type | Description |
|---|---|---|
| `collaboration_id` | `BytesN<32>` | — |
| `participants` | `Vec<Address>` | — |
| `collaboration_type` | `String` | — |
| `data_sharing_protocol` | `String` | — |
| `exchange_method` | `AggregationMethod` | — |
| `objectives` | `Vec<String>` | — |
| `start_date` | `u64` | — |
| `end_date` | `u64` | — |
| `lead_organization` | `Address` | — |
| `established_at` | `u64` | — |

#### `enum DataKey`

| Variant | Value | Description |
|---|---|---|
| `Initialized` | — | — |
| `Admin` | — | — |
| `OutbreakData` | — | — |
| `BytesN` | — | — |
| `32` | — | — |
| `EpidemicModel` | — | — |
| `BytesN` | — | — |
| `32` | — | — |
| `PublicHealthAlert` | — | — |
| `u64` | — | — |
| `VaccinationCoverage` | — | — |
| `BytesN` | — | — |
| `32` | — | — |
| `EnvironmentalHealth` | — | — |
| `BytesN` | — | — |
| `32` | — | — |
| `AntimicrobialResistance` | — | — |
| `BytesN` | — | — |
| `32` | — | — |
| `SocialHealthDeterminants` | — | — |
| `BytesN` | — | — |
| `32` | — | — |
| `PublicHealthIntervention` | — | — |
| `BytesN` | — | — |
| `32` | — | — |
| `GlobalHealthCollaboration` | — | — |
| `BytesN` | — | — |
| `32` | — | — |
| `AlertCounter` | — | — |
| `ModelCounter` | — | — |
| `CoverageCounter` | — | — |
| `InterventionCounter` | — | — |
| `CollaborationCounter` | — | — |
| `PrivacyBudget` | — | — |
| `Address` | — | — |

#### `enum Error`

| Variant | Value | Description |
|---|---|---|
| `AlreadyInitialized` | 1 | — |
| `NotInitialized` | 2 | — |
| `NotAuthorized` | 3 | — |
| `InvalidInput` | 4 | — |
| `DataNotFound` | 5 | — |
| `InvalidAggregationMethod` | 6 | — |
| `PrivacyBudgetExceeded` | 7 | — |
| `InsufficientPrivilege` | 8 | — |
| `InvalidSeverity` | 9 | — |
| `AlertExpired` | 10 | — |
| `ModelNotFound` | 11 | — |
| `InterventionNotFound` | 12 | — |
| `CollaborationNotFound` | 13 | — |
| `InvalidTimeRange` | 14 | — |
| `InvalidRegion` | 15 | — |

### Examples

#### `test_public_health_surveillance_initialization`

```rust
let env = Env::default();
    env.mock_all_auths();

    let (client, _id) = setup(&env);
    let admin = Address::generate(&env);

    client.initialize(&admin);

    // Test that initialization works
```

#### `test_outbreak_data_reporting`

```rust
let env = Env::default();
    env.mock_all_auths();

    let (client, _id) = setup(&env);
    let admin = Address::generate(&env);
    client.initialize(&admin);

    let provider = Address::generate(&env);
    let data_id = BytesN::from_array(&env, &[1u8; 32]);
```

#### `test_epidemic_model_creation`

```rust
let env = Env::default();
    env.mock_all_auths();

    let (client, _id) = setup(&env);
    let admin = Address::generate(&env);
    client.initialize(&admin);

    let modeler = Address::generate(&env);
    let model_id = BytesN::from_array(&env, &[2u8; 32]);
```

---

## reentrancy_guard

### Functions

| Function | Parameters | Returns | Description |
|---|---|---|---|
| `enter` | `env: &Env` | `bool` | Try to acquire the per-contract reentrancy lock. Returns `true` when the lock was acquired, or `false` if already locked. |
| `exit` | `env: &Env` | `()` | Release the per-contract reentrancy lock. |

---

## regulatory_compliance

### Functions

| Function | Parameters | Returns | Description |
|---|---|---|---|
| `initialize` | `env: Env, admin: Address` | `Result<(), Error>` | — |
| `set_rule` | `env: Env, framework: String, rule: ComplianceRule` | `Result<(), Error>` | — |
| `get_rule` | `env: Env, framework: String` | `Option<ComplianceRule>` | — |
| `grant_consent` | `env: Env, user: Address, action: String` | `Result<(), Error>` | — |
| `revoke_consent` | `env: Env, user: Address, action: String` | `Result<(), Error>` | — |
| `has_consent` | `env: Env, user: Address, action: String` | `bool` | — |
| `log_audit` | `env: Env, user: Address, action: String, details: String` | `()` | — |
| `get_audit_logs` | `env: Env, user: Address` | `Result<Vec<AuditLog>, Error>` | — |
| `invoke_right_to_be_forgotten` | `env: Env, user: Address` | `Result<(), Error>` | — |
| `is_forgotten` | `env: &Env, user: Address` | `bool` | — |

### Types

#### `enum Error`

| Variant | Value | Description |
|---|---|---|
| `NotInitialized` | 1 | — |
| `AlreadyInitialized` | 2 | — |
| `UserAlreadyForgotten` | 3 | — |
| `RuleNotConfigured` | 4 | — |
| `RightToBeForgottenDisabled` | 5 | — |

#### `enum DataResidency`

| Variant | Value | Description |
|---|---|---|
| `Global` | — | — |
| `EU` | — | — |
| `US` | — | — |
| `Local` | — | — |
| `String` | — | — |

#### `struct ComplianceRule`

| Field | Type | Description |
|---|---|---|
| `require_consent` | `bool` | — |
| `right_to_be_forgotten` | `bool` | — |
| `residency` | `DataResidency` | — |
| `strict_auditing` | `bool` | — |

#### `struct AuditLog`

| Field | Type | Description |
|---|---|---|
| `action` | `String` | — |
| `actor` | `Address` | — |
| `timestamp` | `u64` | — |
| `details` | `String` | — |

#### `enum DataKey`

| Variant | Value | Description |
|---|---|---|
| `Admin` | — | — |
| `Rule` | — | — |
| `String` | — | — |
| `Consent` | — | — |
| `Address` | — | — |
| `String` | — | — |
| `AuditLogs` | — | — |
| `Address` | — | — |
| `Forgotten` | — | — |
| `Address` | — | — |

---

## reputation

### Functions

| Function | Parameters | Returns | Description |
|---|---|---|---|
| `initialize` | `env: Env, admin: Address` | `Result<(), Error>` | — |
| `get_score` | `env: Env, user: Address` | `i128` | — |
| `mint` | `env: Env, user: Address, amount: i128` | `Result<(), Error>` | — |
| `slash` | `env: Env, user: Address, amount: i128` | `Result<(), Error>` | — |

### Types

#### `enum Error`

| Variant | Value | Description |
|---|---|---|
| `AlreadyInitialized` | 1 | — |
| `NotInitialized` | 2 | — |
| `NegativeAmount` | 3 | — |
| `InvalidAmount` | 4 | — |

---

## reputation_access_control

### Functions

| Function | Parameters | Returns | Description |
|---|---|---|---|
| `approve_request` | `env: Env, admin: Address, request_id: BytesN<32>` | `Result<(), Error>` | — |
| `deny_request` | `env: Env, admin: Address, request_id: BytesN<32>` | `Result<(), Error>` | — |
| `get_provider_requests` | `env: Env, provider: Address` | `Result<Vec<AccessRequest>, Error>` | — |

### Types

#### `enum Error`

| Variant | Value | Description |
|---|---|---|
| `AlreadyInitialized` | 1 | — |
| `NotInitialized` | 2 | — |
| `NotAuthorized` | 3 | — |
| `InsufficientReputation` | 4 | — |
| `AccessDenied` | 5 | — |
| `InvalidResource` | 6 | — |
| `PolicyNotFound` | 7 | — |
| `ProviderNotVerified` | 8 | — |
| `CredentialExpired` | 9 | — |

#### `enum ResourceType`

| Variant | Value | Description |
|---|---|---|
| `PatientRecords` | 0 | — |
| `MedicalPrescriptions` | 1 | — |
| `DiagnosticReports` | 2 | — |
| `SurgicalProcedures` | 3 | — |
| `EmergencyAccess` | 4 | — |
| `ResearchData` | 5 | — |
| `AdministrativeFunctions` | 6 | — |
| `ProviderDirectory` | 7 | — |
| `CredentialManagement` | 8 | — |

#### `enum AccessLevel`

| Variant | Value | Description |
|---|---|---|
| `None` | 0 | — |
| `Read` | 1 | — |
| `Write` | 2 | — |
| `Update` | 3 | — |
| `Delete` | 4 | — |
| `Admin` | 5 | — |

#### `enum TimeRestrictionPolicy`

| Variant | Value | Description |
|---|---|---|
| `None` | — | — |
| `Restricted` | — | — |
| `TimeRestriction` | — | — |

#### `struct AccessPolicy`

| Field | Type | Description |
|---|---|---|
| `resource_type` | `ResourceType` | — |
| `min_reputation_score` | `u32` | — |
| `required_credentials` | `Vec<Symbol>` | — |
| `access_level` | `AccessLevel` | — |
| `time_restriction` | `TimeRestrictionPolicy` | — |
| `special_conditions` | `Vec<Symbol>` | — |

#### `struct TimeRestriction`

| Field | Type | Description |
|---|---|---|
| `start_hour` | `u32` | — |
| `end_hour` | `u32` | — |
| `allowed_days` | `Vec<u32>` | — |
| `timezone` | `String` | — |

#### `struct AccessRequest`

| Field | Type | Description |
|---|---|---|
| `request_id` | `BytesN<32>` | — |
| `provider` | `Address` | — |
| `resource_type` | `ResourceType` | — |
| `requested_access` | `AccessLevel` | — |
| `timestamp` | `u64` | — |
| `justification` | `String` | — |
| `status` | `RequestStatus` | — |

#### `enum RequestStatus`

| Variant | Value | Description |
|---|---|---|
| `Pending` | 0 | — |
| `Approved` | 1 | — |
| `Denied` | 2 | — |
| `Expired` | 3 | — |

#### `enum DataKey`

| Variant | Value | Description |
|---|---|---|
| `Admin` | — | — |
| `Initialized` | — | — |
| `AccessPolicy` | — | — |
| `ResourceType` | — | — |
| `AccessRequest` | — | — |
| `BytesN` | — | — |
| `32` | — | — |
| `ProviderRequests` | — | — |
| `Address` | — | — |
| `Vec` | — | — |
| `BytesN` | — | — |
| `32` | — | — |
| `ProviderAccessLevel` | — | — |
| `Address` | — | — |
| `ResourceType` | — | — |
| `Current` | — | — |
| `access` | — | — |
| `level` | — | — |
| `ReputationThreshold` | — | — |
| `ResourceType` | — | — |
| `EmergencyAccess` | — | — |
| `Address` | — | — |
| `bool` | — | — |
| `for` | — | — |
| `emergency` | — | — |
| `access` | — | — |
| `granted` | — | — |

---

## reputation_integration

### Functions

| Function | Parameters | Returns | Description |
|---|---|---|---|
| `auto_sync_all_providers` | `env: Env, admin: Address` | `Result<u32, Error>` | — |
| `get_combined_score` | `env: Env, provider: Address` | `Result<i128, Error>` | — |
| `trigger_credential_sync` | `env: Env, provider: Address` | `Result<(), Error>` | — |
| `trigger_feedback_sync` | `env: Env, provider: Address` | `Result<(), Error>` | — |
| `trigger_conduct_sync` | `env: Env, provider: Address` | `Result<(), Error>` | — |

### Types

#### `enum Error`

| Variant | Value | Description |
|---|---|---|
| `AlreadyInitialized` | 1 | — |
| `NotInitialized` | 2 | — |
| `NotAuthorized` | 3 | — |
| `ProviderNotFound` | 4 | — |
| `ReputationContractNotFound` | 5 | — |
| `HealthcareReputationContractNotFound` | 6 | — |
| `InvalidScoreMapping` | 7 | — |
| `SyncFailed` | 8 | — |

#### `struct ScoreMapping`

| Field | Type | Description |
|---|---|---|
| `base_reputation_weight` | `u32` | — |
| `healthcare_reputation_weight` | `u32` | — |
| `adjustment_factor` | `i32` | — |
| `last_sync_timestamp` | `u64` | — |

#### `struct SyncRecord`

| Field | Type | Description |
|---|---|---|
| `provider` | `Address` | — |
| `base_score` | `i128` | — |
| `healthcare_score` | `u32` | — |
| `combined_score` | `i128` | — |
| `timestamp` | `u64` | — |
| `sync_type` | `SyncType` | — |

#### `enum SyncType`

| Variant | Value | Description |
|---|---|---|
| `Manual` | 0 | — |
| `Automatic` | 1 | — |
| `CredentialUpdate` | 2 | — |
| `FeedbackUpdate` | 3 | — |
| `ConductUpdate` | 4 | — |

#### `enum DataKey`

| Variant | Value | Description |
|---|---|---|
| `Admin` | — | — |
| `Initialized` | — | — |
| `BaseReputationContract` | — | — |
| `HealthcareReputationContract` | — | — |
| `ScoreMapping` | — | — |
| `SyncRecord` | — | — |
| `Address` | — | — |
| `u64` | — | — |
| `provider` | — | — |
| `timestamp` | — | — |
| `ProviderSyncList` | — | — |
| `Address` | — | — |
| `Vec` | — | — |
| `u64` | — | — |
| `timestamps` | — | — |
| `LastSyncTime` | — | — |
| `Address` | — | — |
| `u64` | — | — |
| `timestamp` | — | — |
| `SyncSettings` | — | — |

#### `struct SyncSettings`

| Field | Type | Description |
|---|---|---|
| `auto_sync_enabled` | `bool` | — |
| `sync_interval_hours` | `u32` | — |
| `sync_on_credential_change` | `bool` | — |
| `sync_on_feedback_change` | `bool` | — |
| `sync_on_conduct_change` | `bool` | — |

---

## runtime_validation

### Functions

| Function | Parameters | Returns | Description |
|---|---|---|---|
| `initialize` | `env: Env, admin: Address` | `Result<(), Error>` | Initialize the runtime validation system |
| `verify_permission` | `env: Env, check_id: String, user_role: String` | `Result<bool, Error>` | Check permission |
| `get_violation_report` | `env: Env, violation_id: u64` | `Result<ValidationReport, Error>` | Get validation report |
| `get_violation_count` | `env: Env` | `u64` | Get total violations |

### Error Codes

| Variant | Code | Description |
|---|---|---|
| `NotInitialized` | 1 | — |
| `AlreadyInitialized` | 2 | — |
| `NotAuthorized` | 3 | — |
| `CheckNotFound` | 4 | — |
| `CheckAlreadyExists` | 5 | — |
| `CheckNotActive` | 6 | — |
| `InvalidSeverity` | 7 | — |
| `InvalidResourceLimit` | 8 | — |
| `ResourceLimitExceeded` | 9 | — |
| `ViolationNotFound` | 10 | — |

### Examples

#### `test_initialize`

```rust
let env = Env::default();
        env.mock_all_auths();

        let admin = Address::generate(&env);
        let contract_id = env.register_contract(None, RuntimeValidation);
        let client = RuntimeValidationClient::new(&env, &contract_id);

        client.initialize(&admin);
    }
```

---

## sanitization

### Functions

| Function | Parameters | Returns | Description |
|---|---|---|---|
| `sanitize_string` | `_env: &Env, input: &String, max_len: u32` | `Result<(), SanitizationError>` | Validates a general-purpose string: non-empty, within `max_len` bytes, no null bytes, no ASCII control characters (allows tab/LF/CR). |
| `sanitize_name` | `_env: &Env, input: &String` | `Result<(), SanitizationError>` | Validates a human name: letters (any UTF-8), digits, spaces, hyphens, apostrophes, commas, and periods only (ASCII subset). |
| `sanitize_email` | `_env: &Env, input: &String` | `Result<(), SanitizationError>` | Validates an email address: single '@', non-empty local and domain parts, domain contains at least one '.', all chars from the RFC 5321 allowed set. |
| `sanitize_id` | `_env: &Env, input: &String` | `Result<(), SanitizationError>` | Validates an identifier: alphanumeric chars, hyphens, underscores, colons, dots, and forward slashes (covers DIDs, slugs, and resource paths). |
| `sanitize_url` | `_env: &Env, input: &String` | `Result<(), SanitizationError>` | Validates a URL: printable ASCII only, length within MAX_URL_LEN. |

### Types

#### `enum SanitizationError`

| Variant | Value | Description |
|---|---|---|
| `InputTooLong` | 1 | — |
| `EmptyInput` | 2 | — |
| `NullByte` | 3 | — |
| `InvalidCharacter` | 4 | — |
| `InvalidFormat` | 5 | — |

---

## secure_enclave

### Functions

| Function | Parameters | Returns | Description |
|---|---|---|---|
| `initialize` | `env: Env, genesis_key: BytesN<32>` | `()` | Initialize enclave infrastructure with a genesis signing key. |
| `rotate_signing_key` | `env: Env, new_key: BytesN<32>, grace_period_seconds: u64` | `()` | Performs atomic key rotation using a programmatic expiration cutoff window. |
| `verify_signature` | `env: Env, signer: BytesN<32>` | `bool` | Validates payload cryptographic authenticity across active key state boundaries. |
| `housekeeping` | `env: Env` | `bool` | Housekeeping endpoint used to explicitly clear old records and save storage fees. |
| `get_node` | `env: Env, node_id: BytesN<32>` | `Option<EnclaveNode>` | Resolves an existing registered Enclave Node data record. |
| `get_task` | `env: Env, task_id: BytesN<32>` | `Option<ProcessingTask>` | Resolves an existing assigned Processing Task details record. |

### Types

#### `enum EnclaveError`

| Variant | Value | Description |
|---|---|---|
| `NotInitialized` | 1 | — |
| `UnauthenticatedSigner` | 2 | — |
| `AlreadyInitialized` | 3 | — |
| `AttestationFailed` | 4 | — |
| `TaskNotFound` | 5 | — |

#### `struct EnclaveNode`

| Field | Type | Description |
|---|---|---|
| `id` | `BytesN<32>` | — |
| `public_key` | `BytesN<32>` | — |
| `status` | `u32` | — |
| `reputation_score` | `u32` | — |

#### `struct ProcessingTask`

| Field | Type | Description |
|---|---|---|
| `task_id` | `BytesN<32>` | — |
| `payload_hash` | `BytesN<32>` | — |
| `completed` | `bool` | — |
| `handled_by` | `BytesN<32>` | — |

#### `enum CoreStorageKey`

| Variant | Value | Description |
|---|---|---|
| `NodeRegistry` | — | — |
| `BytesN` | — | — |
| `32` | — | — |
| `TaskRegistry` | — | — |
| `BytesN` | — | — |
| `32` | — | — |

### Examples

#### `test_registration_and_attestation`

```rust
let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let contract_id = env.register_contract(None, SecureEnclaveContract);
    let client = SecureEnclaveContractClient::new(&env, &contract_id);

    client.initialize(&admin);
```

#### `test_submit_and_complete_task`

```rust
let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let contract_id = env.register_contract(None, SecureEnclaveContract);
    let client = SecureEnclaveContractClient::new(&env, &contract_id);

    client.initialize(&admin);
```

---

## storage_cleanup

### Functions

| Function | Parameters | Returns | Description |
|---|---|---|---|
| `initialize` | `env: Env, admin: Address` | `Result<(), Error>` | — |
| `set_paused` | `env: Env, caller: Address, paused: bool` | `Result<(), Error>` | — |
| `register_credential` | `env: Env, id: u64, expires_at: u64` | `()` | — |
| `register_audit_log` | `env: Env, id: u64, logged_at: u64` | `()` | — |
| `register_escrow` | `env: Env, id: u64, settled_at: u64` | `()` | — |
| `register_consent` | `env: Env, id: u64, revoked_at: u64` | `()` | — |
| `register_schedule` | `env: Env, id: u64, end_at: u64` | `()` | — |
| `cleanup_expired` | `env: Env, caller: Address, max_items: u32` | `Result<u32, Error>` | Clean up expired items across all categories. Returns total number of items removed. |
| `preview_cleanup` | `env: Env, max_items: u32` | `Result<u32, Error>` | Preview how many items would be cleaned without removing them. |
| `cleanup_credentials` | `env: Env, caller: Address, max_items: u32` | `Result<u32, Error>` | — |
| `cleanup_audit_logs` | `env: Env, caller: Address, max_items: u32` | `Result<u32, Error>` | — |
| `cleanup_escrows` | `env: Env, caller: Address, max_items: u32` | `Result<u32, Error>` | — |
| `cleanup_consents` | `env: Env, caller: Address, max_items: u32` | `Result<u32, Error>` | — |
| `cleanup_schedules` | `env: Env, caller: Address, max_items: u32` | `Result<u32, Error>` | — |
| `get_cleanup_log` | `env: Env` | `Vec<CleanupEntry>` | — |
| `is_paused` | `env: Env` | `bool` | — |

### Types

#### `enum DataKey`

| Variant | Value | Description |
|---|---|---|
| `Admin` | — | — |
| `Paused` | — | — |
| `Indexed` | — | — |
| `lists` | — | — |
| `of` | — | — |
| `item` | — | — |
| `IDs` | — | — |
| `per` | — | — |
| `category` | — | — |
| `kept` | — | — |
| `for` | — | — |
| `backward` | — | — |
| `compat` | — | — |
| `CredentialIds` | — | — |
| `AuditLogIds` | — | — |
| `EscrowIds` | — | — |
| `ConsentIds` | — | — |
| `ScheduleIds` | — | — |
| `Per` | — | — |
| `item` | — | — |
| `metadata` | — | — |
| `expiry` | — | — |
| `timestamp` | — | — |
| `CredentialExpiry` | — | — |
| `u64` | — | — |
| `AuditLogExpiry` | — | — |
| `u64` | — | — |
| `EscrowSettledAt` | — | — |
| `u64` | — | — |
| `ConsentRevokedAt` | — | — |
| `u64` | — | — |
| `ScheduleEndAt` | — | — |
| `u64` | — | — |
| `Cleanup` | — | — |
| `audit` | — | — |
| `trail` | — | — |
| `CleanupLog` | — | — |
| `Configurable` | — | — |
| `retention` | — | — |
| `overrides` | — | — |
| `RetentionConfig` | — | — |
| `Counter` | — | — |
| `based` | — | — |
| `pagination` | — | — |
| `replaces` | — | — |
| `unbounded` | — | — |
| `Vecs` | — | — |
| `CategoryCount` | — | — |
| `u32` | — | — |
| `category` | — | — |
| `count` | — | — |
| `CategoryEntry` | — | — |
| `u32` | — | — |
| `u32` | — | — |
| `category` | — | — |
| `idx` | — | — |
| `item_id` | — | — |

#### `struct RetentionConfig`

| Field | Type | Description |
|---|---|---|
| `credential_secs` | `u64` | — |
| `audit_log_secs` | `u64` | — |
| `escrow_secs` | `u64` | — |
| `consent_secs` | `u64` | — |
| `schedule_secs` | `u64` | — |

#### `struct CleanupEntry`

| Field | Type | Description |
|---|---|---|
| `timestamp` | `u64` | — |
| `caller` | `Address` | — |
| `category` | `u32` | — |
| `count` | `u32` | — |

#### `enum Error`

| Variant | Value | Description |
|---|---|---|
| `NotInitialized` | 1 | — |
| `AlreadyInitialized` | 2 | — |
| `NotAuthorized` | 3 | — |
| `Paused` | 4 | — |
| `BatchTooLarge` | 5 | — |

---

## sut_token

### Functions

| Function | Parameters | Returns | Description |
|---|---|---|---|
| `name` | `env: Env` | `Result<String, Error>` | Get token name |
| `symbol` | `env: Env` | `Result<String, Error>` | Get token symbol |
| `decimals` | `env: Env` | `Result<u32, Error>` | Get token decimals |
| `total_supply` | `env: Env` | `Result<i128, Error>` | Get total supply |
| `supply_cap` | `env: Env` | `Result<i128, Error>` | Get supply cap |
| `balance_of` | `env: Env, account: Address` | `i128` | Get balance of an address |
| `allowance` | `env: Env, owner: Address, spender: Address` | `i128` | Get allowance between owner and spender |
| `transfer` | `env: Env, from: Address, to: Address, amount: i128` | `Result<(), Error>` | Transfer tokens |
| `approve` | `env: Env, owner: Address, spender: Address, amount: i128` | `Result<(), Error>` | Approve spender to spend tokens |
| `mint` | `env: Env, minter: Address, to: Address, amount: i128` | `Result<(), Error>` | Mint new tokens (only by minter) |
| `burn` | `env: Env, minter: Address, from: Address, amount: i128` | `Result<(), Error>` | Burn tokens (only by minter) |
| `add_minter` | `env: Env, minter: Address` | `Result<(), Error>` | Add a new minter (only by admin) |
| `remove_minter` | `env: Env, minter: Address` | `Result<(), Error>` | Remove a minter (only by admin) |
| `is_minter` | `env: Env, address: Address` | `bool` | Check if address is a minter |
| `snapshot` | `env: Env` | `Result<u32, Error>` | Create a snapshot for voting/rewards |
| `balance_of_at` | `env: Env, account: Address, snapshot_id: u32` | `Result<i128, Error>` | Get balance at snapshot |
| `total_supply_at` | `env: Env, snapshot_id: u32` | `Result<i128, Error>` | Get total supply at snapshot |

### Types

#### `enum Error`

| Variant | Value | Description |
|---|---|---|
| `AlreadyInitialized` | 1 | — |
| `NotInitialized` | 2 | — |
| `Unauthorized` | 3 | — |
| `InsufficientBalance` | 4 | — |
| `InsufficientAllowance` | 5 | — |
| `ExceedsSupplyCap` | 6 | — |
| `InvalidAmount` | 7 | — |
| `InvalidAddress` | 8 | — |
| `SnapshotNotFound` | 9 | — |
| `Overflow` | 10 | — |
| `IndexOutOfBounds` | 11 | — |

#### `struct TokenMetadata`

| Field | Type | Description |
|---|---|---|
| `name` | `String` | — |
| `symbol` | `String` | — |
| `decimals` | `u32` | — |

#### `struct TokenInfo`

| Field | Type | Description |
|---|---|---|
| `total_supply` | `i128` | — |
| `supply_cap` | `i128` | — |
| `admin` | `Address` | — |

#### `struct Snapshot`

| Field | Type | Description |
|---|---|---|
| `block_number` | `u32` | — |
| `total_supply` | `i128` | — |

#### `struct Checkpoint`

| Field | Type | Description |
|---|---|---|
| `snapshot_id` | `u32` | — |
| `balance` | `i128` | — |

#### `enum DataKey`

| Variant | Value | Description |
|---|---|---|
| `Metadata` | — | — |
| `TokenInfo` | — | — |
| `Balance` | — | — |
| `Address` | — | — |
| `Allowance` | — | — |
| `Address` | — | — |
| `Address` | — | — |
| `owner` | — | — |
| `spender` | — | — |
| `Minter` | — | — |
| `Address` | — | — |
| `Snapshot` | — | — |
| `u32` | — | — |
| `snapshot_id` | — | — |
| `SnapshotCount` | — | — |
| `UserCheckpoints` | — | — |
| `Address` | — | — |
| `Vec` | — | — |
| `Checkpoint` | — | — |
| `for` | — | — |
| `user` | — | — |
| `UserCheckpointCount` | — | — |
| `Address` | — | — |
| `number` | — | — |
| `of` | — | — |
| `checkpoints` | — | — |
| `for` | — | — |
| `user` | — | — |

#### `struct TransferEvent`

| Field | Type | Description |
|---|---|---|
| `from` | `Address` | — |
| `to` | `Address` | — |
| `amount` | `i128` | — |

#### `struct ApprovalEvent`

| Field | Type | Description |
|---|---|---|
| `owner` | `Address` | — |
| `spender` | `Address` | — |
| `amount` | `i128` | — |

#### `struct MintEvent`

| Field | Type | Description |
|---|---|---|
| `to` | `Address` | — |
| `amount` | `i128` | — |

#### `struct BurnEvent`

| Field | Type | Description |
|---|---|---|
| `from` | `Address` | — |
| `amount` | `i128` | — |

#### `struct SnapshotEvent`

| Field | Type | Description |
|---|---|---|
| `id` | `u32` | — |
| `block_number` | `u32` | — |

### Examples

#### `test_initialize`

```rust
let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let contract_id = create_token_contract(&env);

    let (name, symbol, decimals, supply_cap) = initialize_token(&env, &contract_id, &admin);
    let client = SutTokenClient::new(&env, &contract_id);
```

#### `test_initialize_twice_fails`

```rust
let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let contract_id = create_token_contract(&env);
    let client = SutTokenClient::new(&env, &contract_id);

    let name = String::from_str(&env, "Test Token");
    let symbol = String::from_str(&env, "TEST");
```

#### `test_mint`

```rust
let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let user = Address::generate(&env);
    let contract_id = create_token_contract(&env);

    initialize_token(&env, &contract_id, &admin);
    let client = SutTokenClient::new(&env, &contract_id);
```

---

## timelock

### Functions

| Function | Parameters | Returns | Description |
|---|---|---|---|
| `initialize` | `env: Env, admin: Address, delay_seconds: u64` | `Result<(), Error>` | — |
| `get_config` | `env: Env` | `Option<TimelockConfig>` | — |
| `queue` | `env: Env, id: u64, target: Address, call: BytesN<32>` | `Result<(), Error>` | — |
| `execute` | `env: Env, id: u64` | `Result<(), Error>` | — |

### Types

#### `struct TimelockConfig`

| Field | Type | Description |
|---|---|---|
| `admin` | `Address` | — |
| `delay_seconds` | `u64` | — |

#### `struct QueuedTx`

| Field | Type | Description |
|---|---|---|
| `target` | `Address` | — |
| `call` | `BytesN<32>` | — |
| `eta` | `u64` | — |

### Error Codes

| Variant | Code | Description |
|---|---|---|
| `Unauthorized` | 100 | — |
| `InvalidSignature` | 207 | — |
| `NotInitialized` | 300 | — |
| `AlreadyInitialized` | 301 | — |
| `ContractPaused` | 302 | — |
| `DeadlineExceeded` | 306 | — |
| `AlreadyQueued` | 375 | — |
| `NotQueued` | 372 | — |
| `NotReady` | 376 | — |
| `ReentrancyRejected` | 377 | — |
| `InsufficientFunds` | 500 | — |
| `StorageFull` | 502 | — |
| `CrossChainTimeout` | 702 | — |

---

## token_sale

### Functions

| Function | Parameters | Returns | Description |
|---|---|---|---|

### Error Codes

| Variant | Code | Description |
|---|---|---|
| `AlreadyInitialized` | 1 | — |
| `InvalidArgument` | 2 | — |
| `Overflow` | 3 | — |
| `PhaseNotFound` | 4 | — |
| `PhaseClosed` | 5 | — |
| `CapExceeded` | 6 | — |
| `NotFinalized` | 7 | — |
| `AlreadyClaimed` | 8 | — |
| `RefundsNotEnabled` | 9 | — |
| `Paused` | 10 | — |
| `ReplayDetected` | 11 | — |
| `InsufficientFunds` | 500 | — |

### Examples

#### `test_token_sale_initialization`

```rust
let env = Env::default();
    env.mock_all_auths();

    let owner = Address::generate(&env);
    let treasury = Address::generate(&env);
    let (token_address, _token_client, _token_admin) = create_token_contract(&env, &owner);

    let contract_id = env.register_contract(None, TokenSaleContract);
    let client = TokenSaleContractClient::new(&env, &contract_id);
```

#### `test_add_sale_phase`

```rust
let env = Env::default();
    env.mock_all_auths();

    let owner = Address::generate(&env);
    let treasury = Address::generate(&env);
    let (token_address, _token_client, _token_admin) = create_token_contract(&env, &owner);

    let contract_id = env.register_contract(None, TokenSaleContract);
    let client = TokenSaleContractClient::new(&env, &contract_id);
```

#### `test_contribution_and_claim`

```rust
let env = Env::default();
    env.mock_all_auths();

    let owner = Address::generate(&env);
    let treasury = Address::generate(&env);
    let contributor = Address::generate(&env);

    let (sut_token_address, sut_token_client, sut_token_admin) =
        create_token_contract(&env, &owner);
```

---

## upgrade_manager

### Functions

| Function | Parameters | Returns | Description |
|---|---|---|---|
| `initialize` | `env: Env, admin: Address, validators: Vec<Address>` | `Result<(), Error>` | — |
| `approve` | `env: Env, validator: Address, proposal_id: u64` | `Result<(), Error>` | — |
| `execute` | `env: Env, proposal_id: u64` | `Result<(), Error>` | — |
| `execute_emergency` | `env: Env, proposal_id: u64` | `Result<(), Error>` | — |
| `validate_proposal` | `env: Env, proposal_id: u64` | `Result<UpgradeValidation, Error>` | — |

### Types

#### `struct UpgradeProposal`

| Field | Type | Description |
|---|---|---|
| `target` | `Address` | — |
| `new_wasm_hash` | `BytesN<32>` | — |
| `new_version` | `u32` | — |
| `description` | `Symbol` | — |
| `proposer` | `Address` | — |
| `created_at` | `u64` | — |
| `executable_at` | `u64` | — |
| `executed` | `bool` | — |
| `canceled` | `bool` | — |
| `approvals` | `Vec<Address>` | — |
| `is_emergency` | `bool` | — |

#### `struct Config`

| Field | Type | Description |
|---|---|---|
| `admin` | `Address` | — |
| `min_delay` | `u64` | — |
| `required_approvals` | `u32` | — |
| `validators` | `Vec<Address>` | — |
| `emergency_approvals` | `u32` | — |

### Error Codes

| Variant | Code | Description |
|---|---|---|
| `NotAValidator` | 110 | — |
| `NotEnoughApprovals` | 120 | — |
| `AlreadyInitialized` | 301 | — |
| `InvalidState` | 304 | — |
| `TimelockNotExpired` | 376 | — |
| `ConfigNotFound` | 390 | — |
| `ProposalNotFound` | 450 | — |
| `AlreadyApproved` | 451 | — |

### Examples

#### `test_complex_upgrade_flow`

```rust
let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let v1 = Address::generate(&env);
    let v2 = Address::generate(&env);
    let v3 = Address::generate(&env);
    let validators = Vec::from_array(&env, [v1.clone(), v2.clone(), v3.clone()]);
```

#### `test_error_codes_are_stable`

```rust
use crate::errors::Error;
    assert_eq!(Error::NotAValidator as u32, 110);
    assert_eq!(Error::NotEnoughApprovals as u32, 120);
    assert_eq!(Error::AlreadyInitialized as u32, 301);
    assert_eq!(Error::InvalidState as u32, 304);
    assert_eq!(Error::TimelockNotExpired as u32, 376);
    assert_eq!(Error::ProposalNotFound as u32, 450);
```

#### `test_get_suggestion_returns_expected_hint`

```rust
use crate::errors::{get_suggestion, Error};
    assert_eq!(
        get_suggestion(Error::NotAValidator),
        symbol_short!("CHK_AUTH")
    );
    assert_eq!(
        get_suggestion(Error::AlreadyInitialized),
        symbol_short!("ALREADY")
    );
```

---

## upgradeability

### Functions

| Function | Parameters | Returns | Description |
|---|---|---|---|
| `get_version` | `env: &Env` | `u32` | — |
| `set_version` | `env: &Env, version: u32` | `()` | — |
| `get_admin` | `env: &Env` | `Option<Address>` | — |
| `set_admin` | `env: &Env, admin: &Address` | `()` | — |
| `is_frozen` | `env: &Env` | `bool` | — |
| `freeze` | `env: &Env` | `()` | — |
| `add_history` | `env: &Env, history: UpgradeHistory` | `()` | — |
| `get_history` | `env: &Env` | `Vec<UpgradeHistory>` | — |
| `set_deprecated_functions` | `env: &Env, deprecations: &Vec<DeprecatedFunction>` | `()` | — |
| `get_deprecated_functions` | `env: &Env` | `Vec<DeprecatedFunction>` | — |
| `authorize_upgrade` | `env: &Env` | `Result<Address, UpgradeError>` | — |
| `rollback` | `env: &Env` | `Result<(), UpgradeError>` | — |
| `get_deprecated_functions` | `env: &Env` | `Vec<DeprecatedFunction>` | — |
| `get_deprecated_function` | `env: &Env, function: Symbol` | `Option<DeprecatedFunction>` | — |
| `emit_deprecation_warning` | `env: &Env, function: Symbol` | `Result<(), UpgradeError>` | — |

### Types

#### `enum UpgradeError`

| Variant | Value | Description |
|---|---|---|
| `NotAuthorized` | 100 | — |
| `InvalidWasmHash` | 101 | — |
| `VersionAlreadyExists` | 102 | — |
| `MigrationFailed` | 103 | — |
| `IncompatibleVersion` | 104 | — |
| `ContractPaused` | 105 | — |
| `HistoryNotFound` | 106 | — |
| `IntegrityCheckFailed` | 107 | — |
| `DeprecatedFunctionNotTracked` | 108 | — |

#### `struct UpgradeHistory`

| Field | Type | Description |
|---|---|---|
| `wasm_hash` | `BytesN<32>` | — |
| `version` | `u32` | — |
| `upgraded_at` | `u64` | — |
| `description` | `Symbol` | — |
| `state_hash` | `BytesN<32>` | — |

#### `struct DeprecatedFunction`

| Field | Type | Description |
|---|---|---|
| `function` | `Symbol` | — |
| `since` | `String` | — |
| `replacement` | `Option<Symbol>` | — |
| `removed_in` | `Option<String>` | — |
| `note` | `String` | — |
| `migration_guide` | `Option<String>` | — |

### Examples

#### `test_deprecated_functions_are_tracked`

```rust
let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register_contract(None, TestContract);
    let admin = Address::generate(&env);

    let deprecation = sample_deprecation(&env);

    env.as_contract(&contract_id, || {
```

#### `test_deprecation_warning_emits_event`

```rust
let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register_contract(None, TestContract);
    let admin = Address::generate(&env);

    let deprecation = sample_deprecation(&env);

    env.as_contract(&contract_id, || {
```

---

## zk_verifier

### Functions

| Function | Parameters | Returns | Description |
|---|---|---|---|
| `initialize` | `env: Env, admin: Address, default_ttl: u64` | `Result<(), Error>` | — |
| `set_default_ttl` | `env: Env, caller: Address, ttl: u64` | `Result<bool, Error>` | — |
| `get_default_ttl` | `env: Env` | `u64` | — |
| `get_verifying_key` | `env: Env, version: u32` | `Option<VerifyingKeyConfig>` | — |
| `get_current_version` | `env: Env` | `u32` | — |
| `set_minimum_attestors` | `env: Env, caller: Address, min: u32` | `Result<bool, Error>` | — |
| `get_minimum_attestors` | `env: Env` | `u32` | — |
| `compute_proof_hash` | `env: Env, proof: Bytes` | `BytesN<32>` | — |
| `mark_nullifier_used` | `env: Env, nullifier: BytesN<32>` | `bool` | — |
| `is_nullifier_used` | `env: Env, nullifier: BytesN<32>` | `bool` | — |

### Types

#### `struct VerifyingKeyConfig`

| Field | Type | Description |
|---|---|---|
| `version` | `u32` | — |
| `vk_hash` | `BytesN<32>` | — |
| `circuit_id` | `BytesN<32>` | — |
| `attestor` | `Address` | — |
| `metadata_hash` | `BytesN<32>` | — |
| `created_at` | `u64` | — |
| `active` | `bool` | — |

#### `struct ProofAttestation`

| Field | Type | Description |
|---|---|---|
| `vk_version` | `u32` | — |
| `public_inputs_hash` | `BytesN<32>` | — |
| `proof_hash` | `BytesN<32>` | — |
| `verified` | `bool` | — |
| `attestor` | `Address` | — |
| `created_at` | `u64` | — |
| `expires_at` | `u64` | — |

#### `struct NullifierRecord`

| Field | Type | Description |
|---|---|---|
| `nullifier` | `BytesN<32>` | — |
| `consumed_at` | `u64` | — |

#### `enum DataKey`

| Variant | Value | Description |
|---|---|---|
| `Initialized` | — | — |
| `Admin` | — | — |
| `CurrentVersion` | — | — |
| `DefaultTtl` | — | — |
| `VerifyingKey` | — | — |
| `u32` | — | — |
| `Attestation` | — | — |
| `BytesN` | — | — |
| `32` | — | — |
| `Nullifier` | — | — |
| `BytesN` | — | — |
| `32` | — | — |
| `MinimumAttestors` | — | — |
| `ProofAttestationCount` | — | — |
| `BytesN` | — | — |
| `32` | — | — |

### Error Codes

| Variant | Code | Description |
|---|---|---|
| `Unauthorized` | 100 | — |
| `InvalidInput` | 200 | — |
| `NotInitialized` | 300 | — |
| `AlreadyInitialized` | 301 | — |
| `VersionNotFound` | 430 | — |
| `InvalidProof` | 600 | — |
| `VerificationFailed` | 601 | — |
| `InsufficientAttestors` | 700 | — |

---

## zkp_registry

### Functions

| Function | Parameters | Returns | Description |
|---|---|---|---|
| `initialize` | `env: Env, admin: Address` | `Result<(), Error>` | — |
| `get_range_proof` | `env: Env, proof_id: BytesN<32>` | `Result<RangeProof, Error>` | — |
| `get_circuit_params` | `env: Env, circuit_id: String` | `Result<ZKPCircuitParams, Error>` | — |
| `get_gas_stats` | `env: Env, user: Address` | `Result<u64, Error>` | — |

### Types

#### `enum ZKPType`

| Variant | Value | Description |
|---|---|---|
| `SNARK` | — | — |
| `STARK` | — | — |
| `Bulletproof` | — | — |
| `PedersenCommitment` | — | — |
| `Recursive` | — | — |

#### `enum ZKPHashFunction`

| Variant | Value | Description |
|---|---|---|
| `Poseidon` | — | — |
| `MiMC` | — | — |
| `SHA256` | — | — |
| `Rescue` | — | — |

#### `struct ZKProof`

| Field | Type | Description |
|---|---|---|
| `proof_type` | `ZKPType` | — |
| `hash_function` | `ZKPHashFunction` | — |
| `circuit_id` | `String` | — |
| `public_inputs` | `Vec<Bytes>` | — |
| `proof_data` | `Bytes` | — |
| `vk_hash` | `BytesN<32>` | — |
| `verification_gas` | `u64` | — |
| `created_at` | `u64` | — |

#### `struct MedicalRecordProof`

| Field | Type | Description |
|---|---|---|
| `patient_id` | `Address` | — |
| `record_id` | `u64` | — |
| `authenticity_proof` | `ZKProof` | — |
| `access_proof` | `ZKProof` | — |
| `metadata_hash` | `BytesN<32>` | — |
| `is_verified` | `bool` | — |
| `verified_at` | `u64` | — |

#### `struct RangeProof`

| Field | Type | Description |
|---|---|---|
| `prover` | `Address` | — |
| `encrypted_value` | `Bytes` | — |
| `min_value` | `u64` | — |
| `max_value` | `u64` | — |
| `proof_data` | `Bytes` | — |
| `vk_hash` | `BytesN<32>` | — |
| `verification_gas` | `u64` | — |
| `created_at` | `u64` | — |

#### `struct CredentialProof`

| Field | Type | Description |
|---|---|---|
| `holder` | `Address` | — |
| `credential_type` | `String` | — |
| `issuer` | `Address` | — |
| `validity_proof` | `ZKProof` | — |
| `attribute_proof` | `ZKProof` | — |
| `encrypted_expiration` | `Bytes` | — |
| `is_verified` | `bool` | — |
| `verified_at` | `u64` | — |

#### `struct RecursiveProof`

| Field | Type | Description |
|---|---|---|
| `base_proof_id` | `BytesN<32>` | — |
| `recursive_proof` | `ZKProof` | — |
| `aggregated_vk` | `Bytes` | — |
| `composition_depth` | `u32` | — |
| `total_gas` | `u64` | — |
| `composed_at` | `u64` | — |

#### `struct ZKPCircuitParams`

| Field | Type | Description |
|---|---|---|
| `circuit_id` | `String` | — |
| `circuit_type` | `ZKPType` | — |
| `num_public_inputs` | `u32` | — |
| `num_private_inputs` | `u32` | — |
| `num_constraints` | `u32` | — |
| `security_param` | `u32` | — |
| `vk_hash` | `BytesN<32>` | — |
| `pk_hash` | `BytesN<32>` | — |
| `setup_at` | `u64` | — |
| `trusted_setup` | `bool` | — |

#### `struct ZKPVerificationResult`

| Field | Type | Description |
|---|---|---|
| `proof_id` | `BytesN<32>` | — |
| `is_valid` | `bool` | — |
| `gas_used` | `u64` | — |
| `verified_at` | `u64` | — |
| `verifier` | `Address` | — |
| `metadata` | `Bytes` | — |

#### `enum DataKey`

| Variant | Value | Description |
|---|---|---|
| `Initialized` | — | — |
| `Admin` | — | — |
| `ZKProof` | — | — |
| `BytesN` | — | — |
| `32` | — | — |
| `MedicalRecordProof` | — | — |
| `Address` | — | — |
| `u64` | — | — |
| `RangeProof` | — | — |
| `BytesN` | — | — |
| `32` | — | — |
| `CredentialProof` | — | — |
| `Address` | — | — |
| `String` | — | — |
| `RecursiveProof` | — | — |
| `BytesN` | — | — |
| `32` | — | — |
| `ZKPCircuitParams` | — | — |
| `String` | — | — |
| `VerificationResult` | — | — |
| `BytesN` | — | — |
| `32` | — | — |
| `ProofCounter` | — | — |
| `GasTracker` | — | — |
| `Address` | — | — |

#### `enum Error`

| Variant | Value | Description |
|---|---|---|
| `AlreadyInitialized` | 1 | — |
| `NotInitialized` | 2 | — |
| `NotAuthorized` | 3 | — |
| `InvalidProof` | 4 | — |
| `ProofNotFound` | 5 | — |
| `CircuitNotFound` | 6 | — |
| `VerificationFailed` | 7 | — |
| `GasLimitExceeded` | 8 | — |
| `InvalidInput` | 9 | — |
| `InvalidRange` | 10 | — |
| `CredentialExpired` | 11 | — |
| `InvalidCircuit` | 12 | — |
| `ProofTooLarge` | 13 | — |
| `RecursiveDepthExceeded` | 14 | — |
| `InvalidHashFunction` | 15 | — |
| `CommitmentMismatch` | 16 | — |
| `InvalidRangeProof` | 17 | — |
| `VersionMismatch` | 18 | — |

### Examples

#### `test_initialize_and_register_circuit`

```rust
let env = Env::default();
    env.mock_all_auths();
    env.ledger().set_timestamp(1_000_000);

    let (client, _id) = setup(&env);
    let admin = Address::generate(&env);
    client.initialize(&admin);

    let circuit_id = String::from_str(&env, "circuit-a");
```

#### `test_submit_zkp_smoke`

```rust
let env = Env::default();
    env.mock_all_auths();
    env.ledger().set_timestamp(1_000_000);

    let (client, _) = setup(&env);
    let admin = Address::generate(&env);
    client.initialize(&admin);

    let circuit_id = String::from_str(&env, "circuit-b");
```

#### `test_create_credential_proof_valid_future_window`

```rust
let env = Env::default();
    env.mock_all_auths();
    env.ledger().set_timestamp(1_000_000);
    let (client, _) = init_contract(&env);

    let holder = Address::generate(&env);
    let issuer = Address::generate(&env);
    let credential_type = String::from_str(&env, "medical_license");
    let validity_proof = make_proof(&env, b"validity", ZKPType::SNARK, ZKPHashFunction::SHA256);
```

---

