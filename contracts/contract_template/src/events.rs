use soroban_sdk::{Address, Env, String, Symbol};

/// Helper macro to create standardized event topics with the vst/<contract> prefix.
/// Usage: emit_event!(env, "contract_name", "event_name", payload)
#[macro_export]
macro_rules! emit_event {
    ($env:expr, $contract:expr, $event:expr, $payload:expr) => {
        $env.events().publish(
            (
                String::from_str($env, &format!("vst/{}", $contract)),
                Symbol::new($env, $event),
            ),
            $payload,
        )
    };
}

/// Helper to create the standardized topic string for a contract
pub fn contract_topic(env: &Env, contract_name: &str) -> String {
    format!("vst/{}", contract_name)
        .as_str()
        .try_into()
        .unwrap_or_else(|_| String::from_str(env, &format!("vst/{}", contract_name)))
}

/// Emit initialization event with standardized format
pub fn emit_initialized(env: &Env, admin: &Address) {
    env.events().publish(
        (
            String::from_str(env, "vst/contract_template"),
            Symbol::new(env, "initialized"),
        ),
        (admin.clone(),),
    );
}

/// Emit admin transfer event with standardized format
pub fn emit_admin_transferred(env: &Env, old_admin: &Address, new_admin: &Address) {
    env.events().publish(
        (
            String::from_str(env, "vst/contract_template"),
            Symbol::new(env, "admin_transferred"),
        ),
        (old_admin.clone(), new_admin.clone()),
    );
}

/// Emit data updated event with standardized format
pub fn emit_data_updated(env: &Env, caller: &Address, data: &String) {
    env.events().publish(
        (
            String::from_str(env, "vst/contract_template"),
            Symbol::new(env, "data_updated"),
        ),
        (caller.clone(), data.clone()),
    );
}
