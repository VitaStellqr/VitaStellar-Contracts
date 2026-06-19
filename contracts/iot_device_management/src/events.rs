use soroban_sdk::{Address, BytesN, Env, String, Symbol};

use crate::{DeviceStatus, DeviceType, FirmwareStatus, HealthStatus};

pub fn emit_initialized(env: &Env, admin: &Address) {
    env.events().publish(
        (
            String::from_str(env, "vst/iot_device_mgmt"),
            Symbol::new(env, "init"),
        ),
        admin.clone(),
    );
}

pub fn emit_device_registered(
    env: &Env,
    device_id: &BytesN<32>,
    device_type: DeviceType,
    operator: &Address,
) {
    env.events().publish(
        (
            String::from_str(env, "vst/iot_device_mgmt"),
            Symbol::new(env, "dev_reg"),
        ),
        (device_id.clone(), device_type as u32, operator.clone()),
    );
}

pub fn emit_device_status_changed(
    env: &Env,
    device_id: &BytesN<32>,
    old_status: DeviceStatus,
    new_status: DeviceStatus,
) {
    env.events().publish(
        (
            String::from_str(env, "vst/iot_device_mgmt"),
            Symbol::new(env, "dev_sts"),
        ),
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
        (
            String::from_str(env, "vst/iot_device_mgmt"),
            Symbol::new(env, "fw_pub"),
        ),
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
        (
            String::from_str(env, "vst/iot_device_mgmt"),
            Symbol::new(env, "fw_sts"),
        ),
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
        (
            String::from_str(env, "vst/iot_device_mgmt"),
            Symbol::new(env, "fw_upd"),
        ),
        (device_id.clone(), from_version, to_version, success),
    );
}

pub fn emit_heartbeat(env: &Env, device_id: &BytesN<32>, health_status: HealthStatus) {
    env.events().publish(
        (
            String::from_str(env, "vst/iot_device_mgmt"),
            Symbol::new(env, "heartbeat"),
        ),
        (device_id.clone(), health_status as u32),
    );
}

pub fn emit_key_rotated(env: &Env, device_id: &BytesN<32>, rotation_count: u32) {
    env.events().publish(
        (
            String::from_str(env, "vst/iot_device_mgmt"),
            Symbol::new(env, "key_rot"),
        ),
        (device_id.clone(), rotation_count),
    );
}

#[allow(dead_code)]
pub fn emit_manufacturer_registered(env: &Env, manufacturer_id: &BytesN<32>, _name: &str) {
    env.events().publish(
        (
            String::from_str(env, "vst/iot_device_mgmt"),
            Symbol::new(env, "mfr_reg"),
        ),
        manufacturer_id.clone(),
    );
}

pub fn emit_paused(env: &Env, admin: &Address) {
    env.events().publish(
        (
            String::from_str(env, "vst/iot_device_mgmt"),
            Symbol::new(env, "paused"),
        ),
        admin.clone(),
    );
}

pub fn emit_unpaused(env: &Env, admin: &Address) {
    env.events().publish(
        (
            String::from_str(env, "vst/iot_device_mgmt"),
            Symbol::new(env, "unpaused"),
        ),
        admin.clone(),
    );
}
