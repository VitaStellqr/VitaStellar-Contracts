use soroban_sdk::contracterror;

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum Error {
    NotInitialized = 1,
    AlreadyInitialized = 2,
    Unauthorized = 3,
    AlreadyRegistered = 4,
    NotFound = 5,
    AlreadyPaused = 6,
    NotPaused = 7,
    UnpauseNotScheduled = 8,
    TimelockNotElapsed = 9,
    ReentrantCall = 10,
}
