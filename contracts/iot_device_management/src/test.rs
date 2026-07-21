use super::*;
use ed25519_dalek::{Signer, SigningKey};
use rand::rngs::OsRng;
use soroban_sdk::testutils::{Address as _, Ledger};
use soroban_sdk::{Address, BytesN, Env, String};

// =====================================================================
// Issue #194 test scaffolding: full iot + crypto_registry test fixture
// =====================================================================

/// Deploy + initialize a matching `crypto_registry` contract and return its
/// SDK-generated client.
fn deploy_crypto_registry(env: &Env) -> crypto_registry::CryptoRegistryClient<'_> {
    let contract_id = Address::generate(env);
    env.register_contract(&contract_id, crypto_registry::CryptoRegistry);
    crypto_registry::CryptoRegistryClient::new(env, &contract_id)
}

/// Build a `PublicKey` whose algorithm is `Custom(0)` and whose `Bytes` is
/// empty — the placeholder we use for `pq_encryption_key` when the
/// manufacturer does not advertise a post-quantum key.
fn dummy_public_key(env: &Env) -> crypto_registry::PublicKey {
    crypto_registry::PublicKey {
        algorithm: crypto_registry::KeyAlgorithm::Custom(0),
        key: soroban_sdk::Bytes::new(env),
    }
}

fn setup(env: &Env) -> (IoTDeviceManagementClient<'_>, Address) {
    let contract_id = Address::generate(env);
    env.register_contract(&contract_id, IoTDeviceManagement);
    let client = IoTDeviceManagementClient::new(env, &contract_id);
    let admin = Address::generate(env);
    env.mock_all_auths();
    (client, admin)
}

/// Build a deterministic 32-byte `BytesN<32>` padded to the right.
fn make_bytes32(env: &Env, val: u8) -> BytesN<32> {
    let mut bytes = [0u8; 32];
    bytes[0] = val;
    BytesN::from_array(env, &bytes)
}

/// Generate a fresh Ed25519 keypair and return `(signing_key, pubkey_bytes)`.
fn make_ed25519_keypair() -> SigningKey {
    SigningKey::generate(&mut OsRng)
}

/// Sign the `DOMAIN_PREFIX || device_id || firmware_hash` payload exactly
/// as the contract does. The domain prefix is a 15-byte ASCII string
/// (`"iot_fw_sig_v1__"`), so the total payload length is 15 + 32 + 32 = 79.
fn sign_firmware_payload(
    signing_key: &SigningKey,
    device_id: &BytesN<32>,
    firmware_hash: &BytesN<32>,
) -> [u8; 64] {
    let mut msg = [0u8; 79];
    msg[..15].copy_from_slice(b"iot_fw_sig_v1__");
    msg[15..47].copy_from_slice(device_id.to_array().as_slice());
    msg[47..79].copy_from_slice(firmware_hash.to_array().as_slice());
    signing_key.sign(&msg).to_bytes()
}

/// Build a synthetic 32-byte firmware hash for tests.
fn make_firmware_hash(env: &Env, val: u8) -> BytesN<32> {
    let mut bytes = [0u8; 32];
    bytes[31] = val; // distinct from device_id byte[0]
    BytesN::from_array(env, &bytes)
}

/// Bundle a freshly-generated keypair into the cross-contract inputs the
/// `register_device` entrypoint now expects.
fn make_info(env: &Env, device_byte: u8) -> DeviceRegistrationInfo {
    DeviceRegistrationInfo {
        model: String::from_str(env, "Model-X100"),
        serial_number: String::from_str(env, "SN-00001"),
        location: String::from_str(env, "Ward A, Room 101"),
        encryption_key_hash: make_bytes32(env, device_byte.wrapping_add(50)),
        metadata_ref: String::from_str(env, "ipfs://Qm..."),
    }
}

fn make_attestation(
    env: &Env,
    device_byte: u8,
    mfr_signing_key: &SigningKey,
) -> FirmwareAttestation {
    let device_id = make_bytes32(env, device_byte);
    let firmware_hash = make_firmware_hash(env, device_byte);
    let sig_bytes = sign_firmware_payload(mfr_signing_key, &device_id, &firmware_hash);
    FirmwareAttestation {
        firmware_hash,
        firmware_signature: BytesN::<64>::from_array(env, &sig_bytes),
    }
}

fn register_manufacturer(
    env: &Env,
    client: &IoTDeviceManagementClient<'_>,
    admin: &Address,
    id_byte: u8,
) -> BytesN<32> {
    let mfr_id = make_bytes32(env, id_byte);
    let cert = make_bytes32(env, id_byte.wrapping_add(100));
    let name = String::from_str(env, "TestManufacturer");
    client.register_manufacturer(admin, &mfr_id, &name, &cert);
    mfr_id
}

fn setup_with_crypto(
    env: &Env,
) -> (
    IoTDeviceManagementClient<'_>,
    crypto_registry::CryptoRegistryClient<'_>,
    Address,
    SigningKey,
) {
    let (iot_client, admin) = setup(env);
    iot_client.initialize(&admin);

    let crypto_client = deploy_crypto_registry(env);
    crypto_client.initialize(&admin);
    iot_client.set_crypto_registry_contract(&admin, &crypto_client.address);
    let signing_key = make_ed25519_keypair();
    let pubkey_bytes = signing_key.verifying_key().to_bytes();

    let encryption_key = crypto_registry::PublicKey {
        algorithm: crypto_registry::KeyAlgorithm::X25519,
        key: soroban_sdk::Bytes::from_array(env, &pubkey_bytes),
    };
    let signing_pub = crypto_registry::PublicKey {
        algorithm: crypto_registry::KeyAlgorithm::Ed25519,
        key: soroban_sdk::Bytes::from_array(env, &pubkey_bytes),
    };
    crypto_client.register_key_bundle(
        &admin,
        &encryption_key,
        &dummy_public_key(env),
        &false,
        &signing_pub,
        &true,
    );
    (iot_client, crypto_client, admin, signing_key)
}

/// Register a manufacturer in iot, register a dedicated Ed25519 signing key
/// in crypto_registry under the `admin` address (which doubles as the
/// manufacturer's crypto_owner), and link the manufacturer to that
/// crypto_owner. Returns the `(manufacturer_id, manufacturer_signing_key)`.
///
/// Note: `crypto_registry.rotate_key` does not exist as a public entrypoint;
/// calling `register_key_bundle` for the same owner simply bumps the version
/// pointer under persistent storage, which is what we want for "rotation".
fn register_manufacturer_with_crypto(
    env: &Env,
    iot_client: &IoTDeviceManagementClient<'_>,
    crypto_client: &crypto_registry::CryptoRegistryClient<'_>,
    admin: &Address,
    id_byte: u8,
) -> (BytesN<32>, SigningKey) {
    let mfr_id = make_bytes32(env, id_byte);
    let cert = make_bytes32(env, id_byte.wrapping_add(100));
    let name = String::from_str(env, "TestManufacturer");
    iot_client.register_manufacturer(admin, &mfr_id, &name, &cert);

    let mfr_signing_key = make_ed25519_keypair();
    let pubkey_bytes = mfr_signing_key.verifying_key().to_bytes();
    let encryption_key = crypto_registry::PublicKey {
        algorithm: crypto_registry::KeyAlgorithm::X25519,
        key: soroban_sdk::Bytes::from_array(env, &pubkey_bytes),
    };
    let signing_pub = crypto_registry::PublicKey {
        algorithm: crypto_registry::KeyAlgorithm::Ed25519,
        key: soroban_sdk::Bytes::from_array(env, &pubkey_bytes),
    };

    // Re-register the admin's bundle with the manufacturer-specific signing
    // key. `get_current_key_bundle` will return this latest version.
    crypto_client.register_key_bundle(
        admin,
        &encryption_key,
        &dummy_public_key(env),
        &false,
        &signing_pub,
        &true,
    );

    iot_client.set_manufacturer_crypto_owner(admin, &mfr_id, admin);

    (mfr_id, mfr_signing_key)
}

/// Helper to register a device with valid firmware signature using the
/// freshly-generated keypair from `setup_with_crypto`/`register_manufacturer_with_crypto`.
fn register_device(
    env: &Env,
    client: &IoTDeviceManagementClient<'_>,
    operator: &Address,
    mfr_id: &BytesN<32>,
    device_byte: u8,
    mfr_signing_key: &SigningKey,
) -> BytesN<32> {
    let device_id = make_bytes32(env, device_byte);
    let info = make_info(env, device_byte);
    let firmware = make_attestation(env, device_byte, mfr_signing_key);

    client.register_device(
        operator,
        &device_id,
        mfr_id,
        &DeviceType::VitalSignsMonitor,
        &info,
        &firmware,
    );
    device_id
}

#[test]
fn test_initialize() {
    let env = Env::default();
    let (client, admin) = setup(&env);
    client.initialize(&admin);
    let result = client.try_initialize(&admin);
    assert_eq!(result, Err(Ok(Error::AlreadyInitialized)));
}

#[test]
fn test_pause_unpause() {
    let env = Env::default();
    let (client, admin) = setup(&env);
    client.initialize(&admin);
    client.pause(&admin);
    let user = Address::generate(&env);
    let result = client.try_set_role(&admin, &user, &Role::Operator);
    assert_eq!(result, Err(Ok(Error::ContractPaused)));
    client.unpause(&admin);
    client.set_role(&admin, &user, &Role::Operator);
}

#[test]
fn test_pause_not_admin() {
    let env = Env::default();
    let (client, admin) = setup(&env);
    client.initialize(&admin);
    let non_admin = Address::generate(&env);
    let result = client.try_pause(&non_admin);
    assert_eq!(result, Err(Ok(Error::NotAdmin)));
}

#[test]
fn test_set_role() {
    let env = Env::default();
    let (client, admin) = setup(&env);
    client.initialize(&admin);
    let user = Address::generate(&env);
    client.set_role(&admin, &user, &Role::Operator);
    let role = client.get_role(&user);
    assert_eq!(role, Role::Operator);
}

#[test]
fn test_register_manufacturer() {
    let env = Env::default();
    let (client, admin) = setup(&env);
    client.initialize(&admin);
    let mfr_id = register_manufacturer(&env, &client, &admin, 1);
    let mfr = client.get_manufacturer(&mfr_id);
    assert!(mfr.is_active);
    assert_eq!(mfr.device_count, 0);
}

#[test]
fn test_register_manufacturer_duplicate() {
    let env = Env::default();
    let (client, admin) = setup(&env);
    client.initialize(&admin);
    let mfr_id = register_manufacturer(&env, &client, &admin, 1);
    let cert = make_bytes32(&env, 200);
    let name = String::from_str(&env, "Dup");
    let result = client.try_register_manufacturer(&admin, &mfr_id, &name, &cert);
    assert_eq!(result, Err(Ok(Error::ManufacturerAlreadyRegistered)));
}

#[test]
fn test_deactivate_manufacturer() {
    let env = Env::default();
    let (client, admin) = setup(&env);
    client.initialize(&admin);
    let mfr_id = register_manufacturer(&env, &client, &admin, 1);
    client.deactivate_manufacturer(&admin, &mfr_id);
    let mfr = client.get_manufacturer(&mfr_id);
    assert!(!mfr.is_active);
}

#[test]
fn test_register_device() {
    let env = Env::default();
    let (iot_client, crypto_client, admin, _admin_key) = setup_with_crypto(&env);
    let operator = Address::generate(&env);
    iot_client.set_role(&admin, &operator, &Role::Operator);
    let (mfr_id, mfr_key) =
        register_manufacturer_with_crypto(&env, &iot_client, &crypto_client, &admin, 1);

    let device_id = register_device(&env, &iot_client, &operator, &mfr_id, 10, &mfr_key);
    let device = iot_client.get_device(&device_id);
    assert_eq!(device.status, DeviceStatus::Provisioning);
    assert_eq!(device.device_type, DeviceType::VitalSignsMonitor);
    assert_eq!(device.operator, operator);
}

#[test]
fn test_register_device_requires_crypto_registry_configured() {
    let env = Env::default();
    let (iot_client, admin) = setup(&env);
    iot_client.initialize(&admin);
    let operator = Address::generate(&env);
    iot_client.set_role(&admin, &operator, &Role::Operator);
    let mfr_id = register_manufacturer(&env, &iot_client, &admin, 1);

    let device_id = make_bytes32(&env, 10);
    let info = DeviceRegistrationInfo {
        model: String::from_str(&env, "M"),
        serial_number: String::from_str(&env, "S"),
        location: String::from_str(&env, "L"),
        encryption_key_hash: make_bytes32(&env, 99),
        metadata_ref: String::from_str(&env, "x"),
    };
    let firmware = FirmwareAttestation {
        firmware_hash: make_firmware_hash(&env, 10),
        firmware_signature: BytesN::<64>::from_array(&env, &[0u8; 64]),
    };

    let result = iot_client.try_register_device(
        &operator,
        &device_id,
        &mfr_id,
        &DeviceType::VitalSignsMonitor,
        &info,
        &firmware,
    );
    assert_eq!(result, Err(Ok(Error::CryptoRegistryNotConfigured)));
}

#[test]
fn test_register_device_requires_manufacturer_crypto_owner() {
    let env = Env::default();
    let (iot_client, _crypto_client, admin, _key) = setup_with_crypto(&env);
    let operator = Address::generate(&env);
    iot_client.set_role(&admin, &operator, &Role::Operator);
    // Do NOT link manufacturer crypto owner.
    let mfr_id = register_manufacturer(&env, &iot_client, &admin, 1);

    let device_id = make_bytes32(&env, 10);
    let info = DeviceRegistrationInfo {
        model: String::from_str(&env, "M"),
        serial_number: String::from_str(&env, "S"),
        location: String::from_str(&env, "L"),
        encryption_key_hash: make_bytes32(&env, 99),
        metadata_ref: String::from_str(&env, "x"),
    };
    let firmware = FirmwareAttestation {
        firmware_hash: make_firmware_hash(&env, 10),
        firmware_signature: BytesN::<64>::from_array(&env, &[0u8; 64]),
    };

    let result = iot_client.try_register_device(
        &operator,
        &device_id,
        &mfr_id,
        &DeviceType::VitalSignsMonitor,
        &info,
        &firmware,
    );
    assert_eq!(result, Err(Ok(Error::ManufacturerCryptoOwnerNotSet)));
}

#[test]
fn test_register_device_rejects_invalid_signature() {
    let env = Env::default();
    let (iot_client, crypto_client, admin, _key) = setup_with_crypto(&env);
    let operator = Address::generate(&env);
    iot_client.set_role(&admin, &operator, &Role::Operator);
    let (mfr_id, _mfr_key) =
        register_manufacturer_with_crypto(&env, &iot_client, &crypto_client, &admin, 1);

    // Sign with a key the contract does NOT trust.
    let attacker_key = make_ed25519_keypair();
    let device_id = make_bytes32(&env, 10);
    let firmware_hash = make_firmware_hash(&env, 10);
    let sig_bytes = sign_firmware_payload(&attacker_key, &device_id, &firmware_hash);
    let info = DeviceRegistrationInfo {
        model: String::from_str(&env, "M"),
        serial_number: String::from_str(&env, "S"),
        location: String::from_str(&env, "L"),
        encryption_key_hash: make_bytes32(&env, 99),
        metadata_ref: String::from_str(&env, "x"),
    };
    let firmware = FirmwareAttestation {
        firmware_hash,
        firmware_signature: BytesN::<64>::from_array(&env, &sig_bytes),
    };

    let result = iot_client.try_register_device(
        &operator,
        &device_id,
        &mfr_id,
        &DeviceType::VitalSignsMonitor,
        &info,
        &firmware,
    );
    // `env.crypto().ed25519_verify` panics on a failed verification — the
    // SDK 21.7.7 host returns `Err(Err(ConversionError))` (host-level abort)
    // rather than a contract-level `Err(Ok(InvalidFirmwareSignature))`.
    // We can't surface `InvalidFirmwareSignature` as a contract error
    // without re-implementing ed25519 inside the contract.
    assert!(
        matches!(result, Err(Err(_))),
        "expected host abort from bad ed25519 sig, got {:?}",
        result
    );
}

#[test]
fn test_register_device_rejects_missing_key_bundle() {
    let env = Env::default();
    let (iot_client, crypto_client, admin, _key) = setup_with_crypto(&env);
    let operator = Address::generate(&env);
    iot_client.set_role(&admin, &operator, &Role::Operator);

    // Wiring is in place: iot points at the deployed crypto_registry.
    assert!(iot_client.get_crypto_registry_contract().is_some());
    let _ = crypto_client.address;

    // Manufacturer registered but link points to an address with NO key
    // bundle in crypto_registry.
    let empty_owner = Address::generate(&env);
    let mfr_id = make_bytes32(&env, 1);
    let cert = make_bytes32(&env, 101);
    let name = String::from_str(&env, "NoOwner");
    iot_client.register_manufacturer(&admin, &mfr_id, &name, &cert);
    iot_client.set_manufacturer_crypto_owner(&admin, &mfr_id, &empty_owner);

    let device_id = make_bytes32(&env, 10);
    let info = DeviceRegistrationInfo {
        model: String::from_str(&env, "M"),
        serial_number: String::from_str(&env, "S"),
        location: String::from_str(&env, "L"),
        encryption_key_hash: make_bytes32(&env, 99),
        metadata_ref: String::from_str(&env, "x"),
    };
    let firmware = FirmwareAttestation {
        firmware_hash: make_firmware_hash(&env, 10),
        firmware_signature: BytesN::<64>::from_array(&env, &[0u8; 64]),
    };

    let result = iot_client.try_register_device(
        &operator,
        &device_id,
        &mfr_id,
        &DeviceType::VitalSignsMonitor,
        &info,
        &firmware,
    );
    assert_eq!(result, Err(Ok(Error::CryptoKeyBundleNotFound)));
}

#[test]
fn test_register_device_duplicate() {
    let env = Env::default();
    let (iot_client, crypto_client, admin, _key) = setup_with_crypto(&env);
    let operator = Address::generate(&env);
    iot_client.set_role(&admin, &operator, &Role::Operator);
    let (mfr_id, mfr_key) =
        register_manufacturer_with_crypto(&env, &iot_client, &crypto_client, &admin, 1);
    let _device_id = register_device(&env, &iot_client, &operator, &mfr_id, 10, &mfr_key);

    let device_id_dup = make_bytes32(&env, 10);
    let info = DeviceRegistrationInfo {
        model: String::from_str(&env, "M"),
        serial_number: String::from_str(&env, "S"),
        location: String::from_str(&env, "L"),
        encryption_key_hash: make_bytes32(&env, 99),
        metadata_ref: String::from_str(&env, "x"),
    };
    let firmware = make_attestation(&env, 10, &mfr_key);

    let result = iot_client.try_register_device(
        &operator,
        &device_id_dup,
        &mfr_id,
        &DeviceType::VitalSignsMonitor,
        &info,
        &firmware,
    );
    assert_eq!(result, Err(Ok(Error::DeviceAlreadyRegistered)));
}

#[test]
fn test_set_crypto_registry_contract_admin_only() {
    let env = Env::default();
    let (iot_client, _crypto_client, admin, _key) = setup_with_crypto(&env);
    let non_admin = Address::generate(&env);
    let arbitrary = Address::generate(&env);
    let result = iot_client.try_set_crypto_registry_contract(&non_admin, &arbitrary);
    assert_eq!(result, Err(Ok(Error::NotAdmin)));
    // Admin succeeds.
    iot_client.set_crypto_registry_contract(&admin, &arbitrary);
    assert_eq!(iot_client.get_crypto_registry_contract(), Some(arbitrary));
}

#[test]
fn test_set_manufacturer_crypto_owner_admin_only() {
    let env = Env::default();
    let (iot_client, _crypto_client, admin, _key) = setup_with_crypto(&env);
    let mfr_id = register_manufacturer(&env, &iot_client, &admin, 1);
    let non_admin = Address::generate(&env);
    let crypto_owner = Address::generate(&env);
    let result = iot_client.try_set_manufacturer_crypto_owner(&non_admin, &mfr_id, &crypto_owner);
    assert_eq!(result, Err(Ok(Error::NotAdmin)));
}

#[test]
fn test_manufacturer_crypto_owner_rotation_uses_new_key() {
    let env = Env::default();
    let (iot_client, crypto_client, admin, _key) = setup_with_crypto(&env);
    let operator = Address::generate(&env);
    iot_client.set_role(&admin, &operator, &Role::Operator);

    // Manufacturer registered against the initial admin key bundle.
    let (mfr_id, mfr_key_a) =
        register_manufacturer_with_crypto(&env, &iot_client, &crypto_client, &admin, 1);

    let device_id_a = make_bytes32(&env, 10);
    let fw_a_hash = make_firmware_hash(&env, 10);
    let sig_a = sign_firmware_payload(&mfr_key_a, &device_id_a, &fw_a_hash);
    let info = make_info(&env, 10);
    let firmware_a = FirmwareAttestation {
        firmware_hash: fw_a_hash.clone(),
        firmware_signature: BytesN::<64>::from_array(&env, &sig_a),
    };

    // First registration succeeds using the initial mfr signing key.
    iot_client.register_device(
        &operator,
        &device_id_a,
        &mfr_id,
        &DeviceType::VitalSignsMonitor,
        &info,
        &firmware_a,
    );

    // Publish a fresh bundle under a brand-new owner address.
    let new_owner = Address::generate(&env);
    let new_owner_key = make_ed25519_keypair();
    let new_owner_pub = new_owner_key.verifying_key().to_bytes();
    let new_owner_enc = crypto_registry::PublicKey {
        algorithm: crypto_registry::KeyAlgorithm::X25519,
        key: soroban_sdk::Bytes::from_array(&env, &new_owner_pub),
    };
    let new_owner_signing_pub = crypto_registry::PublicKey {
        algorithm: crypto_registry::KeyAlgorithm::Ed25519,
        key: soroban_sdk::Bytes::from_array(&env, &new_owner_pub),
    };
    crypto_client.register_key_bundle(
        &new_owner,
        &new_owner_enc,
        &dummy_public_key(&env),
        &false,
        &new_owner_signing_pub,
        &true,
    );

    iot_client.set_manufacturer_crypto_owner(&admin, &mfr_id, &new_owner);

    // The OLD signature must now fail (key bundle is the new owner's).
    let device_id_b = make_bytes32(&env, 11);
    let fw_b_hash = make_firmware_hash(&env, 11);
    let stale_sig = sign_firmware_payload(&mfr_key_a, &device_id_b, &fw_b_hash);
    let info_b = make_info(&env, 11);
    let stale_firmware = FirmwareAttestation {
        firmware_hash: fw_b_hash.clone(),
        firmware_signature: BytesN::<64>::from_array(&env, &stale_sig),
    };
    let result_stale = iot_client.try_register_device(
        &operator,
        &device_id_b,
        &mfr_id,
        &DeviceType::VitalSignsMonitor,
        &info_b,
        &stale_firmware,
    );
    // Same host-abort-on-bad-sig constraint as above — the stale signature
    // was signed under the previous manufacturer key, so the contract's
    // `ed25519_verify` panics at the host.
    assert!(
        matches!(result_stale, Err(Err(_))),
        "expected host abort from rotated-manufacturer ed25519 sig, got {:?}",
        result_stale
    );

    // The NEW signature (signed with the new owner's key) must succeed.
    let fresh_sig = sign_firmware_payload(&new_owner_key, &device_id_b, &fw_b_hash);
    let fresh_firmware = FirmwareAttestation {
        firmware_hash: fw_b_hash,
        firmware_signature: BytesN::<64>::from_array(&env, &fresh_sig),
    };
    iot_client.register_device(
        &operator,
        &device_id_b,
        &mfr_id,
        &DeviceType::VitalSignsMonitor,
        &info_b,
        &fresh_firmware,
    );
}

#[test]
fn test_firmware_hash_stored_on_device() {
    let env = Env::default();
    let (iot_client, crypto_client, admin, _key) = setup_with_crypto(&env);
    let operator = Address::generate(&env);
    iot_client.set_role(&admin, &operator, &Role::Operator);
    let (mfr_id, mfr_key) =
        register_manufacturer_with_crypto(&env, &iot_client, &crypto_client, &admin, 1);
    let device_id = register_device(&env, &iot_client, &operator, &mfr_id, 42, &mfr_key);
    let device = iot_client.get_device(&device_id);
    assert_eq!(device.firmware_hash, make_firmware_hash(&env, 42));
}

#[test]
fn test_activate_device() {
    let env = Env::default();
    let (iot_client, crypto_client, admin, _key) = setup_with_crypto(&env);
    let operator = Address::generate(&env);
    iot_client.set_role(&admin, &operator, &Role::Operator);
    let (mfr_id, mfr_key) =
        register_manufacturer_with_crypto(&env, &iot_client, &crypto_client, &admin, 1);
    let device_id = register_device(&env, &iot_client, &operator, &mfr_id, 10, &mfr_key);

    iot_client.activate_device(&operator, &device_id);
    let device = iot_client.get_device(&device_id);
    assert_eq!(device.status, DeviceStatus::Active);
}

#[test]
fn test_suspend_and_reactivate_device() {
    let env = Env::default();
    let (iot_client, crypto_client, admin, _key) = setup_with_crypto(&env);
    let operator = Address::generate(&env);
    iot_client.set_role(&admin, &operator, &Role::Operator);
    let (mfr_id, mfr_key) =
        register_manufacturer_with_crypto(&env, &iot_client, &crypto_client, &admin, 1);
    let device_id = register_device(&env, &iot_client, &operator, &mfr_id, 10, &mfr_key);
    iot_client.activate_device(&operator, &device_id);

    iot_client.suspend_device(&operator, &device_id);
    let device = iot_client.get_device(&device_id);
    assert_eq!(device.status, DeviceStatus::Suspended);

    iot_client.activate_device(&operator, &device_id);
    let device = iot_client.get_device(&device_id);
    assert_eq!(device.status, DeviceStatus::Active);
}

#[test]
fn test_decommission_device() {
    let env = Env::default();
    let (iot_client, crypto_client, admin, _key) = setup_with_crypto(&env);
    let operator = Address::generate(&env);
    iot_client.set_role(&admin, &operator, &Role::Operator);
    let (mfr_id, mfr_key) =
        register_manufacturer_with_crypto(&env, &iot_client, &crypto_client, &admin, 1);
    let device_id = register_device(&env, &iot_client, &operator, &mfr_id, 10, &mfr_key);

    iot_client.decommission_device(&admin, &device_id);
    let device = iot_client.get_device(&device_id);
    assert_eq!(device.status, DeviceStatus::Decommissioned);
}

#[test]
fn test_get_device_count() {
    let env = Env::default();
    let (iot_client, crypto_client, admin, _key) = setup_with_crypto(&env);
    let operator = Address::generate(&env);
    iot_client.set_role(&admin, &operator, &Role::Operator);
    let (mfr_id, mfr_key) =
        register_manufacturer_with_crypto(&env, &iot_client, &crypto_client, &admin, 1);
    register_device(&env, &iot_client, &operator, &mfr_id, 10, &mfr_key);
    register_device(&env, &iot_client, &operator, &mfr_id, 11, &mfr_key);

    assert_eq!(iot_client.get_device_count(), 2);
}

#[test]
fn test_publish_firmware() {
    let env = Env::default();
    let (iot_client, _crypto_client, admin, _key) = setup_with_crypto(&env);
    // Need to grant the admin the Manufacturer role.
    iot_client.set_role(&admin, &admin, &Role::Manufacturer);
    let mfr_id = register_manufacturer(&env, &iot_client, &admin, 1);

    let binary_hash = make_bytes32(&env, 200);
    let notes = String::from_str(&env, "ipfs://release-notes");
    iot_client.publish_firmware(
        &admin,
        &mfr_id,
        &1u32,
        &DeviceType::VitalSignsMonitor,
        &binary_hash,
        &notes,
        &0u32,
        &1024u64,
    );

    let fw = iot_client.get_firmware(&mfr_id, &1u32);
    assert_eq!(fw.status, FirmwareStatus::Pending);
    assert_eq!(fw.size_bytes, 1024);
}

#[test]
fn test_approve_firmware() {
    let env = Env::default();
    let (iot_client, _crypto_client, admin, _key) = setup_with_crypto(&env);
    iot_client.set_role(&admin, &admin, &Role::Manufacturer);
    let mfr_id = register_manufacturer(&env, &iot_client, &admin, 1);

    let binary_hash = make_bytes32(&env, 200);
    let notes = String::from_str(&env, "notes");
    iot_client.publish_firmware(
        &admin,
        &mfr_id,
        &1u32,
        &DeviceType::VitalSignsMonitor,
        &binary_hash,
        &notes,
        &0u32,
        &1024u64,
    );
    iot_client.approve_firmware(&admin, &mfr_id, &1u32);
    let fw = iot_client.get_firmware(&mfr_id, &1u32);
    assert_eq!(fw.status, FirmwareStatus::Approved);
}

#[test]
fn test_update_device_firmware() {
    let env = Env::default();
    let (iot_client, crypto_client, admin, _key) = setup_with_crypto(&env);
    iot_client.set_role(&admin, &admin, &Role::Manufacturer);
    let operator = Address::generate(&env);
    iot_client.set_role(&admin, &operator, &Role::Operator);
    let (mfr_id, mfr_key) =
        register_manufacturer_with_crypto(&env, &iot_client, &crypto_client, &admin, 1);
    let device_id = register_device(&env, &iot_client, &operator, &mfr_id, 10, &mfr_key);
    iot_client.activate_device(&operator, &device_id);

    let binary_hash = make_bytes32(&env, 200);
    let notes = String::from_str(&env, "v1");
    iot_client.publish_firmware(
        &admin,
        &mfr_id,
        &1u32,
        &DeviceType::VitalSignsMonitor,
        &binary_hash,
        &notes,
        &0u32,
        &1024u64,
    );
    iot_client.approve_firmware(&admin, &mfr_id, &1u32);

    iot_client.update_device_firmware(&operator, &device_id, &1u32);
    let device = iot_client.get_device(&device_id);
    assert_eq!(device.firmware_version, 1);
}

#[test]
fn test_firmware_downgrade_not_allowed() {
    let env = Env::default();
    let (iot_client, crypto_client, admin, _key) = setup_with_crypto(&env);
    iot_client.set_role(&admin, &admin, &Role::Manufacturer);
    let operator = Address::generate(&env);
    iot_client.set_role(&admin, &operator, &Role::Operator);
    let (mfr_id, mfr_key) =
        register_manufacturer_with_crypto(&env, &iot_client, &crypto_client, &admin, 1);
    let device_id = register_device(&env, &iot_client, &operator, &mfr_id, 10, &mfr_key);
    iot_client.activate_device(&operator, &device_id);

    let hash1 = make_bytes32(&env, 200);
    let hash2 = make_bytes32(&env, 201);
    let notes = String::from_str(&env, "notes");
    iot_client.publish_firmware(
        &admin,
        &mfr_id,
        &1u32,
        &DeviceType::VitalSignsMonitor,
        &hash1,
        &notes,
        &0u32,
        &512u64,
    );
    iot_client.approve_firmware(&admin, &mfr_id, &1u32);
    iot_client.publish_firmware(
        &admin,
        &mfr_id,
        &2u32,
        &DeviceType::VitalSignsMonitor,
        &hash2,
        &notes,
        &1u32,
        &1024u64,
    );
    iot_client.approve_firmware(&admin, &mfr_id, &2u32);

    iot_client.update_device_firmware(&operator, &device_id, &2u32);
    let result = iot_client.try_update_device_firmware(&operator, &device_id, &1u32);
    assert_eq!(result, Err(Ok(Error::DowngradeNotAllowed)));
}

#[test]
fn test_submit_heartbeat() {
    let env = Env::default();
    let (iot_client, crypto_client, admin, _key) = setup_with_crypto(&env);
    let operator = Address::generate(&env);
    iot_client.set_role(&admin, &operator, &Role::Operator);
    let (mfr_id, mfr_key) =
        register_manufacturer_with_crypto(&env, &iot_client, &crypto_client, &admin, 1);
    let device_id = register_device(&env, &iot_client, &operator, &mfr_id, 10, &mfr_key);
    iot_client.activate_device(&operator, &device_id);

    env.ledger().with_mut(|li| li.timestamp = 1000);

    let metrics_ref = String::from_str(&env, "ipfs://metrics-001");
    iot_client.submit_heartbeat(
        &operator,
        &device_id,
        &HealthStatus::Healthy,
        &95u32,
        &80u32,
        &0u32,
        &metrics_ref,
    );

    let device = iot_client.get_device(&device_id);
    assert_eq!(device.last_heartbeat, 1000);
    assert_eq!(device.health_status, HealthStatus::Healthy);
}

#[test]
fn test_heartbeat_too_frequent() {
    let env = Env::default();
    let (iot_client, crypto_client, admin, _key) = setup_with_crypto(&env);
    let operator = Address::generate(&env);
    iot_client.set_role(&admin, &operator, &Role::Operator);
    let (mfr_id, mfr_key) =
        register_manufacturer_with_crypto(&env, &iot_client, &crypto_client, &admin, 1);
    let device_id = register_device(&env, &iot_client, &operator, &mfr_id, 10, &mfr_key);
    iot_client.activate_device(&operator, &device_id);

    env.ledger().with_mut(|li| li.timestamp = 1000);
    let metrics_ref = String::from_str(&env, "m");
    iot_client.submit_heartbeat(
        &operator,
        &device_id,
        &HealthStatus::Healthy,
        &95u32,
        &80u32,
        &0u32,
        &metrics_ref,
    );

    env.ledger().with_mut(|li| li.timestamp = 1030);
    let result = iot_client.try_submit_heartbeat(
        &operator,
        &device_id,
        &HealthStatus::Healthy,
        &95u32,
        &80u32,
        &0u32,
        &metrics_ref,
    );
    assert_eq!(result, Err(Ok(Error::HeartbeatTooFrequent)));
}

#[test]
fn test_get_device_uptime() {
    let env = Env::default();
    let (iot_client, crypto_client, admin, _key) = setup_with_crypto(&env);
    let operator = Address::generate(&env);
    iot_client.set_role(&admin, &operator, &Role::Operator);
    let (mfr_id, mfr_key) =
        register_manufacturer_with_crypto(&env, &iot_client, &crypto_client, &admin, 1);
    let device_id = register_device(&env, &iot_client, &operator, &mfr_id, 10, &mfr_key);

    env.ledger().with_mut(|li| li.timestamp = 1000);
    iot_client.activate_device(&operator, &device_id);

    env.ledger().with_mut(|li| li.timestamp = 2000);
    let uptime_bps = iot_client.get_device_uptime_bps(&device_id);
    assert_eq!(uptime_bps, 10000);
}

#[test]
fn test_get_active_device_count() {
    let env = Env::default();
    let (iot_client, crypto_client, admin, _key) = setup_with_crypto(&env);
    let operator = Address::generate(&env);
    iot_client.set_role(&admin, &operator, &Role::Operator);
    let (mfr_id, mfr_key) =
        register_manufacturer_with_crypto(&env, &iot_client, &crypto_client, &admin, 1);

    let d1 = register_device(&env, &iot_client, &operator, &mfr_id, 10, &mfr_key);
    let d2 = register_device(&env, &iot_client, &operator, &mfr_id, 11, &mfr_key);
    iot_client.activate_device(&operator, &d1);
    iot_client.activate_device(&operator, &d2);

    assert_eq!(iot_client.get_active_device_count(), 2);
    iot_client.suspend_device(&operator, &d1);
    assert_eq!(iot_client.get_active_device_count(), 1);
}

#[test]
fn test_create_comm_channel() {
    let env = Env::default();
    let (iot_client, crypto_client, admin, _key) = setup_with_crypto(&env);
    let operator = Address::generate(&env);
    iot_client.set_role(&admin, &operator, &Role::Operator);
    let (mfr_id, mfr_key) =
        register_manufacturer_with_crypto(&env, &iot_client, &crypto_client, &admin, 1);
    let device_id = register_device(&env, &iot_client, &operator, &mfr_id, 10, &mfr_key);
    iot_client.activate_device(&operator, &device_id);

    let channel_id = make_bytes32(&env, 30);
    let enc_key_hash = make_bytes32(&env, 31);
    let protocol = String::from_str(&env, "TLS1.3-MQTT");
    iot_client.create_comm_channel(&operator, &device_id, &channel_id, &enc_key_hash, &protocol);

    let channel = iot_client.get_comm_channel(&channel_id);
    assert_eq!(channel.device_id, device_id);
    assert_eq!(channel.rotation_count, 0);
}

#[test]
fn test_rotate_encryption_key() {
    let env = Env::default();
    let (iot_client, crypto_client, admin, _key) = setup_with_crypto(&env);
    let operator = Address::generate(&env);
    iot_client.set_role(&admin, &operator, &Role::Operator);
    let (mfr_id, mfr_key) =
        register_manufacturer_with_crypto(&env, &iot_client, &crypto_client, &admin, 1);
    let device_id = register_device(&env, &iot_client, &operator, &mfr_id, 10, &mfr_key);
    iot_client.activate_device(&operator, &device_id);

    let channel_id = make_bytes32(&env, 30);
    let enc_key_hash = make_bytes32(&env, 31);
    let protocol = String::from_str(&env, "TLS1.3");
    iot_client.create_comm_channel(&operator, &device_id, &channel_id, &enc_key_hash, &protocol);

    env.ledger().with_mut(|li| li.timestamp = 5000);
    let new_key = make_bytes32(&env, 32);
    iot_client.rotate_encryption_key(&operator, &channel_id, &new_key);

    let channel = iot_client.get_comm_channel(&channel_id);
    assert_eq!(channel.encryption_key_hash, new_key);
    assert_eq!(channel.rotation_count, 1);
}

#[test]
fn test_rotate_key_too_frequent() {
    let env = Env::default();
    let (iot_client, crypto_client, admin, _key) = setup_with_crypto(&env);
    let operator = Address::generate(&env);
    iot_client.set_role(&admin, &operator, &Role::Operator);
    let (mfr_id, mfr_key) =
        register_manufacturer_with_crypto(&env, &iot_client, &crypto_client, &admin, 1);
    let device_id = register_device(&env, &iot_client, &operator, &mfr_id, 10, &mfr_key);
    iot_client.activate_device(&operator, &device_id);

    let channel_id = make_bytes32(&env, 30);
    let enc_key = make_bytes32(&env, 31);
    let protocol = String::from_str(&env, "TLS1.3");
    iot_client.create_comm_channel(&operator, &device_id, &channel_id, &enc_key, &protocol);

    env.ledger().with_mut(|li| li.timestamp = 5000);
    let key2 = make_bytes32(&env, 32);
    iot_client.rotate_encryption_key(&operator, &channel_id, &key2);

    env.ledger().with_mut(|li| li.timestamp = 5100);
    let key3 = make_bytes32(&env, 33);
    let result = iot_client.try_rotate_encryption_key(&operator, &channel_id, &key3);
    assert_eq!(result, Err(Ok(Error::KeyRotationTooFrequent)));
}

#[test]
fn test_rotate_device_encryption_key() {
    let env = Env::default();
    let (iot_client, crypto_client, admin, _key) = setup_with_crypto(&env);
    let operator = Address::generate(&env);
    iot_client.set_role(&admin, &operator, &Role::Operator);
    let (mfr_id, mfr_key) =
        register_manufacturer_with_crypto(&env, &iot_client, &crypto_client, &admin, 1);
    let device_id = register_device(&env, &iot_client, &operator, &mfr_id, 10, &mfr_key);

    let new_key = make_bytes32(&env, 99);
    iot_client.rotate_device_key(&operator, &device_id, &new_key);

    let device = iot_client.get_device(&device_id);
    assert_eq!(device.encryption_key_hash, new_key);
}

#[test]
fn test_get_devices_by_manufacturer() {
    let env = Env::default();
    let (iot_client, crypto_client, admin, _key) = setup_with_crypto(&env);
    let operator = Address::generate(&env);
    iot_client.set_role(&admin, &operator, &Role::Operator);
    let (mfr_id, mfr_key) =
        register_manufacturer_with_crypto(&env, &iot_client, &crypto_client, &admin, 1);

    register_device(&env, &iot_client, &operator, &mfr_id, 10, &mfr_key);
    register_device(&env, &iot_client, &operator, &mfr_id, 11, &mfr_key);

    let devices = iot_client.get_devices_by_manufacturer(&mfr_id);
    assert_eq!(devices.len(), 2);
}

#[test]
fn test_get_firmware_update_history() {
    let env = Env::default();
    let (iot_client, crypto_client, admin, _key) = setup_with_crypto(&env);
    iot_client.set_role(&admin, &admin, &Role::Manufacturer);
    let operator = Address::generate(&env);
    iot_client.set_role(&admin, &operator, &Role::Operator);
    let (mfr_id, mfr_key) =
        register_manufacturer_with_crypto(&env, &iot_client, &crypto_client, &admin, 1);
    let device_id = register_device(&env, &iot_client, &operator, &mfr_id, 10, &mfr_key);
    iot_client.activate_device(&operator, &device_id);

    let hash = make_bytes32(&env, 200);
    let notes = String::from_str(&env, "v1");
    iot_client.publish_firmware(
        &admin,
        &mfr_id,
        &1u32,
        &DeviceType::VitalSignsMonitor,
        &hash,
        &notes,
        &0u32,
        &512u64,
    );
    iot_client.approve_firmware(&admin, &mfr_id, &1u32);
    iot_client.update_device_firmware(&operator, &device_id, &1u32);

    let history = iot_client.get_device_firmware_history(&device_id);
    assert_eq!(history.len(), 1);
    assert_eq!(history.get(0).unwrap().to_version, 1);
}

#[test]
fn test_get_manufacturer_count() {
    let env = Env::default();
    let (iot_client, _crypto_client, admin, _key) = setup_with_crypto(&env);
    register_manufacturer(&env, &iot_client, &admin, 1);
    register_manufacturer(&env, &iot_client, &admin, 2);
    assert_eq!(iot_client.get_manufacturer_count(), 2);
}

#[test]
fn test_error_codes_are_stable() {
    assert_eq!(Error::Unauthorized as u32, 100);
    assert_eq!(Error::NotAdmin as u32, 102);
    assert_eq!(Error::InputTooLong as u32, 201);
    assert_eq!(Error::InputTooShort as u32, 202);
    assert_eq!(Error::NotInitialized as u32, 300);
    assert_eq!(Error::AlreadyInitialized as u32, 301);
    assert_eq!(Error::ContractPaused as u32, 302);
    assert_eq!(Error::DeviceNotFound as u32, 405);
    assert_eq!(Error::InvalidEncryptionKey as u32, 602);
    assert_eq!(Error::DeviceDecommissioned as u32, 820);
    assert_eq!(Error::CryptoRegistryNotConfigured as u32, 604);
    assert_eq!(Error::ManufacturerCryptoOwnerNotSet as u32, 605);
    assert_eq!(Error::InvalidFirmwareSignature as u32, 606);
    assert_eq!(Error::CryptoKeyBundleNotFound as u32, 607);
    assert_eq!(Error::InvalidSigningKeyAlgorithm as u32, 608);
}

#[test]
fn test_get_suggestion_returns_expected_hint() {
    use crate::errors::get_suggestion;
    use soroban_sdk::{symbol_short, Env};
    let env = Env::default();
    let _ = env;
    assert_eq!(
        get_suggestion(Error::Unauthorized),
        symbol_short!("CHK_AUTH")
    );
    assert_eq!(
        get_suggestion(Error::NotInitialized),
        symbol_short!("INIT_CTR")
    );
    assert_eq!(
        get_suggestion(Error::AlreadyInitialized),
        symbol_short!("ALREADY")
    );
    assert_eq!(
        get_suggestion(Error::InputTooLong),
        symbol_short!("CHK_LEN")
    );
    assert_eq!(
        get_suggestion(Error::DeviceNotFound),
        symbol_short!("CHK_ID")
    );
    assert_eq!(
        get_suggestion(Error::ContractPaused),
        symbol_short!("RE_TRY_L")
    );
    assert_eq!(
        get_suggestion(Error::CryptoRegistryNotConfigured),
        symbol_short!("CFG_LINK")
    );
    assert_eq!(
        get_suggestion(Error::InvalidFirmwareSignature),
        symbol_short!("BAD_SIG")
    );
}
