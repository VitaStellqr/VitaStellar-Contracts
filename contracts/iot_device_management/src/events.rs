use soroban_sdk::{symbol_short, Address, BytesN, Env};

use crate::{DeviceStatus, DeviceType, FirmwareStatus, HealthStatus};

pub fn emit_initialized(env: &Env, admin: &Address) {
    env.events()
        .publish(("IoT", symbol_short!("init")), admin.clone());
}

pub fn emit_device_registered(
    env: &Env,
    device_id: &BytesN<32>,
    device_type: DeviceType,
    operator: &Address,
) {
    env.events().publish(
        ("IoT", symbol_short!("dev_reg")),
        (device_id.clone(), device_type as u32, operator.clone()),
    );
}

/// Emitted after a device's `(device_id, firmware_hash)` payload was
/// successfully verified against the manufacturer's active signing public
/// key resolved via `crypto_registry`.
pub fn emit_firmware_signature_verified(
    env: &Env,
    device_id: &BytesN<32>,
    firmware_hash: &BytesN<32>,
    manufacturer_id: &BytesN<32>,
) {
    env.events().publish(
        ("IoT", symbol_short!("fw_sig")),
        (device_id.clone(), firmware_hash.clone(), manufacturer_id.clone()),
    );
}

/// Emitted when the admin binds the contract to a crypto_registry. This
/// enables manufacturer-signed firmware verification.
pub fn emit_crypto_registry_set(env: &Env, admin: &Address, contract: &Address) {
    env.events().publish(
        ("IoT", symbol_short!("cry_reg")),
        (admin.clone(), contract.clone()),
    );
}

/// Emitted when the admin links a manufacturer record to its on-chain
/// identity (the `crypto_registry` owner address) so the contract can fetch
/// the manufacturer's signing public key.
pub fn emit_manufacturer_crypto_owner_set(
    env: &Env,
    admin: &Address,
    manufacturer_id: &BytesN<32>,
    crypto_owner: &Address,
) {
    env.events().publish(
        ("IoT", symbol_short!("mfr_key")),
        (admin.clone(), manufacturer_id.clone(), crypto_owner.clone()),
    );
}

pub fn emit_device_status_changed(
    env: &Env,
    device_id: &BytesN<32>,
    old_status: DeviceStatus,
    new_status: DeviceStatus,
) {
    env.events().publish(
        ("IoT", symbol_short!("dev_sts")),
        (device_id.clone(), old_status as u32, new_status as u32),
    );
}

pub fn emit_firmware_published(
    env: &Env,
    manufacturer_id: &BytesN<32>,
    version: u32,
    device_type: DeviceType,
) {
    env.events().publish(
        ("IoT", symbol_short!("fw_pub")),
        (manufacturer_id.clone(), version, device_type as u32),
    );
}

pub fn emit_firmware_status_changed(
    env: &Env,
    manufacturer_id: &BytesN<32>,
    version: u32,
    status: FirmwareStatus,
) {
    env.events().publish(
        ("IoT", symbol_short!("fw_sts")),
        (manufacturer_id.clone(), version, status as u32),
    );
}

pub fn emit_firmware_updated(
    env: &Env,
    device_id: &BytesN<32>,
    from_version: u32,
    to_version: u32,
    success: bool,
) {
    env.events().publish(
        ("IoT", symbol_short!("fw_upd")),
        (device_id.clone(), from_version, to_version, success),
    );
}

pub fn emit_heartbeat(env: &Env, device_id: &BytesN<32>, health_status: HealthStatus) {
    env.events().publish(
        ("IoT", symbol_short!("hbeat")),
        (device_id.clone(), health_status as u32),
    );
}

pub fn emit_key_rotated(env: &Env, device_id: &BytesN<32>, rotation_count: u32) {
    env.events().publish(
        ("IoT", symbol_short!("keyrot")),
        (device_id.clone(), rotation_count),
    );
}

#[allow(dead_code)] // Unused code is intentionally retained for compatibility or test scaffolding
pub fn emit_manufacturer_registered(env: &Env, manufacturer_id: &BytesN<32>, _name: &str) {
    env.events()
        .publish(("IoT", symbol_short!("mfr_reg")), manufacturer_id.clone());
}

pub fn emit_paused(env: &Env, admin: &Address) {
    env.events()
        .publish(("IoT", symbol_short!("paused")), admin.clone());
}

pub fn emit_unpaused(env: &Env, admin: &Address) {
    env.events()
        .publish(("IoT", symbol_short!("unpause")), admin.clone());
}
