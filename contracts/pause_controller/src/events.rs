use soroban_sdk::{symbol_short, Address, Env, Symbol};

pub fn emit_initialized(env: &Env, admin: &Address) {
    env.events()
        .publish((symbol_short!("init"),), (admin.clone(),));
}

pub fn emit_contract_registered(env: &Env, name: &Symbol, address: &Address) {
    env.events()
        .publish((symbol_short!("ctr_reg"),), (name.clone(), address.clone()));
}

pub fn emit_contract_unregistered(env: &Env, name: &Symbol) {
    env.events()
        .publish((symbol_short!("ctr_unreg"),), (name.clone(),));
}

pub fn emit_system_paused(env: &Env, admin: &Address, count: u32) {
    env.events()
        .publish((symbol_short!("SysPause"),), (admin.clone(), count));
}

pub fn emit_system_unpaused(env: &Env, admin: &Address, count: u32) {
    env.events()
        .publish((symbol_short!("SysUnpaus"),), (admin.clone(), count));
}

pub fn emit_unpause_scheduled(env: &Env, admin: &Address, eta: u64) {
    env.events()
        .publish((symbol_short!("UnpSched"),), (admin.clone(), eta));
}
