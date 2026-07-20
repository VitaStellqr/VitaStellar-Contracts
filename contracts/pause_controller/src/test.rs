use super::*;
use soroban_sdk::{
    contract, contractimpl, testutils::Address as _, testutils::Ledger, Address, Env, Symbol,
};

fn set_paused_sym(env: &Env) -> Symbol {
    Symbol::new(env, "set_paused")
}

// ---------------------------------------------------------------------------
// Mock pausable contract used to test cross-contract pause / unpause calls.
// Returns () to be compatible with invoke_contract::<()>.
// ---------------------------------------------------------------------------

#[contract]
struct MockPausable;

#[contractimpl]
impl MockPausable {
    pub fn initialize(env: Env, admin: Address) {
        env.storage().instance().set(&symbol_short!("admin"), &admin);
    }

    pub fn set_paused(env: Env, _caller: Address, paused: bool) {
        env.storage().instance().set(&symbol_short!("paused"), &paused);
        env.events()
            .publish((symbol_short!("mock_set"),), (paused,));
    }

    pub fn pause(env: Env, _caller: Address) {
        env.storage().instance().set(&symbol_short!("paused"), &true);
        env.events()
            .publish((symbol_short!("mck_pause"),), ());
    }

    pub fn unpause(env: Env, _caller: Address) {
        env.storage().instance().set(&symbol_short!("paused"), &false);
        env.events()
            .publish((symbol_short!("mck_unp"),), ());
    }

    pub fn is_paused(env: Env) -> bool {
        env.storage()
            .instance()
            .get(&symbol_short!("paused"))
            .unwrap_or(false)
    }
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn setup() -> (Env, Address, PauseControllerClient<'static>) {
    let env = Env::default();
    env.mock_all_auths();
    let admin = Address::generate(&env);
    let contract_id = env.register_contract(None, PauseController);
    let client = PauseControllerClient::new(&env, &contract_id);
    client.initialize(&admin);
    (env, admin, client)
}

fn deploy_mock(env: &Env, admin: &Address) -> (Address, MockPausableClient<'static>) {
    let addr = env.register_contract(None, MockPausable);
    let client = MockPausableClient::new(env, &addr);
    client.initialize(admin);
    (addr, client)
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[test]
fn test_initialize() {
    let (_, _, client) = setup();
    let admin2 = Address::generate(&client.env);
    assert_eq!(
        client.try_initialize(&admin2),
        Err(Ok(Error::AlreadyInitialized))
    );
}

#[test]
fn test_get_admin() {
    let (_, admin, client) = setup();
    assert_eq!(client.get_admin(), admin);
}

#[test]
fn test_register_contract() {
    let (env, admin, client) = setup();
    let (mock_addr, _) = deploy_mock(&env, &admin);
    let name = symbol_short!("MOCK");
    let sp = set_paused_sym(&env);

    assert!(client
        .try_register_contract(&admin, &name, &mock_addr, &sp)
        .is_ok());

    let registry = client.get_registered_contracts();
    assert_eq!(registry.len(), 1);
    assert_eq!(registry.get(0).unwrap().name, name);
}

#[test]
fn test_register_duplicate() {
    let (env, admin, client) = setup();
    let (mock_addr, _) = deploy_mock(&env, &admin);
    let name = symbol_short!("MOCK");
    let sp = set_paused_sym(&env);

    assert!(client
        .try_register_contract(&admin, &name, &mock_addr, &sp)
        .is_ok());
    assert_eq!(
        client.try_register_contract(&admin, &name, &mock_addr, &sp),
        Err(Ok(Error::AlreadyRegistered))
    );
}

#[test]
fn test_unregister_contract() {
    let (env, admin, client) = setup();
    let (mock_addr, _) = deploy_mock(&env, &admin);
    let name = symbol_short!("MOCK");
    let sp = set_paused_sym(&env);

    client.register_contract(&admin, &name, &mock_addr, &sp);
    assert!(client.try_unregister_contract(&admin, &name).is_ok());
    assert_eq!(client.get_registered_contracts().len(), 0);
}

#[test]
fn test_unregister_not_found() {
    let (_, admin, client) = setup();
    let name = symbol_short!("NOPE");
    assert_eq!(
        client.try_unregister_contract(&admin, &name),
        Err(Ok(Error::NotFound))
    );
}

#[test]
fn test_pause_all_with_set_paused() {
    let (env, admin, client) = setup();
    let (mock_addr, mock_client) = deploy_mock(&env, &admin);
    let name = symbol_short!("MOCK");
    let sp = set_paused_sym(&env);

    client.register_contract(&admin, &name, &mock_addr, &sp);

    client.pause_all(&admin);

    assert!(client.is_system_paused());
    assert!(mock_client.is_paused());
}

#[test]
fn test_pause_all_with_pause_method() {
    let (env, admin, client) = setup();
    let (mock_addr, mock_client) = deploy_mock(&env, &admin);
    let name = symbol_short!("MOCK2");

    client.register_contract(&admin, &name, &mock_addr, &symbol_short!("pause"));

    client.pause_all(&admin);

    assert!(client.is_system_paused());
    assert!(mock_client.is_paused());
}

#[test]
fn test_pause_all_already_paused() {
    let (env, admin, client) = setup();
    let (mock_addr, _) = deploy_mock(&env, &admin);
    let name = symbol_short!("MOCK");
    let sp = set_paused_sym(&env);

    client.register_contract(&admin, &name, &mock_addr, &sp);
    client.pause_all(&admin);

    assert_eq!(
        client.try_pause_all(&admin),
        Err(Ok(Error::AlreadyPaused))
    );
}

#[test]
fn test_unpause_all_schedules_and_execute() {
    let (env, admin, client) = setup();
    let (mock_addr, mock_client) = deploy_mock(&env, &admin);
    let name = symbol_short!("MOCK");
    let sp = set_paused_sym(&env);

    client.register_contract(&admin, &name, &mock_addr, &sp);

    // Pause first
    client.pause_all(&admin);
    assert!(client.is_system_paused());

    // Schedule unpause with 10s delay
    client.unpause_all(&admin, &10);
    assert!(client.get_unpause_eta() > 0);

    // Cannot execute yet (timelock not elapsed)
    assert_eq!(
        client.try_execute_unpause(&admin),
        Err(Ok(Error::TimelockNotElapsed))
    );

    // Advance ledger time past the delay
    env.ledger().set(soroban_sdk::testutils::LedgerInfo {
        timestamp: env.ledger().timestamp() + 15,
        ..Default::default()
    });

    // Now execute succeeds
    client.execute_unpause(&admin);
    assert!(!client.is_system_paused());
    assert!(!mock_client.is_paused());
}

#[test]
fn test_unpause_all_not_paused() {
    let (_, admin, client) = setup();
    assert_eq!(
        client.try_unpause_all(&admin, &10),
        Err(Ok(Error::NotPaused))
    );
}

#[test]
fn test_execute_unpause_not_scheduled() {
    let (env, admin, client) = setup();
    let (mock_addr, _) = deploy_mock(&env, &admin);
    let name = symbol_short!("MOCK");
    let sp = set_paused_sym(&env);

    client.register_contract(&admin, &name, &mock_addr, &sp);
    client.pause_all(&admin);

    // No unpause scheduled
    assert_eq!(
        client.try_execute_unpause(&admin),
        Err(Ok(Error::UnpauseNotScheduled))
    );
}

#[test]
fn test_unauthorized() {
    let (env, admin, client) = setup();
    let (mock_addr, _) = deploy_mock(&env, &admin);
    let name = symbol_short!("MOCK");
    let sp = set_paused_sym(&env);
    let other = Address::generate(&env);

    assert_eq!(
        client.try_register_contract(&other, &name, &mock_addr, &sp),
        Err(Ok(Error::Unauthorized))
    );
}

#[test]
fn test_get_registered_contracts_empty() {
    let (_, _, client) = setup();
    assert_eq!(client.get_registered_contracts().len(), 0);
}

#[test]
fn test_multiple_contracts_pause() {
    let (env, admin, client) = setup();

    let (mock1_addr, mock1_client) = deploy_mock(&env, &admin);
    let (mock2_addr, mock2_client) = deploy_mock(&env, &admin);
    let sp = set_paused_sym(&env);

    client.register_contract(&admin, &symbol_short!("M1"), &mock1_addr, &sp);
    client.register_contract(
        &admin,
        &symbol_short!("M2"),
        &mock2_addr,
        &symbol_short!("pause"),
    );

    client.pause_all(&admin);

    assert!(client.is_system_paused());
    assert!(mock1_client.is_paused());
    assert!(mock2_client.is_paused());

    // Advance time and unpause
    client.unpause_all(&admin, &10);
    env.ledger().set(soroban_sdk::testutils::LedgerInfo {
        timestamp: env.ledger().timestamp() + 20,
        ..Default::default()
    });
    client.execute_unpause(&admin);

    assert!(!client.is_system_paused());
    assert!(!mock1_client.is_paused());
    assert!(!mock2_client.is_paused());
}
