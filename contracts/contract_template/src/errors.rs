//! Typed contract errors.

use soroban_sdk::contracterror;

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum Error {
    /// Contract has not been initialized yet.
    NotInitialized = 1,
    /// Contract has already been initialized.
    AlreadyInitialized = 2,
    /// Caller is not authorized to perform this action.
    Unauthorized = 3,
    /// A string or bytes input exceeded the maximum allowed length.
    InputTooLong = 4,
    /// Raised when `reentrancy::enter` returns `false` because the lock is
    /// already held — i.e. a guarded function was re-entered mid-call.
    ReentrantCall = 5,
}
