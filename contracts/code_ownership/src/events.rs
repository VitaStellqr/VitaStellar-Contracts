use crate::types::{ModuleOwnership, ReviewRoute};
use soroban_sdk::{Address, Env, String, Symbol};

pub fn publish_initialization(env: &Env, admin: &Address) {
    env.events().publish(
        (
            String::from_str(env, "vst/code_ownership"),
            Symbol::new(env, "init"),
        ),
        admin,
    );
}

pub fn publish_module_registered(env: &Env, ownership: &ModuleOwnership) {
    env.events().publish(
        (
            String::from_str(env, "vst/code_ownership"),
            Symbol::new(env, "mod_reg"),
        ),
        (ownership.module_id.clone(), ownership.primary_owner.clone()),
    );
}

pub fn publish_ownership_updated(env: &Env, ownership: &ModuleOwnership) {
    env.events().publish(
        (
            String::from_str(env, "vst/code_ownership"),
            Symbol::new(env, "own_upd"),
        ),
        (ownership.module_id.clone(), ownership.primary_owner.clone()),
    );
}

pub fn publish_review_route_configured(env: &Env, route: &ReviewRoute) {
    env.events().publish(
        (
            String::from_str(env, "vst/code_ownership"),
            Symbol::new(env, "route_cfg"),
        ),
        (route.module_id.clone(), route.required_reviewers),
    );
}
