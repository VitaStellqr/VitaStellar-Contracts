# Error Codes Reference

> Comprehensive reference of all contract error codes across the VitaStellar Contracts ecosystem.
> Auto-generated from contract source. Do not edit manually.

## Per-Contract Error Codes

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

### contract_registry

| Code | Symbol | Description |
|------|--------|-------------|
| 1 | AlreadyInitialized | Generated from contract source |
| 2 | Unauthorized | Generated from contract source |
| 3 | NameNotFound | Generated from contract source |
| 100 | Unauthorized | Generated from contract source |
| 230 | MismatchedLength | Generated from contract source |
| 300 | NotInitialized | Generated from contract source |
| 301 | AlreadyInitialized | Generated from contract source |

### contract_template

| Code | Symbol | Description |
|------|--------|-------------|
| 1 | NotInitialized | Contract has not been initialized yet. |
| 2 | AlreadyInitialized | Contract has already been initialized. |
| 3 | Unauthorized | Caller is not authorized to perform this action. |
| 4 | InputTooLong | A string or bytes input exceeded the maximum allowed length. |
| 5 | ReentrantCall | Raised when `reentrancy::enter` returns `false` because the lock is already held — i.e. a guarded function was re-entered mid-call. |

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

### escrow

| Code | Symbol | Description |
|------|--------|-------------|
| 100 | Unauthorized | Generated from contract source |
| 102 | NotAdmin | Generated from contract source |
| 120 | InsufficientApprovals | Generated from contract source |
| 205 | InvalidAmount | Generated from contract source |
| 260 | InvalidFeeBps | Generated from contract source |
| 380 | FeeNotSet | Generated from contract source |
| 381 | ReentrancyRejected | Generated from contract source |
| 382 | InvalidStateTransition | Generated from contract source |
| 480 | EscrowExists | Generated from contract source |
| 481 | EscrowNotFound | Generated from contract source |
| 482 | AlreadySettled | Generated from contract source |
| 560 | NoBasisToRefund | Generated from contract source |
| 561 | NoCredit | Generated from contract source |
| 562 | Overflow | Generated from contract source |

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

### identity_registry

| Code | Symbol | Description |
|------|--------|-------------|
| 100 | Unauthorized | Generated from contract source |
| 110 | NotVerifier | Generated from contract source |
| 111 | CannotRemoveOwner | Generated from contract source |
| 120 | InvalidRecoveryGuardian | Generated from contract source |
| 121 | InsufficientGuardianApprovals | Generated from contract source |
| 122 | GuardianWeightTooHigh | Generated from contract source |
| 123 | InvalidRecoveryThreshold | Generated from contract source |
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
| 363 | RecoveryAlreadyExecuted | Generated from contract source |
| 450 | VerificationMethodNotFound | Generated from contract source |
| 460 | CredentialNotFound | Generated from contract source |
| 461 | AttestationNotFound | Generated from contract source |
| 462 | ServiceNotFound | Generated from contract source |
| 470 | DIDNotFound | Generated from contract source |
| 471 | DIDAlreadyExists | Generated from contract source |
| 472 | DIDDeactivated | Generated from contract source |
| 500 | ArithmeticOverflow | Generated from contract source |
| 603 | KeyRotationCooldown | Generated from contract source |
| 605 | CredentialExpired | Generated from contract source |
| 606 | CredentialRevoked | Generated from contract source |

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
| 377 | ReentrancyRejected | Generated from contract source |
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
| 11 | ReplayDetected | Generated from contract source |
| 500 | InsufficientFunds | Generated from contract source |

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

