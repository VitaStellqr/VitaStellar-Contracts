use crate::types::{DeprecationStatus, MigrationGuide, SunsetTimeline};
use soroban_sdk::{Address, Env, String, Symbol};

pub fn publish_initialization(env: &Env, admin: &Address) {
    env.events().publish(
        (
            String::from_str(env, "vst/deprecation_framework"),
            Symbol::new(env, "init"),
        ),
        admin,
    );
}

pub fn publish_deprecation_marked(env: &Env, status: &DeprecationStatus) {
    env.events().publish(
        (
            String::from_str(env, "vst/deprecation_framework"),
            Symbol::new(env, "marked"),
        ),
        (status.contract_id.clone(), status.contract_name.clone()),
    );
}

pub fn publish_sunset_timeline_set(env: &Env, timeline: &SunsetTimeline) {
    env.events().publish(
        (
            String::from_str(env, "vst/deprecation_framework"),
            Symbol::new(env, "timeline"),
        ),
        (timeline.contract_id.clone(), timeline.removal_date),
    );
}

pub fn publish_migration_guide_added(env: &Env, guide: &MigrationGuide) {
    env.events().publish(
        (
            String::from_str(env, "vst/deprecation_framework"),
            Symbol::new(env, "guide"),
        ),
        (guide.contract_id.clone(), guide.guide_title.clone()),
    );
}

pub fn publish_phase_updated(env: &Env, status: &DeprecationStatus) {
    env.events().publish(
        (
            String::from_str(env, "vst/deprecation_framework"),
            Symbol::new(env, "phase"),
        ),
        (status.contract_id.clone(), status.phase as u32),
    );
}

pub fn publish_communication_sent(env: &Env, contract_id: &soroban_sdk::String, comm_id: u64) {
    env.events().publish(
        (
            String::from_str(env, "vst/deprecation_framework"),
            Symbol::new(env, "comm"),
        ),
        (contract_id.clone(), comm_id),
    );
}

pub fn publish_removal_checklist_created(env: &Env, contract_id: &soroban_sdk::String) {
    env.events().publish(
        (
            String::from_str(env, "vst/deprecation_framework"),
            Symbol::new(env, "checklist"),
        ),
        contract_id.clone(),
    );
}

pub fn publish_checklist_item_completed(
    env: &Env,
    contract_id: &soroban_sdk::String,
    item_index: u32,
) {
    env.events().publish(
        (
            String::from_str(env, "vst/deprecation_framework"),
            Symbol::new(env, "chk_done"),
        ),
        (contract_id.clone(), item_index),
    );
}
