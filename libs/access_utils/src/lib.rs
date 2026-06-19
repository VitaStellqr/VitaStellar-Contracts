#![no_std]

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
