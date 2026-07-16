#![no_std]

pub mod counter;

use soroban_sdk::{Address, Env};

/// Shared helper for the common admin authorization pattern used across contracts.
///
/// The macro keeps each contract's existing `Self::require_admin(...)` helper and
/// `NotAuthorized` error behavior intact while removing repeated boilerplate.
#[macro_export]
macro_rules! require_admin {
    ($env:expr, $caller:expr) => {{
        $caller.require_auth();
        Self::require_admin(&$env, &$caller)?;
    }};
}

/// Check whether a given storage key exists in instance storage.
/// Used as a generic initialization guard.
pub fn is_initialized<K>(env: &Env, key: &K) -> bool
where
    K: soroban_sdk::IntoVal<Env, soroban_sdk::Val>,
{
    env.storage().instance().has(key)
}

/// Initialize a contract by writing an admin address under the given
/// instance-storage key. Returns `Err(())` if the key already exists
/// (contract already initialized).
pub fn init_admin<K>(env: &Env, key: &K, admin: &Address) -> Result<(), ()>
where
    K: soroban_sdk::IntoVal<Env, soroban_sdk::Val>,
{
    if is_initialized(env, key) {
        return Err(());
    }
    env.storage().instance().set(key, admin);
    Ok(())
}

#[cfg(test)]
mod tests {
    struct DemoCaller {
        authenticated: bool,
        authorized: bool,
    }

    impl DemoCaller {
        fn require_auth(&self) {
            assert!(self.authenticated);
        }
    }

    struct DemoContract;

    impl DemoContract {
        fn require_admin(_env: &(), caller: &DemoCaller) -> Result<(), &'static str> {
            if caller.authorized {
                Ok(())
            } else {
                Err("not authorized")
            }
        }

        fn guarded(env: &(), caller: DemoCaller) -> Result<(), &'static str> {
            crate::require_admin!(env, caller);
            Ok(())
        }
    }

    #[test]
    fn require_admin_macro_allows_authorized_caller() {
        let caller = DemoCaller {
            authenticated: true,
            authorized: true,
        };

        assert_eq!(DemoContract::guarded(&(), caller), Ok(()));
    }

    #[test]
    fn require_admin_macro_propagates_authorization_errors() {
        let caller = DemoCaller {
            authenticated: true,
            authorized: false,
        };

        assert_eq!(DemoContract::guarded(&(), caller), Err("not authorized"));
    }
}
