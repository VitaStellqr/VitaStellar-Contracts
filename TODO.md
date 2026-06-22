- [ ] Implement canonical shared publish_event(env, contract_symbol, event_name, payload) in contracts/contract_template/src/events.rs
- [ ] Update contracts/contract_template/src/lib.rs to use new helper for initialize/transfer_admin/update_data (or keep as-is if already emitting matching topics)
- [ ] Migrate identity_registry to use shared publish_event helper
- [ ] Migrate appointment_booking_escrow to use shared publish_event helper
- [ ] Migrate credential_registry to use shared publish_event helper
- [ ] Migrate cross_chain_access to use shared publish_event helper
- [ ] Migrate cross_chain_bridge to use shared publish_event helper (file path discovery needed)
- [ ] Add snapshot tests for emitted topic strings for migrated events
- [ ] Run cargo test for migrated contracts and ensure existing tests pass

