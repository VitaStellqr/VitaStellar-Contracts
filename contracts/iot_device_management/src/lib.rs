#![no_std]
#![allow(clippy::too_many_arguments)]
// Contract/API entrypoints need >6 args for Soroban ABI ergonomics; clippy default threshold is too tight.
#![allow(clippy::needless_pass_by_value)] // Pass-by-value is intentional for ownership or ABI reasons
#![allow(clippy::cast_possible_truncation)] // Numeric cast is intentional and considered safe here
#![allow(clippy::used_underscore_binding)] // Underscore binding is intentional for documentation or type inference

mod errors;
mod events;
mod validation;
pub use errors::Error;

#[cfg(test)]
mod test;

use soroban_sdk::{contract, contractimpl, contracttype, Address, Bytes, BytesN, Env, String, Vec};

// `crypto_registry` is a direct workspace dependency. We call its
// `get_current_key_bundle` entrypoint at runtime via the SDK-generated
// client (no `contractimport!` shim required) so the cross-contract types
// stay in lock-step with the upstream contract source.

// ============================================================
// ENUMS
// ============================================================

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[contracttype]
pub enum DeviceStatus {
    Provisioning = 0,
    Active = 1,
    Suspended = 2,
    Maintenance = 3,
    Decommissioned = 4,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[contracttype]
pub enum DeviceType {
    VitalSignsMonitor = 0,
    BloodPressureMonitor = 1,
    GlucoseMonitor = 2,
    PulseOximeter = 3,
    ECGMonitor = 4,
    TemperatureSensor = 5,
    InfusionPump = 6,
    Ventilator = 7,
    WearableSensor = 8,
    ImagingDevice = 9,
    LabAnalyzer = 10,
    Other = 99,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[contracttype]
pub enum FirmwareStatus {
    Pending = 0,
    Approved = 1,
    Rejected = 2,
    Deprecated = 3,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[contracttype]
pub enum HealthStatus {
    Healthy = 0,
    Degraded = 1,
    Critical = 2,
    Offline = 3,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[contracttype]
pub enum Role {
    Admin = 0,
    Manufacturer = 1,
    Operator = 2,
    Viewer = 3,
}

// ============================================================
// DATA STRUCTURES
// ============================================================

#[derive(Clone, Debug)]
#[contracttype]
pub struct Manufacturer {
    pub manufacturer_id: BytesN<32>,
    pub address: Address,
    pub name: String,
    pub certification_hash: BytesN<32>,
    pub is_active: bool,
    pub registered_at: u64,
    pub device_count: u32,
}

#[derive(Clone, Debug)]
#[contracttype]
pub struct Device {
    pub device_id: BytesN<32>,
    pub manufacturer_id: BytesN<32>,
    pub device_type: DeviceType,
    pub model: String,
    pub serial_number: String,
    /// Firmware version currently installed on the device.
    pub firmware_version: u32,
    /// SHA-256 hash of the firmware binary that was running when the device
    /// was registered. Verified at registration against the manufacturer's
    /// signing public key (see Issue #194).
    pub firmware_hash: BytesN<32>,
    pub status: DeviceStatus,
    pub operator: Address,
    pub location: String,
    pub registered_at: u64,
    pub last_heartbeat: u64,
    pub health_status: HealthStatus,
    pub uptime_start: u64,
    pub total_uptime_secs: u64,
    pub total_downtime_secs: u64,
    pub encryption_key_hash: BytesN<32>,
    pub metadata_ref: String,
}

/// Static device identity bundle passed to `register_device`.
///
/// Bundling these five fields together keeps `register_device` at seven ABI
/// parameters, well below Soroban's per-function limit of ten. `model`,
/// `serial_number`, and `location` together identify the physical device
/// under a manufacturer; `encryption_key_hash` and `metadata_ref` are
/// off-chain anchors recorded at provisioning time.
#[derive(Clone, Debug)]
#[contracttype]
pub struct DeviceRegistrationInfo {
    pub model: String,
    pub serial_number: String,
    pub location: String,
    pub encryption_key_hash: BytesN<32>,
    pub metadata_ref: String,
}

/// Manufacturer-signed firmware authentication bundle for Issue #194.
///
/// `firmware_hash` is the SHA-256 of the firmware binary running on the
/// device at registration. `firmware_signature` is the Ed25519 signature
/// the manufacturer produced over `iot_fw_sig_v1__ || device_id ||
/// firmware_hash`. The contract verifies it against the manufacturer's
/// active signing public key resolved through the configured
/// `crypto_registry`. Bundling the two fields keeps the public ABI
/// compact.
#[derive(Clone, Debug)]
#[contracttype]
pub struct FirmwareAttestation {
    pub firmware_hash: BytesN<32>,
    pub firmware_signature: BytesN<64>,
}

#[derive(Clone, Debug)]
#[contracttype]
pub struct FirmwareVersion {
    pub version: u32,
    pub manufacturer_id: BytesN<32>,
    pub device_type: DeviceType,
    pub binary_hash: BytesN<32>,
    pub release_notes_ref: String,
    pub status: FirmwareStatus,
    pub min_version: u32,
    pub published_at: u64,
    pub approved_by: Address,
    pub size_bytes: u64,
}

#[derive(Clone, Debug)]
#[contracttype]
pub struct FirmwareUpdateRecord {
    pub update_id: u64,
    pub device_id: BytesN<32>,
    pub from_version: u32,
    pub to_version: u32,
    pub initiated_by: Address,
    pub initiated_at: u64,
    pub completed_at: u64,
    pub success: bool,
    pub error_ref: String,
}

#[derive(Clone, Debug)]
#[contracttype]
pub struct Heartbeat {
    pub device_id: BytesN<32>,
    pub timestamp: u64,
    pub health_status: HealthStatus,
    pub battery_pct: u32,
    pub signal_strength: u32,
    pub error_count: u32,
    pub metrics_ref: String,
}

#[derive(Clone, Debug)]
#[contracttype]
pub struct CommChannel {
    pub channel_id: BytesN<32>,
    pub device_id: BytesN<32>,
    pub encryption_key_hash: BytesN<32>,
    pub protocol: String,
    pub created_at: u64,
    pub last_rotated: u64,
    pub rotation_count: u32,
}

// ============================================================
// STORAGE KEYS
// ============================================================

#[contracttype]
pub enum DataKey {
    // System
    Initialized,
    Admin,
    Paused,

    // RBAC
    UserRole(Address),

    // Manufacturers
    Manufacturer(BytesN<32>),
    ManufacturerByAddr(Address),
    ManufacturerCount,

    // Devices
    Device(BytesN<32>),
    DevicesByOperator(Address),
    DevicesByManufacturer(BytesN<32>),
    DevicesByType(u32),
    DeviceCount,
    ActiveDeviceCount,

    // Firmware
    Firmware(BytesN<32>, u32),       // (manufacturer_id, version)
    LatestFirmware(BytesN<32>, u32), // (manufacturer_id, device_type) -> version
    FirmwareUpdateRecord(u64),
    FirmwareUpdateCount,
    DeviceFirmwareUpdates(BytesN<32>), // device_id -> Vec<u64>

    // Health
    DeviceHeartbeats(BytesN<32>), // device_id -> Vec<Heartbeat> (last N)
    HeartbeatMinInterval,         // u64 seconds

    // Communication
    CommChannel(BytesN<32>),   // channel_id -> CommChannel
    DeviceChannel(BytesN<32>), // device_id -> channel_id
    KeyRotationMinInterval,    // u64 seconds

    // Issue #194: manufacturer-signed firmware verification
    /// Address of the deployed `crypto_registry` contract used to look up
    /// the manufacturer signing public key for firmware authentication.
    CryptoRegistryContract,
    /// The on-chain identity (crypto_registry owner) whose key bundle belongs
    /// to manufacturer `BytesN<32>`. Only set after admin explicitly links
    /// the manufacturer to its key bundle.
    ManufacturerCryptoOwner(BytesN<32>),
}

// ============================================================
// CONTRACT
// ============================================================

#[contract]
pub struct IoTDeviceManagement;

#[contractimpl]
impl IoTDeviceManagement {
    // ============================================================
    // SYSTEM
    // ============================================================

    pub fn initialize(env: Env, admin: Address) -> Result<(), Error> {
        admin.require_auth();
        if env.storage().instance().has(&DataKey::Initialized) {
            return Err(Error::AlreadyInitialized);
        }
        env.storage().instance().set(&DataKey::Initialized, &true);
        env.storage().instance().set(&DataKey::Admin, &admin);
        env.storage().instance().set(&DataKey::Paused, &false);
        env.storage().persistent().set(&DataKey::DeviceCount, &0u64);
        env.storage()
            .persistent()
            .set(&DataKey::ActiveDeviceCount, &0u64);
        env.storage()
            .persistent()
            .set(&DataKey::ManufacturerCount, &0u32);
        env.storage()
            .persistent()
            .set(&DataKey::FirmwareUpdateCount, &0u64);
        env.storage()
            .persistent()
            .set(&DataKey::HeartbeatMinInterval, &60u64);
        env.storage()
            .persistent()
            .set(&DataKey::KeyRotationMinInterval, &3600u64);
        events::emit_initialized(&env, &admin);
        Ok(())
    }

    pub fn pause(env: Env, admin: Address) -> Result<(), Error> {
        access_utils::require_admin!(env, admin);
        env.storage().instance().set(&DataKey::Paused, &true);
        events::emit_paused(&env, &admin);
        Ok(())
    }

    pub fn unpause(env: Env, admin: Address) -> Result<(), Error> {
        access_utils::require_admin!(env, admin);
        let paused: bool = env
            .storage()
            .instance()
            .get(&DataKey::Paused)
            .unwrap_or(false);
        if !paused {
            return Err(Error::NotPaused);
        }
        env.storage().instance().set(&DataKey::Paused, &false);
        events::emit_unpaused(&env, &admin);
        Ok(())
    }

    // ============================================================
    // RBAC
    // ============================================================

    pub fn set_role(env: Env, admin: Address, user: Address, role: Role) -> Result<(), Error> {
        access_utils::require_admin!(env, admin);
        Self::check_not_paused(&env)?;
        env.storage()
            .persistent()
            .set(&DataKey::UserRole(user), &role);
        Ok(())
    }

    pub fn get_role(env: Env, user: Address) -> Role {
        env.storage()
            .persistent()
            .get(&DataKey::UserRole(user))
            .unwrap_or(Role::Viewer)
    }

    // ============================================================
    // INTERNAL HELPERS
    // ============================================================

    fn require_admin(env: &Env, caller: &Address) -> Result<(), Error> {
        if !env.storage().instance().has(&DataKey::Initialized) {
            return Err(Error::NotInitialized);
        }
        let admin: Address = env.storage().instance().get(&DataKey::Admin).unwrap();
        if caller != &admin {
            return Err(Error::NotAdmin);
        }
        Ok(())
    }

    fn check_not_paused(env: &Env) -> Result<(), Error> {
        let paused: bool = env
            .storage()
            .instance()
            .get(&DataKey::Paused)
            .unwrap_or(false);
        if paused {
            return Err(Error::ContractPaused);
        }
        Ok(())
    }

    fn require_role(env: &Env, caller: &Address, required: Role) -> Result<(), Error> {
        let role: Role = env
            .storage()
            .persistent()
            .get(&DataKey::UserRole(caller.clone()))
            .unwrap_or(Role::Viewer);
        // Admin can do anything; otherwise must match
        let admin: Address = env.storage().instance().get(&DataKey::Admin).unwrap();
        if caller == &admin {
            return Ok(());
        }
        if role as u32 > required as u32 {
            return Err(Error::Unauthorized);
        }
        Ok(())
    }

    // ============================================================
    // MANUFACTURERS
    // ============================================================

    pub fn register_manufacturer(
        env: Env,
        admin: Address,
        manufacturer_id: BytesN<32>,
        name: String,
        certification_hash: BytesN<32>,
    ) -> Result<(), Error> {
        access_utils::require_admin!(env, admin);
        Self::check_not_paused(&env)?;
        validation::validate_name(&name)?;

        if env
            .storage()
            .persistent()
            .has(&DataKey::Manufacturer(manufacturer_id.clone()))
        {
            return Err(Error::ManufacturerAlreadyRegistered);
        }

        let mfr = Manufacturer {
            manufacturer_id: manufacturer_id.clone(),
            address: admin.clone(),
            name,
            certification_hash,
            is_active: true,
            registered_at: env.ledger().timestamp(),
            device_count: 0,
        };

        env.storage()
            .persistent()
            .set(&DataKey::Manufacturer(manufacturer_id.clone()), &mfr);
        env.storage().persistent().set(
            &DataKey::ManufacturerByAddr(admin.clone()),
            &manufacturer_id,
        );

        let count: u32 = env
            .storage()
            .persistent()
            .get(&DataKey::ManufacturerCount)
            .unwrap_or(0);
        env.storage()
            .persistent()
            .set(&DataKey::ManufacturerCount, &count.checked_add(1).unwrap());

        Ok(())
    }

    pub fn get_manufacturer(env: Env, manufacturer_id: BytesN<32>) -> Result<Manufacturer, Error> {
        env.storage()
            .persistent()
            .get(&DataKey::Manufacturer(manufacturer_id))
            .ok_or(Error::ManufacturerNotRegistered)
    }

    pub fn deactivate_manufacturer(
        env: Env,
        admin: Address,
        manufacturer_id: BytesN<32>,
    ) -> Result<(), Error> {
        access_utils::require_admin!(env, admin);
        Self::check_not_paused(&env)?;

        let mut mfr: Manufacturer = env
            .storage()
            .persistent()
            .get(&DataKey::Manufacturer(manufacturer_id.clone()))
            .ok_or(Error::ManufacturerNotRegistered)?;

        mfr.is_active = false;
        env.storage()
            .persistent()
            .set(&DataKey::Manufacturer(manufacturer_id), &mfr);
        Ok(())
    }

    // ============================================================
    // DEVICE ENROLLMENT & PROVISIONING
    // ============================================================

    pub fn register_device(
        env: Env,
        operator: Address,
        device_id: BytesN<32>,
        manufacturer_id: BytesN<32>,
        device_type: DeviceType,
        info: DeviceRegistrationInfo,
        firmware: FirmwareAttestation,
    ) -> Result<(), Error> {
        operator.require_auth();
        Self::check_not_paused(&env)?;
        Self::require_role(&env, &operator, Role::Operator)?;

        validation::validate_model(&info.model)?;
        validation::validate_serial(&info.serial_number)?;
        validation::validate_location(&info.location)?;

        if env
            .storage()
            .persistent()
            .has(&DataKey::Device(device_id.clone()))
        {
            return Err(Error::DeviceAlreadyRegistered);
        }

        let mfr: Manufacturer = env
            .storage()
            .persistent()
            .get(&DataKey::Manufacturer(manufacturer_id.clone()))
            .ok_or(Error::ManufacturerNotRegistered)?;
        if !mfr.is_active {
            return Err(Error::ManufacturerNotRegistered);
        }

        // Issue #194: manufacturer-signed firmware authentication.
        // 1. The crypto_registry contract address must be configured.
        // 2. The manufacturer must be linked to an on-chain identity whose
        //    signing key bundle is registered in crypto_registry.
        // 3. The Ed25519 signature over `device_id || firmware_hash` must
        //    verify against the manufacturer's active signing public key.
        Self::verify_manufacturer_firmware_signature(
            &env,
            &manufacturer_id,
            &device_id,
            &firmware.firmware_hash,
            &firmware.firmware_signature,
        )?;

        let now = env.ledger().timestamp();
        let device = Device {
            device_id: device_id.clone(),
            manufacturer_id: manufacturer_id.clone(),
            device_type,
            model: info.model,
            serial_number: info.serial_number,
            firmware_version: 0,
            firmware_hash: firmware.firmware_hash.clone(),
            status: DeviceStatus::Provisioning,
            operator: operator.clone(),
            location: info.location,
            registered_at: now,
            last_heartbeat: 0,
            health_status: HealthStatus::Offline,
            uptime_start: 0,
            total_uptime_secs: 0,
            total_downtime_secs: 0,
            encryption_key_hash: info.encryption_key_hash,
            metadata_ref: info.metadata_ref,
        };

        env.storage()
            .persistent()
            .set(&DataKey::Device(device_id.clone()), &device);

        // Index by operator
        let mut op_devices: Vec<BytesN<32>> = env
            .storage()
            .persistent()
            .get(&DataKey::DevicesByOperator(operator.clone()))
            .unwrap_or(Vec::new(&env));
        op_devices.push_back(device_id.clone());
        env.storage()
            .persistent()
            .set(&DataKey::DevicesByOperator(operator.clone()), &op_devices);

        // Index by manufacturer
        let mut mfr_devices: Vec<BytesN<32>> = env
            .storage()
            .persistent()
            .get(&DataKey::DevicesByManufacturer(manufacturer_id.clone()))
            .unwrap_or(Vec::new(&env));
        mfr_devices.push_back(device_id.clone());
        env.storage().persistent().set(
            &DataKey::DevicesByManufacturer(manufacturer_id.clone()),
            &mfr_devices,
        );

        // Update manufacturer device count
        let mut updated_mfr = mfr;
        updated_mfr.device_count = updated_mfr.device_count.checked_add(1).unwrap();
        env.storage().persistent().set(
            &DataKey::Manufacturer(manufacturer_id.clone()),
            &updated_mfr,
        );

        // Increment device count
        let count: u64 = env
            .storage()
            .persistent()
            .get(&DataKey::DeviceCount)
            .unwrap_or(0);
        env.storage()
            .persistent()
            .set(&DataKey::DeviceCount, &count.checked_add(1).unwrap());

        events::emit_device_registered(&env, &device_id, device_type, &operator);
        events::emit_firmware_signature_verified(
            &env,
            &device_id,
            &firmware.firmware_hash,
            &manufacturer_id,
        );
        Ok(())
    }

    pub fn get_device(env: Env, device_id: BytesN<32>) -> Result<Device, Error> {
        env.storage()
            .persistent()
            .get(&DataKey::Device(device_id))
            .ok_or(Error::DeviceNotFound)
    }

    pub fn get_device_count(env: Env) -> u64 {
        env.storage()
            .persistent()
            .get(&DataKey::DeviceCount)
            .unwrap_or(0)
    }

    pub fn get_devices_by_operator(env: Env, operator: Address) -> Vec<BytesN<32>> {
        env.storage()
            .persistent()
            .get(&DataKey::DevicesByOperator(operator))
            .unwrap_or(Vec::new(&env))
    }

    pub fn activate_device(env: Env, caller: Address, device_id: BytesN<32>) -> Result<(), Error> {
        caller.require_auth();
        Self::check_not_paused(&env)?;
        Self::require_role(&env, &caller, Role::Operator)?;

        let mut device: Device = env
            .storage()
            .persistent()
            .get(&DataKey::Device(device_id.clone()))
            .ok_or(Error::DeviceNotFound)?;

        if device.status == DeviceStatus::Decommissioned {
            return Err(Error::DeviceDecommissioned);
        }

        let old_status = device.status;
        let now = env.ledger().timestamp();
        device.status = DeviceStatus::Active;
        device.health_status = HealthStatus::Healthy;
        device.uptime_start = now;

        env.storage()
            .persistent()
            .set(&DataKey::Device(device_id.clone()), &device);

        // Increment active count if transitioning from non-active
        if old_status != DeviceStatus::Active {
            let active: u64 = env
                .storage()
                .persistent()
                .get(&DataKey::ActiveDeviceCount)
                .unwrap_or(0);
            env.storage()
                .persistent()
                .set(&DataKey::ActiveDeviceCount, &active.checked_add(1).unwrap());
        }

        events::emit_device_status_changed(&env, &device_id, old_status, DeviceStatus::Active);
        Ok(())
    }

    pub fn suspend_device(env: Env, caller: Address, device_id: BytesN<32>) -> Result<(), Error> {
        caller.require_auth();
        Self::check_not_paused(&env)?;
        Self::require_role(&env, &caller, Role::Operator)?;

        let mut device: Device = env
            .storage()
            .persistent()
            .get(&DataKey::Device(device_id.clone()))
            .ok_or(Error::DeviceNotFound)?;

        if device.status == DeviceStatus::Decommissioned {
            return Err(Error::DeviceDecommissioned);
        }

        let old_status = device.status;
        let now = env.ledger().timestamp();

        // Accumulate uptime if was active
        if old_status == DeviceStatus::Active && device.uptime_start > 0 {
            let uptime_delta = now.saturating_sub(device.uptime_start);
            device.total_uptime_secs = device.total_uptime_secs.saturating_add(uptime_delta);
        }

        device.status = DeviceStatus::Suspended;
        device.uptime_start = 0;
        env.storage()
            .persistent()
            .set(&DataKey::Device(device_id.clone()), &device);

        if old_status == DeviceStatus::Active {
            let active: u64 = env
                .storage()
                .persistent()
                .get(&DataKey::ActiveDeviceCount)
                .unwrap_or(0);
            env.storage()
                .persistent()
                .set(&DataKey::ActiveDeviceCount, &active.saturating_sub(1));
        }

        events::emit_device_status_changed(&env, &device_id, old_status, DeviceStatus::Suspended);
        Ok(())
    }

    pub fn decommission_device(
        env: Env,
        admin: Address,
        device_id: BytesN<32>,
    ) -> Result<(), Error> {
        access_utils::require_admin!(env, admin);
        Self::check_not_paused(&env)?;

        let mut device: Device = env
            .storage()
            .persistent()
            .get(&DataKey::Device(device_id.clone()))
            .ok_or(Error::DeviceNotFound)?;

        let old_status = device.status;
        device.status = DeviceStatus::Decommissioned;
        device.health_status = HealthStatus::Offline;
        device.uptime_start = 0;
        env.storage()
            .persistent()
            .set(&DataKey::Device(device_id.clone()), &device);

        if old_status == DeviceStatus::Active {
            let active: u64 = env
                .storage()
                .persistent()
                .get(&DataKey::ActiveDeviceCount)
                .unwrap_or(0);
            env.storage()
                .persistent()
                .set(&DataKey::ActiveDeviceCount, &active.saturating_sub(1));
        }

        events::emit_device_status_changed(
            &env,
            &device_id,
            old_status,
            DeviceStatus::Decommissioned,
        );
        Ok(())
    }

    // ============================================================
    // FIRMWARE MANAGEMENT
    // ============================================================

    pub fn publish_firmware(
        env: Env,
        caller: Address,
        manufacturer_id: BytesN<32>,
        version: u32,
        device_type: DeviceType,
        binary_hash: BytesN<32>,
        release_notes_ref: String,
        min_version: u32,
        size_bytes: u64,
    ) -> Result<(), Error> {
        caller.require_auth();
        Self::check_not_paused(&env)?;
        Self::require_role(&env, &caller, Role::Manufacturer)?;

        // Verify manufacturer exists
        let _mfr: Manufacturer = env
            .storage()
            .persistent()
            .get(&DataKey::Manufacturer(manufacturer_id.clone()))
            .ok_or(Error::ManufacturerNotRegistered)?;

        if env
            .storage()
            .persistent()
            .has(&DataKey::Firmware(manufacturer_id.clone(), version))
        {
            return Err(Error::FirmwareAlreadyExists);
        }

        let fw = FirmwareVersion {
            version,
            manufacturer_id: manufacturer_id.clone(),
            device_type,
            binary_hash,
            release_notes_ref,
            status: FirmwareStatus::Pending,
            min_version,
            published_at: env.ledger().timestamp(),
            approved_by: caller.clone(),
            size_bytes,
        };

        env.storage()
            .persistent()
            .set(&DataKey::Firmware(manufacturer_id.clone(), version), &fw);
        events::emit_firmware_published(&env, &manufacturer_id, version, device_type);
        Ok(())
    }

    pub fn approve_firmware(
        env: Env,
        admin: Address,
        manufacturer_id: BytesN<32>,
        version: u32,
    ) -> Result<(), Error> {
        access_utils::require_admin!(env, admin);
        Self::check_not_paused(&env)?;

        let mut fw: FirmwareVersion = env
            .storage()
            .persistent()
            .get(&DataKey::Firmware(manufacturer_id.clone(), version))
            .ok_or(Error::FirmwareVersionNotFound)?;

        fw.status = FirmwareStatus::Approved;
        fw.approved_by = admin;
        env.storage()
            .persistent()
            .set(&DataKey::Firmware(manufacturer_id.clone(), version), &fw);

        // Update latest firmware pointer
        env.storage().persistent().set(
            &DataKey::LatestFirmware(manufacturer_id.clone(), fw.device_type as u32),
            &version,
        );

        events::emit_firmware_status_changed(
            &env,
            &manufacturer_id,
            version,
            FirmwareStatus::Approved,
        );
        Ok(())
    }

    pub fn reject_firmware(
        env: Env,
        admin: Address,
        manufacturer_id: BytesN<32>,
        version: u32,
    ) -> Result<(), Error> {
        access_utils::require_admin!(env, admin);

        let mut fw: FirmwareVersion = env
            .storage()
            .persistent()
            .get(&DataKey::Firmware(manufacturer_id.clone(), version))
            .ok_or(Error::FirmwareVersionNotFound)?;

        fw.status = FirmwareStatus::Rejected;
        env.storage()
            .persistent()
            .set(&DataKey::Firmware(manufacturer_id.clone(), version), &fw);
        events::emit_firmware_status_changed(
            &env,
            &manufacturer_id,
            version,
            FirmwareStatus::Rejected,
        );
        Ok(())
    }

    pub fn get_firmware(
        env: Env,
        manufacturer_id: BytesN<32>,
        version: u32,
    ) -> Result<FirmwareVersion, Error> {
        env.storage()
            .persistent()
            .get(&DataKey::Firmware(manufacturer_id, version))
            .ok_or(Error::FirmwareVersionNotFound)
    }

    pub fn get_latest_firmware_version(
        env: Env,
        manufacturer_id: BytesN<32>,
        device_type: DeviceType,
    ) -> Result<u32, Error> {
        env.storage()
            .persistent()
            .get(&DataKey::LatestFirmware(
                manufacturer_id,
                device_type as u32,
            ))
            .ok_or(Error::FirmwareVersionNotFound)
    }

    pub fn update_device_firmware(
        env: Env,
        caller: Address,
        device_id: BytesN<32>,
        target_version: u32,
    ) -> Result<u64, Error> {
        caller.require_auth();
        Self::check_not_paused(&env)?;
        Self::require_role(&env, &caller, Role::Operator)?;

        let mut device: Device = env
            .storage()
            .persistent()
            .get(&DataKey::Device(device_id.clone()))
            .ok_or(Error::DeviceNotFound)?;

        if device.status == DeviceStatus::Decommissioned {
            return Err(Error::DeviceDecommissioned);
        }

        // Cannot downgrade
        if target_version <= device.firmware_version {
            return Err(Error::DowngradeNotAllowed);
        }

        // Firmware must be approved
        let fw: FirmwareVersion = env
            .storage()
            .persistent()
            .get(&DataKey::Firmware(
                device.manufacturer_id.clone(),
                target_version,
            ))
            .ok_or(Error::FirmwareVersionNotFound)?;

        if fw.status != FirmwareStatus::Approved {
            return Err(Error::FirmwareNotApproved);
        }

        let from_version = device.firmware_version;
        let now = env.ledger().timestamp();

        // Record the update
        let update_count: u64 = env
            .storage()
            .persistent()
            .get(&DataKey::FirmwareUpdateCount)
            .unwrap_or(0);
        let update_id = update_count;

        let record = FirmwareUpdateRecord {
            update_id,
            device_id: device_id.clone(),
            from_version,
            to_version: target_version,
            initiated_by: caller.clone(),
            initiated_at: now,
            completed_at: now,
            success: true,
            error_ref: String::from_str(&env, ""),
        };

        env.storage()
            .persistent()
            .set(&DataKey::FirmwareUpdateRecord(update_id), &record);
        env.storage().persistent().set(
            &DataKey::FirmwareUpdateCount,
            &update_count.checked_add(1).unwrap(),
        );

        // Track per-device updates
        let mut device_updates: Vec<u64> = env
            .storage()
            .persistent()
            .get(&DataKey::DeviceFirmwareUpdates(device_id.clone()))
            .unwrap_or(Vec::new(&env));
        device_updates.push_back(update_id);
        env.storage().persistent().set(
            &DataKey::DeviceFirmwareUpdates(device_id.clone()),
            &device_updates,
        );

        // Update device firmware version
        device.firmware_version = target_version;
        env.storage()
            .persistent()
            .set(&DataKey::Device(device_id.clone()), &device);

        events::emit_firmware_updated(&env, &device_id, from_version, target_version, true);
        Ok(update_id)
    }

    // ============================================================
    // DEVICE HEALTH MONITORING
    // ============================================================

    pub fn submit_heartbeat(
        env: Env,
        caller: Address,
        device_id: BytesN<32>,
        health_status: HealthStatus,
        battery_pct: u32,
        signal_strength: u32,
        error_count: u32,
        metrics_ref: String,
    ) -> Result<(), Error> {
        caller.require_auth();
        Self::check_not_paused(&env)?;
        Self::require_role(&env, &caller, Role::Operator)?;

        validation::validate_metric_value(battery_pct, 100)?;
        validation::validate_metric_value(signal_strength, 100)?;

        let mut device: Device = env
            .storage()
            .persistent()
            .get(&DataKey::Device(device_id.clone()))
            .ok_or(Error::DeviceNotFound)?;

        if device.status == DeviceStatus::Decommissioned {
            return Err(Error::DeviceDecommissioned);
        }
        if device.status != DeviceStatus::Active && device.status != DeviceStatus::Maintenance {
            return Err(Error::DeviceNotActive);
        }

        let now = env.ledger().timestamp();
        let min_interval: u64 = env
            .storage()
            .persistent()
            .get(&DataKey::HeartbeatMinInterval)
            .unwrap_or(60);

        if device.last_heartbeat > 0 && now.saturating_sub(device.last_heartbeat) < min_interval {
            return Err(Error::HeartbeatTooFrequent);
        }

        let heartbeat = Heartbeat {
            device_id: device_id.clone(),
            timestamp: now,
            health_status,
            battery_pct,
            signal_strength,
            error_count,
            metrics_ref,
        };

        // Store last N heartbeats (rolling window of 10)
        let mut heartbeats: Vec<Heartbeat> = env
            .storage()
            .persistent()
            .get(&DataKey::DeviceHeartbeats(device_id.clone()))
            .unwrap_or(Vec::new(&env));

        if heartbeats.len() >= 10 {
            heartbeats.remove(0);
        }
        heartbeats.push_back(heartbeat);
        env.storage()
            .persistent()
            .set(&DataKey::DeviceHeartbeats(device_id.clone()), &heartbeats);

        // Update device health
        device.last_heartbeat = now;
        device.health_status = health_status;
        env.storage()
            .persistent()
            .set(&DataKey::Device(device_id.clone()), &device);

        events::emit_heartbeat(&env, &device_id, health_status);
        Ok(())
    }

    pub fn get_device_heartbeats(env: Env, device_id: BytesN<32>) -> Result<Vec<Heartbeat>, Error> {
        if !env
            .storage()
            .persistent()
            .has(&DataKey::Device(device_id.clone()))
        {
            return Err(Error::DeviceNotFound);
        }
        Ok(env
            .storage()
            .persistent()
            .get(&DataKey::DeviceHeartbeats(device_id))
            .unwrap_or(Vec::new(&env)))
    }

    pub fn get_device_uptime_bps(env: Env, device_id: BytesN<32>) -> Result<u32, Error> {
        let device: Device = env
            .storage()
            .persistent()
            .get(&DataKey::Device(device_id))
            .ok_or(Error::DeviceNotFound)?;

        let now = env.ledger().timestamp();

        // Calculate current uptime
        let mut total_up = device.total_uptime_secs;
        if device.status == DeviceStatus::Active && device.uptime_start > 0 {
            total_up = total_up.saturating_add(now.saturating_sub(device.uptime_start));
        }

        let total_time = total_up.saturating_add(device.total_downtime_secs);
        if total_time == 0 {
            // Device just registered, consider it at 100% if active
            if device.status == DeviceStatus::Active {
                return Ok(10000);
            }
            return Ok(0);
        }

        // bps = (uptime / total) * 10000
        let bps = total_up
            .saturating_mul(10000)
            .checked_div(total_time)
            .unwrap_or(0);

        Ok(bps as u32)
    }

    pub fn get_active_device_count(env: Env) -> u64 {
        env.storage()
            .persistent()
            .get(&DataKey::ActiveDeviceCount)
            .unwrap_or(0)
    }

    pub fn set_heartbeat_interval(
        env: Env,
        admin: Address,
        interval_secs: u64,
    ) -> Result<(), Error> {
        access_utils::require_admin!(env, admin);
        env.storage()
            .persistent()
            .set(&DataKey::HeartbeatMinInterval, &interval_secs);
        Ok(())
    }

    // ============================================================
    // SECURE DEVICE COMMUNICATION
    // ============================================================

    pub fn create_comm_channel(
        env: Env,
        caller: Address,
        device_id: BytesN<32>,
        channel_id: BytesN<32>,
        encryption_key_hash: BytesN<32>,
        protocol: String,
    ) -> Result<(), Error> {
        caller.require_auth();
        Self::check_not_paused(&env)?;
        Self::require_role(&env, &caller, Role::Operator)?;

        // Device must exist
        let _device: Device = env
            .storage()
            .persistent()
            .get(&DataKey::Device(device_id.clone()))
            .ok_or(Error::DeviceNotFound)?;

        let now = env.ledger().timestamp();
        let channel = CommChannel {
            channel_id: channel_id.clone(),
            device_id: device_id.clone(),
            encryption_key_hash,
            protocol,
            created_at: now,
            last_rotated: now,
            rotation_count: 0,
        };

        env.storage()
            .persistent()
            .set(&DataKey::CommChannel(channel_id.clone()), &channel);
        env.storage()
            .persistent()
            .set(&DataKey::DeviceChannel(device_id), &channel_id);
        Ok(())
    }

    pub fn get_comm_channel(env: Env, channel_id: BytesN<32>) -> Result<CommChannel, Error> {
        env.storage()
            .persistent()
            .get(&DataKey::CommChannel(channel_id))
            .ok_or(Error::ChannelNotFound)
    }

    pub fn rotate_encryption_key(
        env: Env,
        caller: Address,
        channel_id: BytesN<32>,
        new_encryption_key_hash: BytesN<32>,
    ) -> Result<(), Error> {
        caller.require_auth();
        Self::check_not_paused(&env)?;
        Self::require_role(&env, &caller, Role::Operator)?;

        let mut channel: CommChannel = env
            .storage()
            .persistent()
            .get(&DataKey::CommChannel(channel_id.clone()))
            .ok_or(Error::ChannelNotFound)?;

        let now = env.ledger().timestamp();
        let min_interval: u64 = env
            .storage()
            .persistent()
            .get(&DataKey::KeyRotationMinInterval)
            .unwrap_or(3600);

        if now.saturating_sub(channel.last_rotated) < min_interval {
            return Err(Error::KeyRotationTooFrequent);
        }

        channel.encryption_key_hash = new_encryption_key_hash;
        channel.last_rotated = now;
        channel.rotation_count = channel.rotation_count.checked_add(1).unwrap();

        env.storage()
            .persistent()
            .set(&DataKey::CommChannel(channel_id), &channel);
        events::emit_key_rotated(&env, &channel.device_id, channel.rotation_count);
        Ok(())
    }

    pub fn rotate_device_key(
        env: Env,
        caller: Address,
        device_id: BytesN<32>,
        new_encryption_key_hash: BytesN<32>,
    ) -> Result<(), Error> {
        caller.require_auth();
        Self::check_not_paused(&env)?;
        Self::require_role(&env, &caller, Role::Operator)?;

        let mut device: Device = env
            .storage()
            .persistent()
            .get(&DataKey::Device(device_id.clone()))
            .ok_or(Error::DeviceNotFound)?;

        if device.status == DeviceStatus::Decommissioned {
            return Err(Error::DeviceDecommissioned);
        }

        device.encryption_key_hash = new_encryption_key_hash;
        env.storage()
            .persistent()
            .set(&DataKey::Device(device_id.clone()), &device);
        events::emit_key_rotated(&env, &device_id, 0);
        Ok(())
    }

    pub fn set_key_rotation_interval(
        env: Env,
        admin: Address,
        interval_secs: u64,
    ) -> Result<(), Error> {
        access_utils::require_admin!(env, admin);
        env.storage()
            .persistent()
            .set(&DataKey::KeyRotationMinInterval, &interval_secs);
        Ok(())
    }

    // ============================================================
    // QUERIES & REPORTING
    // ============================================================

    pub fn get_devices_by_manufacturer(env: Env, manufacturer_id: BytesN<32>) -> Vec<BytesN<32>> {
        env.storage()
            .persistent()
            .get(&DataKey::DevicesByManufacturer(manufacturer_id))
            .unwrap_or(Vec::new(&env))
    }

    pub fn get_device_firmware_history(
        env: Env,
        device_id: BytesN<32>,
    ) -> Result<Vec<FirmwareUpdateRecord>, Error> {
        if !env
            .storage()
            .persistent()
            .has(&DataKey::Device(device_id.clone()))
        {
            return Err(Error::DeviceNotFound);
        }

        let update_ids: Vec<u64> = env
            .storage()
            .persistent()
            .get(&DataKey::DeviceFirmwareUpdates(device_id))
            .unwrap_or(Vec::new(&env));

        let mut records = Vec::new(&env);
        for id in update_ids.iter() {
            if let Some(record) = env
                .storage()
                .persistent()
                .get::<DataKey, FirmwareUpdateRecord>(&DataKey::FirmwareUpdateRecord(id))
            {
                records.push_back(record);
            }
        }
        Ok(records)
    }

    pub fn get_manufacturer_count(env: Env) -> u32 {
        env.storage()
            .persistent()
            .get(&DataKey::ManufacturerCount)
            .unwrap_or(0)
    }

    pub fn get_firmware_update_record(
        env: Env,
        update_id: u64,
    ) -> Result<FirmwareUpdateRecord, Error> {
        env.storage()
            .persistent()
            .get(&DataKey::FirmwareUpdateRecord(update_id))
            .ok_or(Error::FirmwareVersionNotFound)
    }

    // ============================================================
    // Issue #194: manufacturer-signed firmware authentication
    // ============================================================

    /// Configure the `crypto_registry` contract address used to resolve
    /// manufacturer signing public keys. Must be called before device
    /// registration can succeed, otherwise registration will fail with
    /// `CryptoRegistryNotConfigured`.
    pub fn set_crypto_registry_contract(
        env: Env,
        admin: Address,
        contract: Address,
    ) -> Result<(), Error> {
        access_utils::require_admin!(env, admin);
        Self::check_not_paused(&env)?;
        env.storage()
            .instance()
            .set(&DataKey::CryptoRegistryContract, &contract);
        events::emit_crypto_registry_set(&env, &admin, &contract);
        Ok(())
    }

    /// Return the configured crypto_registry contract address, if any.
    pub fn get_crypto_registry_contract(env: Env) -> Option<Address> {
        env.storage()
            .instance()
            .get(&DataKey::CryptoRegistryContract)
    }

    /// Link a manufacturer record to its on-chain identity (the
    /// `crypto_registry` owner address) so device registration can fetch
    /// the manufacturer's active signing public key for firmware
    /// authentication. Stored separately so the manufacturer record stays
    /// focused on certification metadata and is rotation-safe.
    pub fn set_manufacturer_crypto_owner(
        env: Env,
        admin: Address,
        manufacturer_id: BytesN<32>,
        crypto_owner: Address,
    ) -> Result<(), Error> {
        access_utils::require_admin!(env, admin);
        Self::check_not_paused(&env)?;
        if env
            .storage()
            .persistent()
            .get::<DataKey, Manufacturer>(&DataKey::Manufacturer(manufacturer_id.clone()))
            .is_none()
        {
            return Err(Error::ManufacturerNotRegistered);
        }
        env.storage().persistent().set(
            &DataKey::ManufacturerCryptoOwner(manufacturer_id.clone()),
            &crypto_owner,
        );
        events::emit_manufacturer_crypto_owner_set(&env, &admin, &manufacturer_id, &crypto_owner);
        Ok(())
    }

    /// Return the linked crypto_registry owner address for a manufacturer
    /// (if any).
    pub fn get_manufacturer_crypto_owner(env: Env, manufacturer_id: BytesN<32>) -> Option<Address> {
        env.storage()
            .persistent()
            .get(&DataKey::ManufacturerCryptoOwner(manufacturer_id))
    }

    /// Verify the `(device_id, firmware_hash, firmware_signature)` tuple
    /// against the manufacturer's active signing public key stored in
    /// `crypto_registry`. Called by `register_device` before persisting a
    /// new device record.
    ///
    /// Steps performed:
    /// 1. Ensure the crypto_registry contract address is configured.
    /// 2. Resolve the manufacturer's crypto_owner address (set via
    ///    `set_manufacturer_crypto_owner`).
    /// 3. Cross-contract call to `crypto_registry.get_current_key_bundle`
    ///    to fetch the manufacturer's signing public key.
    /// 4. Validate that the bundle advertises an Ed25519 signing key.
    /// 5. Verify the Ed25519 signature over `device_id || firmware_hash`.
    fn verify_manufacturer_firmware_signature(
        env: &Env,
        manufacturer_id: &BytesN<32>,
        device_id: &BytesN<32>,
        firmware_hash: &BytesN<32>,
        firmware_signature: &BytesN<64>,
    ) -> Result<(), Error> {
        let crypto_registry_contract: Address = env
            .storage()
            .instance()
            .get(&DataKey::CryptoRegistryContract)
            .ok_or(Error::CryptoRegistryNotConfigured)?;

        let crypto_owner: Address = env
            .storage()
            .persistent()
            .get(&DataKey::ManufacturerCryptoOwner(manufacturer_id.clone()))
            .ok_or(Error::ManufacturerCryptoOwnerNotSet)?;

        let client = crypto_registry::CryptoRegistryClient::new(env, &crypto_registry_contract);
        // The SDK-generated client flattens the upstream
        // `Result<Option<KeyBundle>, ContractError>` into a nested
        // `Result<Result<Option<KeyBundle>, ConversionError>, Result<ContractError, InvokeError>>`.
        // We keep the matching nested so the inner `KeyBundle` is only ever
        // bound inside the success arm, avoiding any shadowing or type
        // inference quirks at the outer `let`:
        //   * Outer `Err(_)` → contract invocation panicked at the host
        //     level (contract not deployed, out of gas, ...). Surface as
        //     `CryptoRegistryNotConfigured` so the operator gets a single
        //     actionable error code for "the chain is broken".
        //   * Outer `Ok(inner Err(_))` → contract returned its own
        //     `Err(_)` (e.g. `NotInitialized`, `NotAuthorized`). Same
        //     error for the operator.
        //   * Outer `Ok(inner Ok(None))` → upstream returned `Ok(None)`.
        //     The manufacturer has no published key bundle yet. Surface
        //     as `CryptoKeyBundleNotFound`.
        //   * Outer `Ok(inner Ok(Some(bundle)))` → proceed with the
        //     bundle validation entirely inside this arm.
        match client.try_get_current_key_bundle(&crypto_owner) {
            Err(_) => Err(Error::CryptoRegistryNotConfigured),
            Ok(Err(_)) => Err(Error::CryptoRegistryNotConfigured),
            Ok(Ok(None)) => Err(Error::CryptoKeyBundleNotFound),
            Ok(Ok(Some(bundle))) => {
                if bundle.revoked {
                    return Err(Error::CryptoKeyBundleNotFound);
                }
                if !bundle.has_signing_key {
                    return Err(Error::CryptoKeyBundleNotFound);
                }
                if bundle.signing_key.algorithm != crypto_registry::KeyAlgorithm::Ed25519 {
                    return Err(Error::InvalidSigningKeyAlgorithm);
                }

                // Ed25519 signing keys in crypto_registry are exactly 32 bytes.
                if bundle.signing_key.key.len() != 32 {
                    return Err(Error::InvalidSigningKeyAlgorithm);
                }
                let mut pubkey_arr = [0u8; 32];
                bundle.signing_key.key.copy_into_slice(&mut pubkey_arr);
                let pubkey = BytesN::<32>::from_array(env, &pubkey_arr);

                // Domain-separated message: prefix || device_id || firmware_hash.
                // The 15-byte prefix binds the signature to this contract + version
                // so the same `(device_id, firmware_hash)` tuple cannot be replayed
                // as a valid signature in any other protocol that happens to also
                // sign over an id+hash pairing.
                const DOMAIN_PREFIX: &[u8] = b"iot_fw_sig_v1__";
                let mut msg = Bytes::new(env);
                msg.append(&Bytes::from_slice(env, DOMAIN_PREFIX));
                msg.append(&Bytes::from_slice(env, device_id.to_array().as_slice()));
                msg.append(&Bytes::from_slice(env, firmware_hash.to_array().as_slice()));

                env.crypto()
                    .ed25519_verify(&pubkey, &msg, firmware_signature);
                Ok(())
            },
        }
    }
}
