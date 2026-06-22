use soroban_sdk::{symbol_short, Address, Env, String, Symbol};

/// Canonical shared event publisher.
///
/// Topics are emitted as: ( <contract_symbol>, <event_name_symbol> )
/// where both are `Symbol`s.
pub fn publish_event<P>(
    env: &Env,
    contract_symbol: &Symbol,
    event_name: &str,
    payload: P,
) where
    P: soroban_sdk::IntoVal<Env, soroban_sdk::Val>,
{
    // Canonical event topic shape: ( <contract_symbol>, <event_name_symbol> )
    let event_sym = Symbol::new(env, event_name);
    env.events().publish((contract_symbol.clone(), event_sym), payload);
}

// ---------------------------------------------------------------------------
// Template events (kept as-is for backwards compatibility with tests)
// ---------------------------------------------------------------------------

pub fn emit_initialized(env: &Env, admin: &Address) {
    env.events()
        .publish((symbol_short!("init"),), (admin.clone(),));
}

pub fn emit_admin_transferred(env: &Env, old_admin: &Address, new_admin: &Address) {
    env.events().publish(
        (symbol_short!("adm_xfer"),),
        (old_admin.clone(), new_admin.clone()),
    );
}



pub fn emit_data_updated(env: &Env, caller: &Address, data: &String) {
    env.events()
        .publish((symbol_short!("upd_data"),), (caller.clone(), data.clone()));
}

