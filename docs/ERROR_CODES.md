# Error Codes Reference

> Comprehensive reference of all contract error codes across the VitaStellar Contracts ecosystem.
> Auto-generated from contract source. Do not edit manually.

## Per-Contract Error Codes

### contract_template

| Code | Symbol | Description |
|------|--------|-------------|
| 1 | NotInitialized | Contract has not been initialized yet. |
| 2 | AlreadyInitialized | Contract has already been initialized. |
| 3 | Unauthorized | Caller is not authorized to perform this action. |
| 4 | InputTooLong | A string or bytes input exceeded the maximum allowed length. |
| 5 | ReentrantCall | Raised when `reentrancy::enter` returns `false` because the lock is already held — i.e. a guarded function was re-entered mid-call. |
| 6 | ContractPaused | Contract is paused by the system-wide PauseController. |

### pause_controller

| Code | Symbol | Description |
|------|--------|-------------|
| 1 | NotInitialized | Generated from contract source |
| 2 | AlreadyInitialized | Generated from contract source |
| 3 | Unauthorized | Generated from contract source |
| 4 | AlreadyRegistered | Generated from contract source |
| 5 | NotFound | Generated from contract source |
| 6 | AlreadyPaused | Generated from contract source |
| 7 | NotPaused | Generated from contract source |
| 8 | UnpauseNotScheduled | Generated from contract source |
| 9 | TimelockNotElapsed | Generated from contract source |
| 10 | ReentrantCall | Generated from contract source |

