//! # Pause Controller
//!
//! System-wide circuit breaker / emergency pause mechanism. Maintains a registry
//! of production contracts and can pause / unpause them atomically.
//!
//! Issue #204 — system-level emergency pause controller.
#![no_std]

mod errors;
mod events;
pub mod reentrancy;
#[cfg(test)]
mod test;
mod types;

pub use errors::Error;

use soroban_sdk::{contract, contractimpl, symbol_short, Address, Env, IntoVal, Symbol, Val, Vec};
use types::RegisteredContract;

const KEY_ADMIN: Symbol = symbol_short!("Admin");
const KEY_REGISTRY: Symbol = symbol_short!("REGISTRY");
const KEY_PAUSED: Symbol = symbol_short!("PAUSED");
const KEY_UNPAUSE_ETA: Symbol = symbol_short!("UPTIME");
const KEY_UNPAUSE_DELAY: Symbol = symbol_short!("UPDELAY");

#[contract]
pub struct PauseController;

#[contractimpl]
impl PauseController {
    // -----------------------------------------------------------------------
    // Initialization
    // -----------------------------------------------------------------------

    /// Initialize the controller. The deployer becomes the admin.
    pub fn initialize(env: Env, admin: Address) -> Result<(), Error> {
        access_utils::init_admin(&env, &KEY_ADMIN, &admin)
            .map_err(|_| Error::AlreadyInitialized)?;
        events::emit_initialized(&env, &admin);
        Ok(())
    }

    // -----------------------------------------------------------------------
    // Registry management
    // -----------------------------------------------------------------------

    /// Register a production contract for system-wide pause control.
    ///
    /// `method` is the function name the contract exposes for pausing:
    ///   - `set_paused` → `fn set_paused(env, caller, paused: bool)`
    ///   - `pause`      → `fn pause(env, caller)` / `fn unpause(env, caller)`
    pub fn register_contract(
        env: Env,
        admin: Address,
        name: Symbol,
        address: Address,
        method: Symbol,
    ) -> Result<(), Error> {
        admin.require_auth();
        Self::require_admin(&env, &admin)?;

        let mut registry: Vec<RegisteredContract> = env
            .storage()
            .instance()
            .get(&KEY_REGISTRY)
            .unwrap_or(Vec::new(&env));

        for c in registry.iter() {
            if c.name == name {
                return Err(Error::AlreadyRegistered);
            }
        }

        registry.push_back(RegisteredContract {
            name: name.clone(),
            address: address.clone(),
            method,
        });

        env.storage().instance().set(&KEY_REGISTRY, &registry);
        events::emit_contract_registered(&env, &name, &address);

        Ok(())
    }

    /// Remove a contract from the registry.
    pub fn unregister_contract(env: Env, admin: Address, name: Symbol) -> Result<(), Error> {
        admin.require_auth();
        Self::require_admin(&env, &admin)?;

        let registry: Vec<RegisteredContract> = env
            .storage()
            .instance()
            .get(&KEY_REGISTRY)
            .unwrap_or(Vec::new(&env));

        let mut new_registry: Vec<RegisteredContract> = Vec::new(&env);
        let mut found = false;

        for c in registry.iter() {
            if c.name == name {
                found = true;
            } else {
                new_registry.push_back(c);
            }
        }

        if !found {
            return Err(Error::NotFound);
        }

        env.storage().instance().set(&KEY_REGISTRY, &new_registry);
        events::emit_contract_unregistered(&env, &name);

        Ok(())
    }

    // -----------------------------------------------------------------------
    // System pause / unpause
    // -----------------------------------------------------------------------

    /// Pause all registered contracts immediately.
    pub fn pause_all(env: Env, admin: Address) -> Result<(), Error> {
        admin.require_auth();
        Self::require_admin(&env, &admin)?;

        if Self::is_paused_internal(&env) {
            return Err(Error::AlreadyPaused);
        }

        let registry: Vec<RegisteredContract> = env
            .storage()
            .instance()
            .get(&KEY_REGISTRY)
            .unwrap_or(Vec::new(&env));

        for c in registry.iter() {
            Self::invoke_pause_method(&env, &c, &admin, true)?;
        }

        env.storage().instance().set(&KEY_PAUSED, &true);
        env.storage().instance().set(&KEY_UNPAUSE_ETA, &0u64);

        events::emit_system_paused(&env, &admin, registry.len());

        Ok(())
    }

    /// Schedule an unpause with a timelock delay (in seconds from now).
    ///
    /// The actual unpause takes effect only after calling `execute_unpause`
    /// once the delay has elapsed. Calling this again before the delay elapses
    /// resets the timer.
    pub fn unpause_all(env: Env, admin: Address, delay: u64) -> Result<(), Error> {
        admin.require_auth();
        Self::require_admin(&env, &admin)?;

        if !Self::is_paused_internal(&env) {
            return Err(Error::NotPaused);
        }

        let now = env.ledger().timestamp();
        let eta = now.saturating_add(delay);

        env.storage().instance().set(&KEY_UNPAUSE_ETA, &eta);
        env.storage().instance().set(&KEY_UNPAUSE_DELAY, &delay);

        events::emit_unpause_scheduled(&env, &admin, eta);

        Ok(())
    }

    /// Execute a previously scheduled unpause. Reverts if the timelock has not
    /// elapsed.
    pub fn execute_unpause(env: Env, admin: Address) -> Result<(), Error> {
        admin.require_auth();
        Self::require_admin(&env, &admin)?;

        if !Self::is_paused_internal(&env) {
            return Err(Error::NotPaused);
        }

        let eta: u64 = env.storage().instance().get(&KEY_UNPAUSE_ETA).unwrap_or(0);
        if eta == 0 {
            return Err(Error::UnpauseNotScheduled);
        }

        let now = env.ledger().timestamp();
        if now < eta {
            return Err(Error::TimelockNotElapsed);
        }

        let registry: Vec<RegisteredContract> = env
            .storage()
            .instance()
            .get(&KEY_REGISTRY)
            .unwrap_or(Vec::new(&env));

        for c in registry.iter() {
            Self::invoke_pause_method(&env, &c, &admin, false)?;
        }

        env.storage().instance().set(&KEY_PAUSED, &false);
        env.storage().instance().set(&KEY_UNPAUSE_ETA, &0u64);

        events::emit_system_unpaused(&env, &admin, registry.len());

        Ok(())
    }

    // -----------------------------------------------------------------------
    // Read-only queries
    // -----------------------------------------------------------------------

    /// Returns the current admin address.
    pub fn get_admin(env: Env) -> Result<Address, Error> {
        env.storage()
            .instance()
            .get(&KEY_ADMIN)
            .ok_or(Error::NotInitialized)
    }

    /// Returns `true` if the system is currently paused.
    pub fn is_system_paused(env: Env) -> bool {
        env.storage().instance().get(&KEY_PAUSED).unwrap_or(false)
    }

    /// Returns the list of registered contracts.
    pub fn get_registered_contracts(env: Env) -> Vec<RegisteredContract> {
        env.storage()
            .instance()
            .get(&KEY_REGISTRY)
            .unwrap_or(Vec::new(&env))
    }

    /// Returns the scheduled unpause timestamp (0 if none).
    pub fn get_unpause_eta(env: Env) -> u64 {
        env.storage().instance().get(&KEY_UNPAUSE_ETA).unwrap_or(0)
    }

    // -----------------------------------------------------------------------
    // Internal helpers
    // -----------------------------------------------------------------------

    fn is_paused_internal(env: &Env) -> bool {
        env.storage().instance().get(&KEY_PAUSED).unwrap_or(false)
    }

    fn require_admin(env: &Env, caller: &Address) -> Result<(), Error> {
        let admin: Address = env
            .storage()
            .instance()
            .get(&KEY_ADMIN)
            .ok_or(Error::NotInitialized)?;
        if *caller != admin {
            return Err(Error::Unauthorized);
        }
        Ok(())
    }

    /// Invoke the pause / unpause method on a registered contract via
    /// `env.invoke_contract()`.
    ///
    /// Supports two conventions:
    /// - `set_paused(caller, paused: bool)` — toggle-style
    /// - `pause(caller)` / `unpause(caller)` — separate call-style
    fn invoke_pause_method(
        env: &Env,
        contract: &RegisteredContract,
        caller: &Address,
        paused: bool,
    ) -> Result<(), Error> {
        let method = contract.method.clone();
        let set_paused = Symbol::new(env, "set_paused");
        let pause_sym = symbol_short!("pause");
        let unpause_sym = symbol_short!("unpause");

        if method == set_paused {
            // set_paused(caller: Address, paused: bool)
            let args: Vec<Val> =
                Vec::from_array(env, [caller.clone().into_val(env), paused.into_val(env)]);
            env.invoke_contract::<()>(&contract.address, &method, args);
        } else if paused {
            // call pause(caller)
            let args: Vec<Val> = Vec::from_array(env, [caller.clone().into_val(env)]);
            env.invoke_contract::<()>(&contract.address, &method, args);
        } else {
            // For unpause when method is "pause", derive "unpause"
            let unpause_method = if method == pause_sym {
                unpause_sym
            } else {
                method
            };
            let args: Vec<Val> = Vec::from_array(env, [caller.clone().into_val(env)]);
            env.invoke_contract::<()>(&contract.address, &unpause_method, args);
        }

        Ok(())
    }
}
