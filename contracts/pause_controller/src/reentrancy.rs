use soroban_sdk::{symbol_short, Env, Symbol};

const REENTRANCY_LOCK: Symbol = symbol_short!("REENTRANC");

/// Try to acquire the per-contract reentrancy lock.
/// Returns `true` when the lock was acquired, or `false` if already locked.
pub fn enter(env: &Env) -> bool {
    let locked: bool = env
        .storage()
        .instance()
        .get(&REENTRANCY_LOCK)
        .unwrap_or(false);
    if locked {
        return false;
    }

    env.storage().instance().set(&REENTRANCY_LOCK, &true);
    true
}

/// Release the per-contract reentrancy lock.
pub fn exit(env: &Env) {
    env.storage().instance().remove(&REENTRANCY_LOCK);
}
