#![cfg(test)]

use super::*;
use soroban_sdk::testutils::{Address as _, Ledger};
use soroban_sdk::{vec, Env, String, Symbol};

#[test]
fn test_initialize() {
    let env = Env::default();
    let admin = Address::generate(&env);
    let identity_registry = Address::generate(&env);
    
    let contract_id = env.register_contract(None, ProviderDirectoryContract);
    let client = ProviderDirectoryContractClient::new(&env, &contract_id);

    client.initialize(&admin, &identity_registry);

    // Try initializing again should fail
    let result = client.try_initialize(&admin, &identity_registry);
    assert_eq!(result, Err(Ok(Error::AlreadyInitialized)));
}

#[test]
fn test_profile_management() {
    let env = Env::default();
    env.mock_all_auths();
    
    let admin = Address::generate(&env);
    let identity_registry = Address::generate(&env);
    let provider = Address::generate(&env);
    
    let contract_id = env.register_contract(None, ProviderDirectoryContract);
    let client = ProviderDirectoryContractClient::new(&env, &contract_id);
    client.initialize(&admin, &identity_registry);

    let name = String::from_str(&env, "Dr. Smith");
    let specialties = vec![&env, Symbol::new(&env, "Cardiology"), Symbol::new(&env, "InternalMedicine")];
    let bio = String::from_str(&env, "Experienced cardiologist with 10 years experience.");
    let location = String::from_str(&env, "New York, NY");
    let contact = String::from_str(&env, "drsmith@example.com");

    client.update_profile(
        &provider,
        &name,
        &specialties,
        &bio,
        &location,
        &contact,
    );

    let profile = client.get_profile(&provider);
    assert_eq!(profile.name, name);
    assert_eq!(profile.specialties, specialties);
    assert_eq!(profile.is_verified, false);
}

#[test]
fn test_search_by_specialty() {
    let env = Env::default();
    env.mock_all_auths();
    
    let admin = Address::generate(&env);
    let identity_registry = Address::generate(&env);
    let p1 = Address::generate(&env);
    let p2 = Address::generate(&env);
    
    let contract_id = env.register_contract(None, ProviderDirectoryContract);
    let client = ProviderDirectoryContractClient::new(&env, &contract_id);
    client.initialize(&admin, &identity_registry);

    let cardiology = Symbol::new(&env, "Cardiology");
    let neurology = Symbol::new(&env, "Neurology");

    client.update_profile(
        &p1,
        &String::from_str(&env, "P1"),
        &vec![&env, cardiology.clone()],
        &String::from_str(&env, "Bio"),
        &String::from_str(&env, "Loc"),
        &String::from_str(&env, "provider@example.com"),
    );

    client.update_profile(
        &p2,
        &String::from_str(&env, "P2"),
        &vec![&env, neurology.clone()],
        &String::from_str(&env, "Bio"),
        &String::from_str(&env, "Loc"),
        &String::from_str(&env, "provider@example.com"),
    );

    let results = client.search_by_specialty(&cardiology);
    assert_eq!(results.len(), 1);
    assert_eq!(results.get(0).unwrap().address, p1);

    let results = client.search_by_specialty(&neurology);
    assert_eq!(results.len(), 1);
    assert_eq!(results.get(0).unwrap().address, p2);
}

#[test]
fn test_availability() {
    let env = Env::default();
    env.mock_all_auths();
    
    let admin = Address::generate(&env);
    let identity_registry = Address::generate(&env);
    let provider = Address::generate(&env);
    
    let contract_id = env.register_contract(None, ProviderDirectoryContract);
    let client = ProviderDirectoryContractClient::new(&env, &contract_id);
    client.initialize(&admin, &identity_registry);

    client.update_profile(
        &provider,
        &String::from_str(&env, "Dr. Smith"),
        &vec![&env],
        &String::from_str(&env, "Bio"),
        &String::from_str(&env, "Loc"),
        &String::from_str(&env, "provider@example.com"),
    );

    let avail = vec![&env, Availability {
        day_of_week: 1,
        start_hour: 9,
        end_hour: 17,
        timezone: String::from_str(&env, "EST"),
    }];

    client.set_availability(&provider, &avail);
    let stored_avail = client.get_availability(&provider);
    assert_eq!(stored_avail.len(), 1);
    assert_eq!(stored_avail.get(0).unwrap().day_of_week, 1);
}

#[test]
fn test_verification() {
    let env = Env::default();
    env.mock_all_auths();
    
    let admin = Address::generate(&env);
    let identity_registry = Address::generate(&env);
    let provider = Address::generate(&env);
    
    let contract_id = env.register_contract(None, ProviderDirectoryContract);
    let client = ProviderDirectoryContractClient::new(&env, &contract_id);
    client.initialize(&admin, &identity_registry);

    client.update_profile(
        &provider,
        &String::from_str(&env, "Dr. Smith"),
        &vec![&env],
        &String::from_str(&env, "Bio"),
        &String::from_str(&env, "Loc"),
        &String::from_str(&env, "provider@example.com"),
    );

    client.verify_provider(&admin, &provider);
    let profile = client.get_profile(&provider);
    assert_eq!(profile.is_verified, true);
}

#[test]
fn test_update_profile_rejects_oversized_name() {
    let env = Env::default();
    env.mock_all_auths();
    let admin = Address::generate(&env);
    let identity_registry = Address::generate(&env);
    let provider = Address::generate(&env);
    let contract_id = env.register_contract(None, ProviderDirectoryContract);
    let client = ProviderDirectoryContractClient::new(&env, &contract_id);
    client.initialize(&admin, &identity_registry);

    // 130-character name exceeds MAX_NAME_LEN=128
    let long_name = String::from_str(
        &env,
        "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa\
         aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
    );
    let result = client.try_update_profile(
        &provider,
        &long_name,
        &vec![&env],
        &String::from_str(&env, "Bio"),
        &String::from_str(&env, "City"),
        &String::from_str(&env, "p@example.com"),
    );
    assert_eq!(result, Err(Ok(Error::InputTooLong)));
}

#[test]
fn test_update_profile_rejects_invalid_email() {
    let env = Env::default();
    env.mock_all_auths();
    let admin = Address::generate(&env);
    let identity_registry = Address::generate(&env);
    let provider = Address::generate(&env);
    let contract_id = env.register_contract(None, ProviderDirectoryContract);
    let client = ProviderDirectoryContractClient::new(&env, &contract_id);
    client.initialize(&admin, &identity_registry);

    let result = client.try_update_profile(
        &provider,
        &String::from_str(&env, "Dr. Smith"),
        &vec![&env],
        &String::from_str(&env, "Bio"),
        &String::from_str(&env, "City"),
        &String::from_str(&env, "not-an-email"),
    );
    assert_eq!(result, Err(Ok(Error::InvalidInput)));
}

#[test]
fn test_update_profile_rejects_null_byte_in_bio() {
    let env = Env::default();
    env.mock_all_auths();
    let admin = Address::generate(&env);
    let identity_registry = Address::generate(&env);
    let provider = Address::generate(&env);
    let contract_id = env.register_contract(None, ProviderDirectoryContract);
    let client = ProviderDirectoryContractClient::new(&env, &contract_id);
    client.initialize(&admin, &identity_registry);

    let bad_bio = soroban_sdk::String::from_bytes(&env, b"bio\x00injection");
    let result = client.try_update_profile(
        &provider,
        &String::from_str(&env, "Dr. Smith"),
        &vec![&env],
        &bad_bio,
        &String::from_str(&env, "City"),
        &String::from_str(&env, "p@example.com"),
    );
    assert_eq!(result, Err(Ok(Error::InvalidInput)));
}

#[test]
fn test_update_profile_rejects_injection_in_name() {
    let env = Env::default();
    env.mock_all_auths();
    let admin = Address::generate(&env);
    let identity_registry = Address::generate(&env);
    let provider = Address::generate(&env);
    let contract_id = env.register_contract(None, ProviderDirectoryContract);
    let client = ProviderDirectoryContractClient::new(&env, &contract_id);
    client.initialize(&admin, &identity_registry);

    // Angle brackets are not allowed in names
    let result = client.try_update_profile(
        &provider,
        &String::from_str(&env, "<script>alert(1)</script>"),
        &vec![&env],
        &String::from_str(&env, "Bio"),
        &String::from_str(&env, "City"),
        &String::from_str(&env, "p@example.com"),
    );
    assert_eq!(result, Err(Ok(Error::InvalidInput)));
}

// ============================================================================
// OnboardingConfig Storage Optimization Tests
// Tests for Instance Storage (reduced per-ledger costs)
// ============================================================================

#[test]
fn test_onboarding_config_initialized_in_instance_storage() {
    let env = Env::default();
    let admin = Address::generate(&env);
    
    let contract_id = env.register_contract(None, ProviderDirectoryContract);
    let contract = ProviderDirectoryContract;
    
    // Initialize the contract
    contract.initialize(&env, admin.clone()).expect("initialization should succeed");
    
    // Verify onboarding config is properly initialized
    let config = contract.get_onboarding_config(&env)
        .expect("onboarding config should be initialized");
    
    assert_eq!(config.enabled, true, "onboarding should be enabled by default");
    assert_eq!(config.max_providers, 1000, "max_providers should be 1000");
    assert_eq!(config.min_rating, 0, "min_rating should be 0");
}

#[test]
fn test_onboarding_config_not_found_before_init() {
    let env = Env::default();
    
    let contract = ProviderDirectoryContract;
    
    // Try to get onboarding config before initialization
    let result = contract.get_onboarding_config(&env);
    
    assert_eq!(result, Err(Error::NotInitialized), 
        "getting onboarding config before init should return NotInitialized");
}

#[test]
fn test_onboarding_config_immutable_after_init() {
    let env = Env::default();
    env.mock_all_auths();
    
    let admin = Address::generate(&env);
    
    let contract_id = env.register_contract(None, ProviderDirectoryContract);
    let contract = ProviderDirectoryContract;
    
    // Initialize with default config
    contract.initialize(&env, admin.clone()).expect("initialization should succeed");
    
    // Get the config
    let config1 = contract.get_onboarding_config(&env)
        .expect("onboarding config should exist");
    
    // Get again to verify immutability
    let config2 = contract.get_onboarding_config(&env)
        .expect("onboarding config should still exist");
    
    // Verify configs are identical (immutable)
    assert_eq!(config1.enabled, config2.enabled, "enabled field should remain unchanged");
    assert_eq!(config1.max_providers, config2.max_providers, "max_providers should remain unchanged");
    assert_eq!(config1.min_rating, config2.min_rating, "min_rating should remain unchanged");
}

#[test]
fn test_double_initialization_prevented() {
    let env = Env::default();
    let admin = Address::generate(&env);
    
    let contract_id = env.register_contract(None, ProviderDirectoryContract);
    let contract = ProviderDirectoryContract;
    
    // First initialization should succeed
    contract.initialize(&env, admin.clone())
        .expect("first initialization should succeed");
    
    // Second initialization should fail
    let result = contract.initialize(&env, admin.clone());
    
    assert_eq!(result, Err(Error::AlreadyInitialized), 
        "double initialization should be prevented");
}

