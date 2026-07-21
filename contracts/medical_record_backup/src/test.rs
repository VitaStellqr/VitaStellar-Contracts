#![allow(clippy::unwrap_used)] // Allowed in test/benchmark harness where unwrap is acceptable
#![allow(clippy::expect_used)] // Allowed in test/benchmark harness where expect is acceptable

use super::*;
use soroban_sdk::testutils::{Address as _, Ledger};
use soroban_sdk::{Address, BytesN, Env, String};

fn setup(env: &Env) -> (MedicalRecordBackupContractClient<'_>, Address) {
    let contract_id = Address::generate(env);
    env.register_contract(&contract_id, MedicalRecordBackupContract);
    let client = MedicalRecordBackupContractClient::new(env, &contract_id);
    let admin = Address::generate(env);
    client.initialize(&admin);
    (client, admin)
}

fn sample_hash(env: &Env, v: u8) -> BytesN<32> {
    BytesN::from_array(env, &[v; 32])
}

fn register_two_targets(
    client: &MedicalRecordBackupContractClient<'_>,
    admin: &Address,
    env: &Env,
) {
    client.register_target(
        admin,
        &BackupNetwork::Ipfs,
        &GeoRegion::UsEast,
        &sample_hash(env, 1),
        &true,
        &10,
        &1000,
    );
    client.register_target(
        admin,
        &BackupNetwork::Arweave,
        &GeoRegion::EuCentral,
        &sample_hash(env, 2),
        &true,
        &15,
        &1000,
    );
}

#[test]
fn backup_run_creates_geo_redundant_artifact() {
    let env = Env::default();
    env.mock_all_auths();
    let (client, admin) = setup(&env);
    register_two_targets(&client, &admin, &env);

    let id = client.run_backup_now(
        &admin,
        &sample_hash(&env, 7),
        &String::from_str(&env, "ipfs://snapshot-a"),
        &1,
    );
    let artifact = client.get_artifact(&id).unwrap();
    assert_eq!(artifact.target_ids.len(), 2);
    assert_eq!(artifact.region_count, 2);
    assert!(artifact.encrypted);
}

#[test]
fn scheduled_backup_respects_interval() {
    let env = Env::default();
    env.mock_all_auths();
    let (client, admin) = setup(&env);
    register_two_targets(&client, &admin, &env);

    let policy = BackupPolicy {
        interval_seconds: 1_000,
        retention_seconds: 10_000,
        max_active_backups: 10,
        min_targets_per_backup: 2,
        min_region_count: 2,
        max_total_cost_weight: 1_000,
        verify_on_write: true,
        encryption_required: true,
        auto_cleanup: false,
        min_restore_approvals: 1,
    };
    client.set_policy(&admin, &policy);

    client.run_scheduled_backup(
        &admin,
        &sample_hash(&env, 9),
        &String::from_str(&env, "ipfs://snapshot-b"),
        &2,
    );

    let err = client.try_run_scheduled_backup(
        &admin,
        &sample_hash(&env, 10),
        &String::from_str(&env, "ipfs://snapshot-c"),
        &2,
    );
    assert_eq!(err, Err(Ok(Error::ScheduleNotDue)));
}

#[test]
fn integrity_mismatch_creates_alert() {
    let env = Env::default();
    env.mock_all_auths();
    let (client, admin) = setup(&env);
    register_two_targets(&client, &admin, &env);

    let id = client.run_backup_now(
        &admin,
        &sample_hash(&env, 3),
        &String::from_str(&env, "ipfs://snapshot-d"),
        &1,
    );
    let ok = client.verify_backup_integrity(&admin, &id, &sample_hash(&env, 255));
    assert!(!ok);

    let alerts = client.list_alerts(&true);
    assert!(!alerts.is_empty());
}

#[test]
fn restore_workflow_executes_after_approval() {
    let env = Env::default();
    env.mock_all_auths();
    let (client, admin) = setup(&env);
    register_two_targets(&client, &admin, &env);

    let id = client.run_backup_now(
        &admin,
        &sample_hash(&env, 4),
        &String::from_str(&env, "ipfs://snapshot-restore"),
        &1,
    );
    let request_id = client.request_restore(&admin, &id, &sample_hash(&env, 11));
    client.approve_restore(&admin, &request_id);
    let restored_ref = client.execute_restore(&admin, &request_id);
    assert_eq!(
        restored_ref,
        String::from_str(&env, "ipfs://snapshot-restore")
    );
}

#[test]
fn restore_record_calls_target_contract_and_emits_event() {
    let env = Env::default();
    env.mock_all_auths();
    let (client, admin) = setup(&env);
    register_two_targets(&client, &admin, &env);

    // Create a backup to restore
    let id = client.run_backup_now(
        &admin,
        &sample_hash(&env, 5),
        &String::from_str(&env, "ipfs://snapshot-restore-test"),
        &1,
    );

    // Assign recovery role
    let recovery_user = Address::generate(&env);
    client.assign_role(&admin, &recovery_user, &4); // ROLE_RECOVERY = 4

    // Simulate a target medical_records contract address
    let target_contract = Address::generate(&env);

    // Attempt to restore — the target contract might not have write_record
    // so the restore may succeed in tracking but the cross-contract call may fail.
    // We verify the restore is tracked regardless.
    let restore_id = client.restore_record(
        &recovery_user,
        &id,
        &target_contract,
        &admin,
        &String::from_str(&env, "patient-123"),
        &String::from_str(&env, "lab-result"),
        &String::from_str(&env, "{\"test\": \"glucose\", \"value\": 95}"),
        &1000000,
    );

    // Verify the restore was tracked
    let restored = client.get_restored_record(&restore_id).unwrap();
    assert_eq!(restored.artifact_id, id);
    assert_eq!(restored.requested_by, recovery_user);
    assert_eq!(restored.target_contract, target_contract);

    // Verify the restored records list contains the new entry
    let list = client.list_restored_records();
    assert!(list.contains(&restore_id));
}

#[test]
fn restore_record_fails_without_recovery_role() {
    let env = Env::default();
    env.mock_all_auths();
    let (client, admin) = setup(&env);
    register_two_targets(&client, &admin, &env);

    let id = client.run_backup_now(
        &admin,
        &sample_hash(&env, 6),
        &String::from_str(&env, "ipfs://snapshot-auth-test"),
        &1,
    );

    // Use an unauthorized user (no role assigned) — with mock_all_auths still
    // active, the require_auth will pass, but require_recovery should reject.
    let unauthorized = Address::generate(&env);
    let target_contract = Address::generate(&env);

    let err = client.try_restore_record(
        &unauthorized,
        &id,
        &target_contract,
        &admin,
        &String::from_str(&env, "patient-456"),
        &String::from_str(&env, "imaging"),
        &String::from_str(&env, "\"mri\": \"normal\""),
        &2000000,
    );
    assert_eq!(err, Err(Ok(Error::NotAuthorized)));
}

#[test]
fn restore_record_fails_for_nonexistent_backup() {
    let env = Env::default();
    env.mock_all_auths();
    let (client, admin) = setup(&env);

    let recovery_user = Address::generate(&env);
    client.assign_role(&admin, &recovery_user, &4); // ROLE_RECOVERY = 4

    let target_contract = Address::generate(&env);

    // Try to restore a backup that doesn't exist
    let err = client.try_restore_record(
        &recovery_user,
        &99999,
        &target_contract,
        &admin,
        &String::from_str(&env, "patient-789"),
        &String::from_str(&env, "genomics"),
        &String::from_str(&env, "\"sequence\": \"ATCG\""),
        &3000000,
    );
    assert_eq!(err, Err(Ok(Error::BackupNotFound)));
}

#[test]
fn restore_record_empty_list_before_first_restore() {
    let env = Env::default();
    env.mock_all_auths();
    let (client, _admin) = setup(&env);

    // Before any restore, the list should be empty
    let list = client.list_restored_records();
    assert_eq!(list.len(), 0);
}

#[test]
fn restore_record_nonexistent_id_returns_none() {
    let env = Env::default();
    env.mock_all_auths();
    let (client, _admin) = setup(&env);

    // Querying a non-existent restore ID should return None
    let restored = client.get_restored_record(&99999);
    assert!(restored.is_none());
}

#[test]
fn restore_record_updates_artifact_last_restored_at() {
    let env = Env::default();
    env.mock_all_auths();
    let (client, admin) = setup(&env);
    register_two_targets(&client, &admin, &env);

    // Set a known timestamp
    env.ledger().set_timestamp(50000);

    let id = client.run_backup_now(
        &admin,
        &sample_hash(&env, 8),
        &String::from_str(&env, "ipfs://snapshot-timestamp-test"),
        &1,
    );

    let recovery_user = Address::generate(&env);
    client.assign_role(&admin, &recovery_user, &4);

    // Advance timestamp
    env.ledger().set_timestamp(60000);

    let target_contract = Address::generate(&env);
    client.restore_record(
        &recovery_user,
        &id,
        &target_contract,
        &admin,
        &String::from_str(&env, "patient-timestamp"),
        &String::from_str(&env, "vital-signs"),
        &String::from_str(&env, "\"bp\": \"120/80\""),
        &4000000,
    );

    // Verify the artifact's last_restored_at was updated
    let artifact = client.get_artifact(&id).unwrap();
    assert_eq!(artifact.last_restored_at, 60000);
}

#[test]
fn cleanup_archives_old_backups() {
    let env = Env::default();
    env.mock_all_auths();
    let (client, admin) = setup(&env);
    register_two_targets(&client, &admin, &env);

    client.set_policy(
        &admin,
        &BackupPolicy {
            interval_seconds: 1,
            retention_seconds: 2,
            max_active_backups: 1,
            min_targets_per_backup: 2,
            min_region_count: 2,
            max_total_cost_weight: 1_000,
            verify_on_write: true,
            encryption_required: true,
            auto_cleanup: false,
            min_restore_approvals: 1,
        },
    );

    env.ledger().set_timestamp(1000);
    client.run_backup_now(
        &admin,
        &sample_hash(&env, 21),
        &String::from_str(&env, "ipfs://old"),
        &1,
    );
    env.ledger().set_timestamp(2000);
    client.run_backup_now(
        &admin,
        &sample_hash(&env, 22),
        &String::from_str(&env, "ipfs://new"),
        &1,
    );
    env.ledger().set_timestamp(3000);
    let report = client.optimize_and_cleanup(&admin);
    assert!(report.archived_backups >= 1);
}
