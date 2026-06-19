use crate::types::{
    InvariantCheck, PermissionCheck, ResourceTracker, StateConsistencyCheck, ValidationReport,
};
use soroban_sdk::{Address, Env, String, Symbol};

pub fn publish_initialization(env: &Env, admin: &Address) {
    env.events().publish(
        (
            String::from_str(env, "vst/runtime_validation"),
            Symbol::new(env, "init"),
        ),
        admin,
    );
}

pub fn publish_invariant_registered(env: &Env, check: &InvariantCheck) {
    env.events().publish(
        (
            String::from_str(env, "vst/runtime_validation"),
            Symbol::new(env, "inv_reg"),
        ),
        (check.check_id.clone(), check.severity),
    );
}

pub fn publish_state_check_registered(env: &Env, check: &StateConsistencyCheck) {
    env.events().publish(
        (
            String::from_str(env, "vst/runtime_validation"),
            Symbol::new(env, "state_reg"),
        ),
        check.check_id.clone(),
    );
}

pub fn publish_permission_check_registered(env: &Env, check: &PermissionCheck) {
    env.events().publish(
        (
            String::from_str(env, "vst/runtime_validation"),
            Symbol::new(env, "perm_reg"),
        ),
        check.check_id.clone(),
    );
}

pub fn publish_resource_tracker_registered(env: &Env, tracker: &ResourceTracker) {
    env.events().publish(
        (
            String::from_str(env, "vst/runtime_validation"),
            Symbol::new(env, "res_reg"),
        ),
        (tracker.tracker_id.clone(), tracker.max_allocation),
    );
}

pub fn publish_violation_reported(env: &Env, report: &ValidationReport) {
    env.events().publish(
        (
            String::from_str(env, "vst/runtime_validation"),
            Symbol::new(env, "violation"),
        ),
        (report.violation_id, report.check_id.clone()),
    );
}

pub fn publish_invariant_violation(env: &Env, check_id: &soroban_sdk::String, value: i128) {
    env.events().publish(
        (
            String::from_str(env, "vst/runtime_validation"),
            Symbol::new(env, "inv_viol"),
        ),
        (check_id.clone(), value),
    );
}

pub fn publish_state_violation(
    env: &Env,
    check_id: &soroban_sdk::String,
    state: &soroban_sdk::String,
) {
    env.events().publish(
        (
            String::from_str(env, "vst/runtime_validation"),
            Symbol::new(env, "state_viol"),
        ),
        (check_id.clone(), state.clone()),
    );
}

pub fn publish_permission_violation(
    env: &Env,
    check_id: &soroban_sdk::String,
    role: &soroban_sdk::String,
) {
    env.events().publish(
        (
            String::from_str(env, "vst/runtime_validation"),
            Symbol::new(env, "perm_viol"),
        ),
        (check_id.clone(), role.clone()),
    );
}

pub fn publish_resource_updated(env: &Env, tracker: &ResourceTracker) {
    env.events().publish(
        (
            String::from_str(env, "vst/runtime_validation"),
            Symbol::new(env, "res_upd"),
        ),
        (tracker.tracker_id.clone(), tracker.current_usage),
    );
}
