# Medical Record Backup and Disaster Recovery

Contract: `contracts/medical_record_backup`

## Core capabilities
- Automated backup schedule state (`run_scheduled_backup`, `get_schedule`) plus manual execution (`run_backup_now`)
- Multi-target redundancy across networks and geo regions (`register_target`, `set_policy`)
- Integrity verification (`verify_backup_integrity`) and restore workflows (`request_restore`, `approve_restore`, `execute_restore`)
- **Direct record restore** (`restore_record`) — streamlined recovery path that bypasses the multi-step approval workflow for authorized recovery personnel
- Recovery drills (`run_recovery_test`)
- Cost-aware retention cleanup (`optimize_and_cleanup`)
- Monitoring and alerting (`list_alerts`, `report_target_failure`, `get_health`)
- Access controls via role masks (operator, auditor, recovery) and admin override

## Design notes
- Backup artifacts only store hashes and encrypted references (`snapshot_ref`) to avoid on-chain PHI leakage.
- Geo-resilience is enforced by `min_region_count` and `min_targets_per_backup`.
- Encryption enforcement is controlled by policy (`encryption_required` + non-zero key version).

---

## Restore procedures

### Direct restore (`restore_record`)

Authorized recovery personnel can restore a backup directly to a target medical
records contract using the `restore_record` function. This bypasses the multi-step
approval workflow for expedited recovery during disasters.

**Function signature:**
```rust
pub fn restore_record(
    env: Env,
    caller: Address,        // Must have ROLE_RECOVERY
    backup_id: u64,          // ID of the backup artifact to restore from
    target_contract: Address, // Medical records contract to restore to
    owner: Address,          // Owner of the restored record
    patient_id: String,      // Patient identifier
    record_type: String,     // Type of record (lab-result, imaging, etc.)
    content: String,         // Record content data
    timestamp: u64,          // Record timestamp
) -> Result<u64, Error>
```

**Workflow:**
1. Caller authorization is verified (must have `ROLE_RECOVERY`).
2. The backup artifact is retrieved and validated.
3. A cross-contract call is made to `target_contract.write_record(...)` to
   write the restored record data.
4. Regardless of the cross-contract call result, a `RestoredRecordInfo` entry
   is stored for audit trail.
5. A `record_restored` event is emitted containing the restore ID, backup ID,
   target contract, caller, and success status.
6. The artifact's `last_restored_at` timestamp is updated.

**Audit trail:**
- Each restore creates a `RestoredRecordInfo` entry accessible via
  `get_restored_record(restore_id)`.
- All restore IDs are tracked in the restored records list, viewable via
  `list_restored_records()`.
- A `record_restored` event is emitted for off-chain monitoring.
- If the cross-contract call fails, a `RestoreFailure` alert is created.

### Multi-step restore (legacy workflow)

The existing `request_restore` → `approve_restore` → `execute_restore` workflow
remains available for non-emergency restores requiring multi-signature approval.

---

## Restore error recovery

| Error | Cause | Resolution |
|-------|-------|------------|
| `BackupNotFound` | Invalid backup ID | Verify the backup ID with `list_artifacts` |
| `NotAuthorized` | Caller lacks `ROLE_RECOVERY` | Assign role via `assign_role` |
| Cross-contract failure | Target contract not deployer or wrong interface | Verify `target_contract` address and its `write_record` function |

---

## Disaster recovery plan

In the event of data loss on a primary medical records contract:

1. **Verify backup availability** — call `list_artifacts(true)` to verify backups exist.
2. **Select a backup** — choose an artifact by reviewing its `snapshot_ref`,
   `checksum`, and `created_at` fields via `get_artifact(artifact_id)`.
3. **Verify integrity** — call `verify_backup_integrity(artifact_id, checksum)`
   to confirm the backup is uncorrupted.
4. **Restore** — use `restore_record` with the target contract address and
   appropriate record data to restore the backup.
5. **Verify restoration** — check `get_restored_record(restore_id)` to confirm
   success, and query the target contract for the restored record.
6. **Audit** — the `record_restored` event and `RestoredRecordInfo` entry provide
   a complete audit trail of the recovery operation.
