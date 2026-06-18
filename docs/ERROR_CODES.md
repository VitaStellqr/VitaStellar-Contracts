# Error Codes Reference

> Comprehensive reference of all contract error codes across the VitaStellar Contracts ecosystem.
> Auto-generated from contract source. Keep this file in sync with contract changes.

## Overview

All VitaStellar contracts use numeric error codes organized by category:

| Range | Category | Description |
|-------|----------|-------------|
| 1–99 | Contract-specific | Per-contract errors (unique to each contract) |
| 100–199 | Access Control | Authorization, authentication, permissions |
| 200–299 | Input Validation | Invalid arguments, format errors |
| 300–399 | Lifecycle & State | Initialization, pause, status transitions |
| 400–499 | Entity Existence | Not found, already exists |
| 500–599 | Financial & Resource | Funds, storage, limits |
| 600–699 | Cryptography | Key management, proofs |
| 700–799 | Cross-Chain | Bridge, oracle, chain operations |
| 800–899 | Reentrancy & Safety | Locking, circuit breaker |

---

## Access Control (100–199)

| Code | Symbol | Contract(s) | Description | Common Causes | Remediation |
|------|--------|-------------|-------------|---------------|-------------|
| 100 | `Unauthorized` | All | Caller lacks permission for this action | Invalid role, expired authorization, not authenticated | Verify caller identity, check role assignments, ensure authentication |
| 110 | `NotVerifier` | identity_registry | Caller is not a registered verifier | Address not in verifier registry | Register address as verifier first |
| 111 | `CannotRemoveOwner` | identity_registry | Cannot remove the owner as verifier | Attempt to remove the contract owner | Owner must remain a verifier |
| 120 | `InsufficientConfirmations` | cross_chain_access | Not enough validator confirmations | Message/operation has fewer confirmations than required threshold | Wait for more validators to confirm |
| 121 | `InsufficientOracleReports` | cross_chain_access | Insufficient oracle reports for consensus | Fewer reports received than MIN_ORACLE_REPORTS | Submit more oracle reports |
| 122 | `DuplicateOracleReport` | cross_chain_access | Oracle has already submitted a report | Duplicate submission from same oracle | Wait for other oracles to submit |

## Input Validation (200–299)

| Code | Symbol | Contract(s) | Description | Common Causes | Remediation |
|------|--------|-------------|-------------|---------------|-------------|
| 205 | `InvalidAmount` | escrow, payment_router | Invalid amount (zero or negative) | Amount <= 0 | Provide a positive amount |
| 207 | `InvalidSignature` | cross_chain_access, cross_chain_identity, timelock | Cryptographic signature validation failed | Corrupted signature, wrong key | Verify signing key and regenerate signature |
| 280 | `InvalidCoverage` | payment_router | Coverage policy validation failed | Missing policy data, invalid coverage BPS | Check policy parameters |
| 281 | `InvalidNonce` | cross_chain_access, cross_chain_identity | Nonce replay protection triggered | Duplicate nonce or nonce too low | Use a higher nonce value |
| 282 | `InvalidPayload` | cross_chain_access, cross_chain_identity | Message payload is invalid | Malformed data | Check payload format |
| 290 | `InvalidAddress` | cross_chain_access, cross_chain_identity | Chain address format is invalid | Wrong length or prefix | Use correct chain-specific address format |

## Lifecycle & State (300–399)

| Code | Symbol | Contract(s) | Description | Common Causes | Remediation |
|------|--------|-------------|-------------|---------------|-------------|
| 300 | `NotInitialized` | All | Contract has not been initialized | Missing `initialize()` call | Call `initialize()` first |
| 301 | `AlreadyInitialized` | All | Contract already initialized | Duplicate initialization attempt | Contract can only be initialized once |
| 302 | `ContractPaused` | All | Contract is paused; no state-changing calls allowed | Emergency pause or maintenance | Wait for unpause or contact admin |
| 303 | `CircuitOpen` | payment_router | Circuit breaker is open | Too many recent failures | Wait for recovery or admin intervention |
| 304 | `InvalidStatus` | payment_router | Claim/entity in wrong status for this operation | Wrong workflow step | Check current status and required transition |
| 305 | `AlreadyInState` | payment_router | Already in the requested state | Redundant state transition | No action needed |
| 306 | `DeadlineExceeded` | payment_router, timelock | Operation deadline/timelock has passed | Operation timed out | Retry with fresh operation |
| 372 | `NotQueued` | timelock | Transaction not in timelock queue | Wrong ID or already executed | Check queue ID |
| 375 | `AlreadyQueued` | timelock | Transaction already queued | Duplicate queue attempt | Use a different ID |
| 376 | `NotReady` | timelock | Timelock delay not yet elapsed | Too early to execute | Wait until ETA has passed |

## Entity Existence (400–499)

| Code | Symbol | Contract(s) | Description | Common Causes | Remediation |
|------|--------|-------------|-------------|---------------|-------------|
| 404 | `DIDNotFound` | identity_registry | DID not found | Address has no DID | Create DID first |
| 470 | `DIDNotFound` | identity_registry | DID document not found | Address has no DID | Create DID first |
| 471 | `DIDAlreadyExists` | identity_registry, escrow | DID/entity already registered/exists | Duplicate DID/order creation | Use existing DID or unique order ID |
| 472 | `DIDDeactivated` | identity_registry | DID has been deactivated | DID was intentionally deactivated | Cannot use deactivated DID |
| 480 | `ClaimNotFound` | payment_router | Claim not found | Wrong claim ID | Verify claim ID |
| 481 | `MessageNotFound` | cross_chain_access, cross_chain_identity | Cross-chain message not found | Wrong message ID | Verify message ID |
| 482 | `PreAuthNotFound` | payment_router | Pre-authorization not found | Wrong pre-auth ID | Verify pre-auth ID |
| 482 | `AlreadySettled` | escrow | Escrow already settled | Duplicate settlement attempt | No action needed |
| 483 | `PaymentPlanNotFound` | payment_router | Payment plan not found | Wrong plan ID | Verify plan ID |
| 484 | `InsuranceProviderNotFound` | payment_router | Insurance provider not found | Wrong provider ID | Verify provider ID |
| 485 | `CoveragePolicyNotFound` | payment_router | Coverage policy not found | Wrong policy ID | Verify policy ID |
| 486 | `EligibilityCheckNotFound` | payment_router | Eligibility check not found | No eligibility check performed | Run eligibility check first |
| 487 | `EobNotFound` | payment_router | Explanation of benefits not found | EOB not processed yet | Process claim to generate EOB |
| 488 | `AtomicTxNotFound` | cross_chain_access, cross_chain_identity | Atomic transaction not found | Wrong transaction ID | Verify tx ID |
| 489 | `RecordRefNotFound` | cross_chain_access, cross_chain_identity | Cross-chain record reference not found | Wrong record/chain pair | Register record reference first |
| 490 | `RollbackNotFound` | cross_chain_access, cross_chain_identity | Rollback record not found | Wrong operation ID | Verify operation ID |
| 491 | `RollbackAlreadyProcessed` | cross_chain_access, cross_chain_identity | Rollback already completed | Duplicate rollback | No action needed |
| 492 | `EventNotFound` | cross_chain_access, cross_chain_identity | Cross-chain event not found | Wrong event ID | Verify event ID |

## Financial & Resource (500–599)

| Code | Symbol | Contract(s) | Description | Common Causes | Remediation |
|------|--------|-------------|-------------|---------------|-------------|
| 500 | `InsufficientFunds` | payment_router, timelock, token_sale | Insufficient funds for operation | Not enough tokens | Add funds |
| 502 | `StorageFull` | payment_router, timelock | Storage capacity limit reached | Too much data stored | Clean up old data |
| 580 | `FraudDetected` | payment_router | Fraud report exists for this claim | Claim flagged as fraudulent | Resolve fraud report |
| 581 | `EscrowFailed` | payment_router | Escrow creation failed | Escrow contract rejected | Check escrow parameters |
| 582 | `UnsupportedTransaction` | payment_router | Unsupported transaction code | Wrong EDI format | Use supported transaction code |

## Cryptography (600–699)

| Code | Symbol | Contract(s) | Description | Common Causes | Remediation |
|------|--------|-------------|-------------|---------------|-------------|
| 600 | `InvalidKey` | crypto_registry | Invalid key format | Key is empty or malformed | Provide valid key data |
| 601 | `KeyNotFound` | crypto_registry | Key not found in registry | Wrong owner/version | Verify key owner and version |
| 602 | `KeyAlreadyRevoked` | crypto_registry | Key already revoked | Duplicate revocation | No action needed |
| 603 | `InvalidKeyLength` | crypto_registry | Key length doesn't match algorithm | Wrong key size for algorithm | Use correct key length |
| 604 | `CredentialNotFound` | identity_registry | Credential not found | Wrong credential ID | Verify credential ID |
| 605 | `CredentialExpired` | identity_registry | Credential has expired | Past expiration date | Renew credential |
| 610 | `ProofNotFound` | cross_chain_access, cross_chain_identity | Cryptographic proof not found | Wrong proof ID | Verify proof ID |
| 611 | `ProofAlreadyVerified` | cross_chain_access, cross_chain_identity | Proof already verified | Duplicate verification | No action needed |

## Cross-Chain (700–799)

| Code | Symbol | Contract(s) | Description | Common Causes | Remediation |
|------|--------|-------------|-------------|---------------|-------------|
| 702 | `CrossChainTimeout` | payment_router, timelock | Cross-chain operation timed out | Message not delivered in time | Retry or escalate |
| 703 | `InvalidChain` | cross_chain_access, cross_chain_identity | Invalid chain identifier | Chain not recognized | Check chain ID |
| 720 | `ChainNotSupported` | cross_chain_access, cross_chain_identity | Chain not in supported list | Chain not configured | Add chain to supported list |
| 721 | `OracleNotFound` | cross_chain_access, cross_chain_identity | Oracle node not found | Wrong oracle address | Verify oracle address |
| 722 | `OracleNotActive` | cross_chain_access, cross_chain_identity | Oracle not active | Oracle deactivated | Contact admin to reactivate |

## Reentrancy & Safety (800–899)

| Code | Symbol | Contract(s) | Description | Common Causes | Remediation |
|------|--------|-------------|-------------|---------------|-------------|
| 800 | `Reentrancy` | payment_router | Reentrancy guard triggered | Concurrent call detected | Retry after current operation completes |
| 801 | `OperationNotFound` | cross_chain_access, cross_chain_identity | Cross-chain operation not found | Wrong operation ID | Verify operation ID |
| 802 | `OperationExpired` | cross_chain_access, cross_chain_identity | Operation has expired | Deadline passed | Create new operation |
| 803 | `OperationAlreadyCompleted` | cross_chain_access, cross_chain_identity | Operation already completed | Duplicate completion | No action needed |
| 804 | `MaxExtensionsReached` | cross_chain_access, cross_chain_identity | Max timeout extensions reached | Too many extensions | Create new operation |

## Per-Contract Error Codes (1–99)
> Auto-generated from contract source. Do not edit manually.

## Per-Contract Error Codes

### anomaly_detection

| Code | Symbol | Description |
|------|--------|-------------|
| 1 | NotAuthorized | Generated from contract source |
| 2 | ConfigNotSet | Generated from contract source |
| 3 | Disabled | Generated from contract source |
| 4 | InvalidScore | Generated from contract source |
| 5 | InvalidSeverity | Generated from contract source |
| 6 | RecordNotFound | Generated from contract source |
| 7 | NotWhitelisted | Generated from contract source |
| 8 | AlertNotFound | Generated from contract source |
| 9 | AlertAlreadyResolved | Generated from contract source |

### anomaly_detector

| Code | Symbol | Description |
|------|--------|-------------|
| 1 | AlreadyInitialized | Generated from contract source |
| 2 | NotInitialized | Generated from contract source |
| 3 | NotAuthorized | Generated from contract source |
| 4 | ContractPaused | Generated from contract source |
| 5 | ModelNotFound | Generated from contract source |
| 6 | AlertNotFound | Generated from contract source |
| 7 | FeatureCountMismatch | Generated from contract source |
| 8 | InvalidWeight | Generated from contract source |
| 9 | InvalidThreshold | Generated from contract source |
| 10 | AlertAlreadyResolved | Generated from contract source |
| 11 | DuplicateFederatedUpdate | Generated from contract source |
| 12 | InvalidFeatureCount | Generated from contract source |
| 13 | InvalidScore | Generated from contract source |

### appointment_booking_escrow

| Code | Symbol | Description |
|------|--------|-------------|
| 100 | Unauthorized | Generated from contract source |
| 110 | OnlyPatientCanRefund | Generated from contract source |
| 111 | OnlyProviderCanConfirm | Generated from contract source |
| 205 | InvalidAmount | Generated from contract source |
| 210 | InvalidPatient | Generated from contract source |
| 211 | InvalidProvider | Generated from contract source |
| 300 | NotInitialized | Generated from contract source |
| 301 | AlreadyInitialized | Generated from contract source |
| 304 | InvalidState | Generated from contract source |
| 410 | AppointmentNotFound | Generated from contract source |
| 411 | AppointmentAlreadyConfirmed | Generated from contract source |
| 412 | AppointmentAlreadyRefunded | Generated from contract source |
| 413 | AppointmentNoShow | Generated from contract source |
| 500 | InsufficientFunds | Generated from contract source |
| 501 | TokenTransferFailed | Generated from contract source |
| 505 | DoubleWithdrawal | Generated from contract source |

### code_ownership

| Code | Symbol | Description |
|------|--------|-------------|
| 1 | NotInitialized | Generated from contract source |
| 2 | AlreadyInitialized | Generated from contract source |
| 3 | NotAuthorized | Generated from contract source |
| 4 | ModuleNotFound | Generated from contract source |
| 5 | ModuleAlreadyExists | Generated from contract source |
| 6 | ReviewRouteNotFound | Generated from contract source |
| 7 | InvalidOwnerCount | Generated from contract source |

### contract_template

| Code | Symbol | Description |
|------|--------|-------------|
| 1 | NotInitialized | Contract has not been initialized yet. |
| 2 | AlreadyInitialized | Contract has already been initialized. |
| 3 | Unauthorized | Caller is not authorized to perform this action. |
| 4 | InputTooLong | A string or bytes input exceeded the maximum allowed length. |

### contract_usage_analytics

| Code | Symbol | Description |
|------|--------|-------------|
| 1 | NotAuthorized | Generated from contract source |
| 2 | AlreadyInitialized | Generated from contract source |
| 3 | NotInitialized | Generated from contract source |
| 4 | InvalidInput | Generated from contract source |

### credential_registry

| Code | Symbol | Description |
|------|--------|-------------|
| 1 | AlreadyInitialized | Generated from contract source |
| 2 | NotInitialized | Generated from contract source |
| 3 | NotAuthorized | Generated from contract source |
| 4 | IssuerNotFound | Generated from contract source |
| 5 | RootVersionNotFound | Generated from contract source |
| 6 | InvalidCredentialId | Generated from contract source |
| 7 | InvalidExpiry | Generated from contract source |
| 8 | InvalidMetadata | Generated from contract source |
| 9 | InvalidSignature | Generated from contract source |

### cross_chain_access

| Code | Symbol | Description |
|------|--------|-------------|
| 1 | NotAuthorized | Generated from contract source |
| 2 | ContractPaused | Generated from contract source |
| 3 | AlreadyInitialized | Generated from contract source |
| 4 | GrantNotFound | Generated from contract source |
| 5 | GrantExpired | Generated from contract source |
| 6 | GrantRevoked | Generated from contract source |
| 7 | RequestNotFound | Generated from contract source |
| 8 | RequestExpired | Generated from contract source |
| 9 | RequestAlreadyProcessed | Generated from contract source |
| 10 | DelegationNotFound | Generated from contract source |
| 11 | DelegationExpired | Generated from contract source |
| 12 | InsufficientPermissions | Generated from contract source |
| 13 | EmergencyNotEnabled | Generated from contract source |
| 14 | EmergencyNotAuthorized | Generated from contract source |
| 15 | InvalidScope | Generated from contract source |
| 16 | InvalidCondition | Generated from contract source |
| 17 | AuditRequired | Generated from contract source |
| 18 | SingleUseConsumed | Generated from contract source |
| 19 | TimeRestrictionViolated | Generated from contract source |
| 20 | Overflow | Generated from contract source |
| 21 | SwapNotFound | Generated from contract source |
| 22 | SwapExpired | Generated from contract source |
| 23 | SwapAlreadyProcessed | Generated from contract source |

### cross_chain_enhancements

| Code | Symbol | Description |
|------|--------|-------------|
| 1 | NotAuthorized | Generated from contract source |
| 2 | NotInitialized | Generated from contract source |
| 3 | AlreadyInitialized | Generated from contract source |
| 4 | InvalidProof | Generated from contract source |
| 5 | ProofAlreadyVerified | Generated from contract source |
| 6 | ProofNotFound | Generated from contract source |
| 7 | ReplayDetected | Generated from contract source |
| 8 | RateLimitExceeded | Generated from contract source |
| 9 | ArithmeticOverflow | Generated from contract source |
| 10 | InvalidMerklePath | Generated from contract source |
| 11 | ExpiredMessage | Generated from contract source |

### cross_chain_identity

| Code | Symbol | Description |
|------|--------|-------------|
| 1 | NotAuthorized | Generated from contract source |
| 2 | ContractPaused | Generated from contract source |
| 3 | AlreadyInitialized | Generated from contract source |
| 4 | IdentityNotFound | Generated from contract source |
| 5 | IdentityAlreadyExists | Generated from contract source |
| 6 | IdentityExpired | Generated from contract source |
| 7 | IdentityRevoked | Generated from contract source |
| 8 | RequestNotFound | Generated from contract source |
| 9 | RequestExpired | Generated from contract source |
| 10 | RequestAlreadyProcessed | Generated from contract source |
| 11 | ValidatorNotFound | Generated from contract source |
| 12 | ValidatorNotActive | Generated from contract source |
| 13 | DuplicateAttestation | Generated from contract source |
| 14 | InsufficientAttestations | Generated from contract source |
| 15 | InvalidProof | Generated from contract source |
| 16 | InvalidChain | Generated from contract source |
| 17 | SyncNotFound | Generated from contract source |
| 18 | SyncFailed | Generated from contract source |

### crypto_registry

| Code | Symbol | Description |
|------|--------|-------------|
| 1 | AlreadyInitialized | Generated from contract source |
| 2 | NotInitialized | Generated from contract source |
| 3 | NotAuthorized | Generated from contract source |
| 4 | InvalidKey | Generated from contract source |
| 5 | KeyNotFound | Generated from contract source |
| 6 | KeyAlreadyRevoked | Generated from contract source |
| 7 | InvalidKeyLength | Generated from contract source |

### deprecation_framework

| Code | Symbol | Description |
|------|--------|-------------|
| 1 | NotInitialized | Generated from contract source |
| 2 | AlreadyInitialized | Generated from contract source |
| 3 | NotAuthorized | Generated from contract source |
| 4 | ContractNotFound | Generated from contract source |
| 5 | ContractAlreadyDeprecated | Generated from contract source |
| 6 | InvalidTimeline | Generated from contract source |
| 7 | InvalidPhaseTransition | Generated from contract source |
| 8 | TimelineNotFound | Generated from contract source |
| 9 | GuideNotFound | Generated from contract source |
| 10 | ChecklistNotFound | Generated from contract source |
| 11 | InvalidChecklistIndex | Generated from contract source |

### dispute_resolution

| Code | Symbol | Description |
|------|--------|-------------|
| 1 | NotInitialized | Generated from contract source |
| 2 | NotArbiter | Generated from contract source |
| 3 | DisputeNotFound | Generated from contract source |

### emr_integration

| Code | Symbol | Description |
|------|--------|-------------|
| 1 | NotAuthorized | Generated from contract source |
| 2 | ContractPaused | Generated from contract source |
| 3 | EMRSystemNotFound | Generated from contract source |
| 4 | EMRSystemAlreadyExists | Generated from contract source |
| 5 | OnboardingNotFound | Generated from contract source |
| 6 | OnboardingAlreadyExists | Generated from contract source |
| 7 | VerificationNotFound | Generated from contract source |
| 8 | NetworkNodeNotFound | Generated from contract source |
| 9 | AgreementNotFound | Generated from contract source |
| 10 | TestNotFound | Generated from contract source |
| 11 | InvalidStatus | Generated from contract source |
| 12 | InvalidEMRSystem | Generated from contract source |
| 13 | ProviderNotFound | Generated from contract source |
| 14 | InvalidNPI | Generated from contract source |
| 15 | InvalidLicenseNumber | Generated from contract source |
| 16 | LicenseExpired | Generated from contract source |
| 17 | InvalidAgreement | Generated from contract source |
| 18 | AgreementNotActive | Generated from contract source |
| 19 | TestFailed | Generated from contract source |
| 20 | InvalidTestType | Generated from contract source |
| 21 | DuplicateTest | Generated from contract source |
| 22 | FHIRContractNotSet | Generated from contract source |
| 23 | OperationFailed | Generated from contract source |
| 24 | UnsupportedMessageFormat | Generated from contract source |
| 25 | MessageParseFailed | Generated from contract source |
| 26 | UnsupportedMessageType | Generated from contract source |
| 27 | InvalidMessagePayload | Generated from contract source |
| 28 | MessageNotFound | Generated from contract source |
| 29 | ValidationReportNotFound | Generated from contract source |
| 30 | TransformationNotFound | Generated from contract source |
| 31 | UnsupportedEncoding | Generated from contract source |

### escrow

| Code | Symbol | Description |
|------|--------|-------------|
| 100 | Unauthorized | Generated from contract source |
| 102 | NotAdmin | Generated from contract source |
| 120 | InsufficientApprovals | Generated from contract source |
| 205 | InvalidAmount | Generated from contract source |
| 260 | InvalidFeeBps | Generated from contract source |
| 380 | FeeNotSet | Generated from contract source |
| 381 | ReentrancyGuard | Generated from contract source |
| 382 | InvalidStateTransition | Generated from contract source |
| 480 | EscrowExists | Generated from contract source |
| 481 | EscrowNotFound | Generated from contract source |
| 482 | AlreadySettled | Generated from contract source |
| 560 | NoBasisToRefund | Generated from contract source |
| 561 | NoCredit | Generated from contract source |
| 562 | Overflow | Generated from contract source |

### explainable_ai

| Code | Symbol | Description |
|------|--------|-------------|
| 1 | NotAuthorized | Generated from contract source |
| 2 | RequestNotFound | Generated from contract source |
| 3 | ExplanationNotFound | Generated from contract source |
| 4 | InvalidImportance | Generated from contract source |
| 5 | AuditNotFound | Generated from contract source |
| 6 | InvalidBPSValue | Generated from contract source |

### fido2_authenticator

| Code | Symbol | Description |
|------|--------|-------------|
| 1 | AlreadyInitialized | Generated from contract source |
| 2 | NotInitialized | Generated from contract source |
| 3 | NotAuthorized | Generated from contract source |
| 4 | DeviceNotFound | Generated from contract source |
| 5 | DeviceAlreadyRegistered | Generated from contract source |
| 6 | MaxDevicesReached | Generated from contract source |
| 7 | DeviceInactive | Generated from contract source |
| 8 | InvalidPublicKey | Generated from contract source |
| 9 | InvalidSignature | Signature or ZK proof verification failed. |
| 10 | InvalidAuthenticatorData | `authenticatorData` is malformed or too short. |
| 11 | ChallengeExpired | The pending challenge has expired (> 5 minutes old). |
| 12 | NoChallengeIssued | Authentication attempted without first issuing a challenge. |
| 13 | SignCountRegression | Sign count did not increase — possible credential clone detected. |
| 14 | InvalidDeviceName | Generated from contract source |
| 15 | InvalidCredentialIdHash | Generated from contract source |
| 16 | ZkVerifierNotSet | `verify_zk_assertion` called but no ZK verifier contract is configured. |
| 17 | NullifierAlreadyUsed | ZK proof nullifier has already been used (replay attack). |
| 18 | RpIdMismatch | `authenticatorData` rpIdHash does not match the contract's configured RP ID. |
| 19 | UserPresenceNotVerified | FIDO2 User Presence (UP) flag is not set in `authenticatorData`. |
| 20 | InvalidRevocationReason | Generated from contract source |
| 21 | AlgorithmKeyMismatch | `register_device` called with an algorithm mismatched to the public key size. |

### governor

| Code | Symbol | Description |
|------|--------|-------------|
| 280 | InvalidVoteType | Generated from contract source |
| 300 | NotInitialized | Generated from contract source |
| 301 | AlreadyInitialized | Generated from contract source |
| 304 | InvalidState | Generated from contract source |
| 370 | VotingClosed | Generated from contract source |
| 371 | AlreadyVoted | Generated from contract source |
| 372 | NotQueued | Generated from contract source |
| 373 | ProposalDisputed | Generated from contract source |
| 450 | ProposalNotFound | Generated from contract source |
| 451 | ProposalNotSuccessful | Generated from contract source |
| 452 | AlreadyExecuted | Generated from contract source |
| 530 | ProposalThresholdNotMet | Generated from contract source |
| 531 | NoVotingPower | Generated from contract source |
| 580 | Overflow | Generated from contract source |

### healthcare_data_conversion

| Code | Symbol | Description |
|------|--------|-------------|
| 1 | NotAuthorized | Generated from contract source |
| 2 | ContractPaused | Generated from contract source |
| 3 | RuleNotFound | Generated from contract source |
| 4 | CodingMappingNotFound | Generated from contract source |
| 5 | FormatNotSupported | Generated from contract source |
| 6 | ConversionFailed | Generated from contract source |
| 7 | ValidationFailed | Generated from contract source |
| 8 | InvalidConversionRequest | Generated from contract source |
| 9 | SourceFormatNotSupported | Generated from contract source |
| 10 | TargetFormatNotSupported | Generated from contract source |
| 11 | MappingTableNotFound | Generated from contract source |
| 12 | DuplicateRule | Generated from contract source |
| 13 | IncompatibleFormats | Generated from contract source |
| 14 | DataLossWarning | Generated from contract source |
| 15 | InvalidMappingData | Generated from contract source |
| 16 | OperationFailed | Generated from contract source |

### healthcare_reputation

| Code | Symbol | Description |
|------|--------|-------------|
| 1 | AlreadyInitialized | Generated from contract source |
| 2 | NotInitialized | Generated from contract source |
| 3 | NotAuthorized | Generated from contract source |
| 4 | ProviderNotFound | Generated from contract source |
| 5 | CredentialNotFound | Generated from contract source |
| 6 | InvalidCredentialType | Generated from contract source |
| 7 | CredentialExpired | Generated from contract source |
| 8 | CredentialRevoked | Generated from contract source |
| 9 | DuplicateCredential | Generated from contract source |
| 10 | InvalidRating | Generated from contract source |
| 11 | FeedbackNotFound | Generated from contract source |
| 12 | DisputeNotFound | Generated from contract source |
| 13 | InsufficientReputation | Generated from contract source |
| 14 | NotVerifiedProvider | Generated from contract source |
| 15 | InvalidConductEntry | Generated from contract source |
| 16 | ConductEntryNotFound | Generated from contract source |

### homomorphic_registry

| Code | Symbol | Description |
|------|--------|-------------|
| 1 | AlreadyInitialized | Generated from contract source |
| 2 | NotInitialized | Generated from contract source |
| 3 | NotAuthorized | Generated from contract source |
| 4 | ContextNotFound | Generated from contract source |
| 5 | ContextInactive | Generated from contract source |
| 6 | InvalidInput | Generated from contract source |
| 7 | ComputationAlreadyExists | Generated from contract source |
| 8 | CiphertextNotFound | Generated from contract source |
| 9 | CiphertextAlreadyExists | Generated from contract source |
| 10 | SchemeMismatch | Generated from contract source |
| 11 | IncompatibleDimensions | Generated from contract source |
| 12 | NoiseBudgetExhausted | Generated from contract source |
| 13 | ArithmeticOverflow | Generated from contract source |
| 14 | KeyNotFound | Generated from contract source |

### identity_registry

| Code | Symbol | Description |
|------|--------|-------------|
| 100 | Unauthorized | Generated from contract source |
| 110 | NotVerifier | Generated from contract source |
| 111 | CannotRemoveOwner | Generated from contract source |
| 120 | InvalidRecoveryGuardian | Generated from contract source |
| 121 | InsufficientGuardianApprovals | Generated from contract source |
| 200 | InvalidInput | Generated from contract source |
| 201 | InputTooLong | Generated from contract source |
| 250 | InvalidVerificationMethod | Generated from contract source |
| 251 | InvalidCredentialType | Generated from contract source |
| 252 | InvalidServiceEndpoint | Generated from contract source |
| 300 | NotInitialized | Generated from contract source |
| 301 | AlreadyInitialized | Generated from contract source |
| 302 | ContractPaused | Generated from contract source |
| 360 | RecoveryNotInitiated | Generated from contract source |
| 361 | RecoveryAlreadyPending | Generated from contract source |
| 362 | RecoveryTimelockNotElapsed | Generated from contract source |
| 450 | VerificationMethodNotFound | Generated from contract source |
| 460 | CredentialNotFound | Generated from contract source |
| 461 | AttestationNotFound | Generated from contract source |
| 462 | ServiceNotFound | Generated from contract source |
| 470 | DIDNotFound | Generated from contract source |
| 471 | DIDAlreadyExists | Generated from contract source |
| 472 | DIDDeactivated | Generated from contract source |
| 603 | KeyRotationCooldown | Generated from contract source |
| 605 | CredentialExpired | Generated from contract source |
| 606 | CredentialRevoked | Generated from contract source |

### ihe_integration

| Code | Symbol | Description |
|------|--------|-------------|
| 1 | NotInitialized | Generated from contract source |
| 2 | AlreadyInitialized | Generated from contract source |
| 3 | NotAuthorized | Generated from contract source |
| 4 | DocumentNotFound | Generated from contract source |
| 5 | DocumentAlreadyExists | Generated from contract source |
| 6 | DocumentDeprecated | Generated from contract source |
| 7 | PatientNotFound | Generated from contract source |
| 8 | CrossReferenceNotFound | Generated from contract source |
| 9 | DemographicsNotFound | Generated from contract source |
| 10 | AuditEventNotFound | Generated from contract source |
| 11 | GatewayNotFound | Generated from contract source |
| 12 | GatewayAlreadyExists | Generated from contract source |
| 13 | MasterPatientNotFound | Generated from contract source |
| 14 | ConsentNotFound | Generated from contract source |
| 15 | ConsentRevoked | Generated from contract source |
| 16 | ConsentExpired | Generated from contract source |
| 17 | SignatureNotFound | Generated from contract source |
| 18 | SignatureInvalid | Generated from contract source |
| 19 | ProviderNotFound | Generated from contract source |
| 20 | ValueSetNotFound | Generated from contract source |
| 21 | ValueSetOidExists | Generated from contract source |
| 22 | InvalidHL7Message | Generated from contract source |
| 23 | ConnectathonTestNotFound | Generated from contract source |
| 24 | EmptyPatientId | Generated from contract source |
| 25 | EmptyDocumentId | Generated from contract source |

### iot_device_management

| Code | Symbol | Description |
|------|--------|-------------|
| 100 | Unauthorized | Generated from contract source |
| 102 | NotAdmin | Generated from contract source |
| 115 | NotDeviceOperator | Generated from contract source |
| 116 | NotManufacturer | Generated from contract source |
| 201 | InputTooLong | Generated from contract source |
| 202 | InputTooShort | Generated from contract source |
| 240 | InvalidDeviceType | Generated from contract source |
| 250 | InvalidFirmwareHash | Generated from contract source |
| 260 | InvalidMetricValue | Generated from contract source |
| 270 | InvalidTimestamp | Generated from contract source |
| 300 | NotInitialized | Generated from contract source |
| 301 | AlreadyInitialized | Generated from contract source |
| 302 | ContractPaused | Generated from contract source |
| 303 | NotPaused | Generated from contract source |
| 405 | DeviceNotFound | Generated from contract source |
| 420 | DeviceAlreadyRegistered | Generated from contract source |
| 425 | ManufacturerNotRegistered | Generated from contract source |
| 426 | ManufacturerAlreadyRegistered | Generated from contract source |
| 430 | FirmwareVersionNotFound | Generated from contract source |
| 431 | FirmwareAlreadyExists | Generated from contract source |
| 440 | ChannelNotFound | Generated from contract source |
| 602 | InvalidEncryptionKey | Generated from contract source |
| 603 | KeyRotationTooFrequent | Generated from contract source |
| 820 | DeviceDecommissioned | Generated from contract source |
| 821 | FirmwareNotApproved | Generated from contract source |
| 822 | HeartbeatTooFrequent | Generated from contract source |
| 823 | DeviceNotActive | Generated from contract source |
| 824 | DeviceSuspended | Generated from contract source |
| 825 | DowngradeNotAllowed | Generated from contract source |
| 826 | DeviceOffline | Generated from contract source |

### medical_record_backup

| Code | Symbol | Description |
|------|--------|-------------|
| 1 | AlreadyInitialized | Generated from contract source |
| 2 | NotInitialized | Generated from contract source |
| 3 | NotAuthorized | Generated from contract source |
| 4 | ContractPaused | Generated from contract source |
| 5 | InvalidInput | Generated from contract source |
| 6 | TargetNotFound | Generated from contract source |
| 7 | BackupNotFound | Generated from contract source |
| 8 | RestoreRequestNotFound | Generated from contract source |
| 9 | RecoveryTestNotFound | Generated from contract source |
| 10 | ScheduleNotDue | Generated from contract source |
| 11 | InsufficientTargets | Generated from contract source |
| 12 | GeoRedundancyNotMet | Generated from contract source |
| 13 | EncryptionRequired | Generated from contract source |
| 14 | IntegrityMismatch | Generated from contract source |
| 15 | RestoreNotApproved | Generated from contract source |
| 16 | AlreadyExecuted | Generated from contract source |
| 17 | DuplicateApproval | Generated from contract source |
| 18 | CostLimitExceeded | Generated from contract source |

### medical_record_search

| Code | Symbol | Description |
|------|--------|-------------|
| 1 | AlreadyInitialized | Generated from contract source |
| 2 | NotInitialized | Generated from contract source |
| 3 | NotAuthorized | Generated from contract source |
| 4 | ContractPaused | Generated from contract source |
| 5 | InvalidInput | Generated from contract source |
| 6 | RecordNotIndexed | Generated from contract source |
| 7 | QueryTooLarge | Generated from contract source |
| 8 | CacheMiss | Generated from contract source |

### mpc_manager

| Code | Symbol | Description |
|------|--------|-------------|
| 1 | AlreadyInitialized | Generated from contract source |
| 2 | NotInitialized | Generated from contract source |
| 3 | NotAuthorized | Generated from contract source |
| 4 | InvalidInput | Generated from contract source |
| 5 | SessionNotFound | Generated from contract source |
| 6 | SessionExpired | Generated from contract source |
| 7 | InvalidState | Generated from contract source |
| 8 | DuplicateCommit | Generated from contract source |
| 9 | DuplicateReveal | Generated from contract source |
| 10 | ThresholdNotMet | Generated from contract source |
| 11 | InvalidShare | Generated from contract source |
| 12 | ComputationFailed | Generated from contract source |
| 13 | ProofVerificationFailed | Generated from contract source |
| 14 | GasLimitExceeded | Generated from contract source |
| 15 | InsufficientParticipants | Generated from contract source |

### notification_system

| Code | Symbol | Description |
|------|--------|-------------|
| 100 | Unauthorized | Generated from contract source |
| 120 | SenderNotAuthorized | Generated from contract source |
| 208 | BatchTooLarge | Generated from contract source |
| 209 | RecipientsEmpty | Generated from contract source |
| 221 | TitleTooLong | Generated from contract source |
| 222 | MessageTooLong | Generated from contract source |
| 223 | NameTooLong | Generated from contract source |
| 224 | LocaleTooLong | Generated from contract source |
| 241 | InvalidNotifType | Generated from contract source |
| 242 | TooManyEnabledTypes | Generated from contract source |
| 300 | NotInitialized | Generated from contract source |
| 301 | AlreadyInitialized | Generated from contract source |
| 307 | RateLimitExceeded | Generated from contract source |
| 330 | AlreadyRead | Generated from contract source |
| 331 | AlreadyArchived | Generated from contract source |
| 450 | NotificationNotFound | Generated from contract source |
| 451 | AlertRuleNotFound | Generated from contract source |
| 452 | TemplateNotFound | Generated from contract source |
| 453 | SenderNotFound | Generated from contract source |
| 510 | MaxSendersReached | Generated from contract source |
| 511 | MaxRulesReached | Generated from contract source |
| 512 | MaxNotificationsReached | Generated from contract source |
| 513 | MaxTemplatesReached | Generated from contract source |

### patient_risk_stratification

| Code | Symbol | Description |
|------|--------|-------------|
| 1 | NotAuthorized | Generated from contract source |
| 2 | ConfigNotSet | Generated from contract source |
| 3 | ModelNotFound | Generated from contract source |
| 4 | InvalidScore | Generated from contract source |
| 5 | LowConfidence | Generated from contract source |
| 6 | AssessmentNotFound | Generated from contract source |
| 7 | InvalidModel | Generated from contract source |
| 8 | DuplicateModel | Generated from contract source |

### payment_router

| Code | Symbol | Description |
|------|--------|-------------|
| 1 | InvalidFeeBps | Generated from contract source |
| 2 | FeeNotSet | Generated from contract source |
| 3 | Overflow | Generated from contract source |
| 10 | InsufficientFunds | Generated from contract source |
| 11 | DeadlineExceeded | Generated from contract source |
| 12 | InvalidSignature | Generated from contract source |
| 13 | UnauthorizedCaller | Generated from contract source |
| 14 | ContractPaused | Generated from contract source |
| 15 | StorageFull | Generated from contract source |
| 16 | CrossChainTimeout | Generated from contract source |

### pharma_supply_chain

| Code | Symbol | Description |
|------|--------|-------------|
| 1 | AlreadyInitialized | Generated from contract source |
| 2 | NotInitialized | Generated from contract source |
| 3 | Unauthorized | Generated from contract source |
| 4 | ManufacturerNotFound | Generated from contract source |
| 5 | MedicationNotFound | Generated from contract source |
| 6 | BatchNotFound | Generated from contract source |
| 7 | ShipmentNotFound | Generated from contract source |
| 8 | InvalidInput | Generated from contract source |
| 9 | BatchAlreadyExists | Generated from contract source |

### predictive_analytics

| Code | Symbol | Description |
|------|--------|-------------|
| 1 | NotAuthorized | Generated from contract source |
| 2 | ConfigNotSet | Generated from contract source |
| 3 | Disabled | Generated from contract source |
| 4 | InvalidValue | Generated from contract source |
| 5 | InvalidConfidence | Generated from contract source |
| 6 | RecordNotFound | Generated from contract source |
| 7 | LowConfidence | Generated from contract source |
| 8 | InvalidHorizon | Generated from contract source |
| 9 | EmptyInput | Generated from contract source |

### provider_directory

| Code | Symbol | Description |
|------|--------|-------------|
| 1 | NotAuthorized | Generated from contract source |
| 1 | NotInitialized | Generated from contract source |
| 2 | AlreadyInitialized | Generated from contract source |
| 2 | AlreadyInitialized | Generated from contract source |
| 3 | NotInitialized | Generated from contract source |
| 3 | RateLimitExceeded | Generated from contract source |
| 4 | NotAuthorized | Generated from contract source |
| 4 | ProfileNotFound | Generated from contract source |
| 5 | ProfileAlreadyExists | Generated from contract source |
| 6 | InvalidSpecialty | Generated from contract source |
| 7 | InvalidAvailability | Generated from contract source |
| 8 | NotVerified | Generated from contract source |
| 9 | ContractPaused | Generated from contract source |
| 10 | InputTooLong | Generated from contract source |
| 11 | InvalidInput | Generated from contract source |

### public_health_surveillance

| Code | Symbol | Description |
|------|--------|-------------|
| 1 | AlreadyInitialized | Generated from contract source |
| 2 | NotInitialized | Generated from contract source |
| 3 | NotAuthorized | Generated from contract source |
| 4 | InvalidInput | Generated from contract source |
| 5 | DataNotFound | Generated from contract source |
| 6 | InvalidAggregationMethod | Generated from contract source |
| 7 | PrivacyBudgetExceeded | Generated from contract source |
| 8 | InsufficientPrivilege | Generated from contract source |
| 9 | InvalidSeverity | Generated from contract source |
| 10 | AlertExpired | Generated from contract source |
| 11 | ModelNotFound | Generated from contract source |
| 12 | InterventionNotFound | Generated from contract source |
| 13 | CollaborationNotFound | Generated from contract source |
| 14 | InvalidTimeRange | Generated from contract source |
| 15 | InvalidRegion | Generated from contract source |

### regulatory_compliance

| Code | Symbol | Description |
|------|--------|-------------|
| 1 | NotInitialized | Generated from contract source |
| 2 | AlreadyInitialized | Generated from contract source |
| 3 | UserAlreadyForgotten | Generated from contract source |
| 4 | RuleNotConfigured | Generated from contract source |
| 5 | RightToBeForgottenDisabled | Generated from contract source |

### reputation

| Code | Symbol | Description |
|------|--------|-------------|
| 1 | AlreadyInitialized | Generated from contract source |
| 2 | NotInitialized | Generated from contract source |
| 3 | NegativeAmount | Generated from contract source |
| 4 | InvalidAmount | Generated from contract source |

### reputation_access_control

| Code | Symbol | Description |
|------|--------|-------------|
| 1 | AlreadyInitialized | Generated from contract source |
| 2 | NotInitialized | Generated from contract source |
| 3 | NotAuthorized | Generated from contract source |
| 4 | InsufficientReputation | Generated from contract source |
| 5 | AccessDenied | Generated from contract source |
| 6 | InvalidResource | Generated from contract source |
| 7 | PolicyNotFound | Generated from contract source |
| 8 | ProviderNotVerified | Generated from contract source |
| 9 | CredentialExpired | Generated from contract source |

### reputation_integration

| Code | Symbol | Description |
|------|--------|-------------|
| 1 | AlreadyInitialized | Generated from contract source |
| 2 | NotInitialized | Generated from contract source |
| 3 | NotAuthorized | Generated from contract source |
| 4 | ProviderNotFound | Generated from contract source |
| 5 | ReputationContractNotFound | Generated from contract source |
| 6 | HealthcareReputationContractNotFound | Generated from contract source |
| 7 | InvalidScoreMapping | Generated from contract source |
| 8 | SyncFailed | Generated from contract source |

### runtime_validation

| Code | Symbol | Description |
|------|--------|-------------|
| 1 | NotInitialized | Generated from contract source |
| 2 | AlreadyInitialized | Generated from contract source |
| 3 | NotAuthorized | Generated from contract source |
| 4 | CheckNotFound | Generated from contract source |
| 5 | CheckAlreadyExists | Generated from contract source |
| 6 | CheckNotActive | Generated from contract source |
| 7 | InvalidSeverity | Generated from contract source |
| 8 | InvalidResourceLimit | Generated from contract source |
| 9 | ResourceLimitExceeded | Generated from contract source |
| 10 | ViolationNotFound | Generated from contract source |

### storage_cleanup

| Code | Symbol | Description |
|------|--------|-------------|
| 1 | NotInitialized | Generated from contract source |
| 2 | AlreadyInitialized | Generated from contract source |
| 3 | NotAuthorized | Generated from contract source |
| 4 | Paused | Generated from contract source |
| 5 | BatchTooLarge | Generated from contract source |

### sut_token

| Code | Symbol | Description |
|------|--------|-------------|
| 1 | AlreadyInitialized | Generated from contract source |
| 2 | NotInitialized | Generated from contract source |
| 3 | Unauthorized | Generated from contract source |
| 4 | InsufficientBalance | Generated from contract source |
| 5 | InsufficientAllowance | Generated from contract source |
| 6 | ExceedsSupplyCap | Generated from contract source |
| 7 | InvalidAmount | Generated from contract source |
| 8 | InvalidAddress | Generated from contract source |
| 9 | SnapshotNotFound | Generated from contract source |
| 10 | Overflow | Generated from contract source |
| 11 | IndexOutOfBounds | Generated from contract source |

### timelock

| Code | Symbol | Description |
|------|--------|-------------|
| 100 | Unauthorized | Generated from contract source |
| 207 | InvalidSignature | Generated from contract source |
| 300 | NotInitialized | Generated from contract source |
| 301 | AlreadyInitialized | Generated from contract source |
| 302 | ContractPaused | Generated from contract source |
| 306 | DeadlineExceeded | Generated from contract source |
| 372 | NotQueued | Generated from contract source |
| 375 | AlreadyQueued | Generated from contract source |
| 376 | NotReady | Generated from contract source |
| 500 | InsufficientFunds | Generated from contract source |
| 502 | StorageFull | Generated from contract source |
| 702 | CrossChainTimeout | Generated from contract source |

### token_sale

| Code | Symbol | Description |
|------|--------|-------------|
| 1 | AlreadyInitialized | Generated from contract source |
| 2 | InvalidArgument | Generated from contract source |
| 3 | Overflow | Generated from contract source |
| 4 | PhaseNotFound | Generated from contract source |
| 5 | PhaseClosed | Generated from contract source |
| 6 | CapExceeded | Generated from contract source |
| 7 | NotFinalized | Generated from contract source |
| 8 | AlreadyClaimed | Generated from contract source |
| 9 | RefundsNotEnabled | Generated from contract source |
| 10 | Paused | Generated from contract source |

### upgrade_manager

| Code | Symbol | Description |
|------|--------|-------------|
| 110 | NotAValidator | Generated from contract source |
| 120 | NotEnoughApprovals | Generated from contract source |
| 301 | AlreadyInitialized | Generated from contract source |
| 304 | InvalidState | Generated from contract source |
| 376 | TimelockNotExpired | Generated from contract source |
| 390 | ConfigNotFound | Generated from contract source |
| 450 | ProposalNotFound | Generated from contract source |
| 451 | AlreadyApproved | Generated from contract source |

### zk_verifier

| Code | Symbol | Description |
|------|--------|-------------|
| 100 | Unauthorized | Generated from contract source |
| 200 | InvalidInput | Generated from contract source |
| 300 | NotInitialized | Generated from contract source |
| 301 | AlreadyInitialized | Generated from contract source |
| 430 | VersionNotFound | Generated from contract source |
| 600 | InvalidProof | Generated from contract source |
| 601 | VerificationFailed | Generated from contract source |

### zkp_registry

| Code | Symbol | Description |
|------|--------|-------------|
| 1 | NotInitialized | Generated from contract source |
| 2 | AlreadyInitialized | Generated from contract source |
| 3 | NotFound | Generated from contract source |

