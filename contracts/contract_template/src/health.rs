#![no_std]

use soroban_sdk::{contracttype, String};

/// Canonical standardized health status shape shared across contracts.
///
/// Target shape (per task): `HealthStatus { status, version, paused }`.
#[derive(Clone, Debug, Eq, PartialEq)]
#[contracttype]
pub struct HealthStatus {
    /// Contract health state.
    pub status: HealthState,
    /// Contract version string.
    pub version: String,
    /// Whether the contract is paused.
    pub paused: bool,
}

/// Standard health states.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[contracttype]
#[repr(u32)]
pub enum HealthState {

    /// Contract is initialized and operating normally.
    Ok = 0,
    /// Contract is initialized but paused.
    Paused = 1,
    /// Contract storage not initialized.
    NotInitialized = 2,
    /// Contract is initialized but degraded (non-fatal issues).
    Degraded = 3,
}

