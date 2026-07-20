#![no_std]

use soroban_sdk::{
    contract, contracterror, contractimpl, contracttype, symbol_short, Address, Env, Map, Symbol,
};

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum Error {
    AlreadyInitialized = 1,
    Unauthorized = 2,
    NameNotFound = 3,
}

#[contracttype]
pub enum DataKey {
    Admin,
    Registry,
}

#[contract]
pub struct ContractRegistry;

#[contractimpl]
impl ContractRegistry {
    /// Initialize the registry with an admin.
    pub fn initialize(env: Env, admin: Address) -> Result<(), Error> {
        if env.storage().instance().has(&DataKey::Admin) {
            return Err(Error::AlreadyInitialized);
        }
        env.storage().instance().set(&DataKey::Admin, &admin);
        let registry: Map<Symbol, Address> = Map::new(&env);
        env.storage().instance().set(&DataKey::Registry, &registry);
        Ok(())
    }

    /// Register or update a contract address (admin-only).
    pub fn set(env: Env, caller: Address, name: Symbol, address: Address) -> Result<(), Error> {
        caller.require_auth();
        Self::require_admin(&env, &caller)?;

        let mut registry: Map<Symbol, Address> = env
            .storage()
            .instance()
            .get(&DataKey::Registry)
            .unwrap_or(Map::new(&env));
        registry.set(name.clone(), address.clone());
        env.storage().instance().set(&DataKey::Registry, &registry);

        env.events().publish(
            (symbol_short!("REG"), symbol_short!("SET")),
            (name, address),
        );
        Ok(())
    }

    /// Batch-register multiple contracts (admin-only).
    pub fn set_many(
        env: Env,
        caller: Address,
        names: soroban_sdk::Vec<Symbol>,
        addresses: soroban_sdk::Vec<Address>,
    ) -> Result<(), Error> {
        caller.require_auth();
        Self::require_admin(&env, &caller)?;

        if names.len() != addresses.len() {
            return Err(Error::NameNotFound);
        }

        let mut registry: Map<Symbol, Address> = env
            .storage()
            .instance()
            .get(&DataKey::Registry)
            .unwrap_or(Map::new(&env));

        for i in 0..names.len() {
            let name = names.get(i).unwrap();
            let address = addresses.get(i).unwrap();
            registry.set(name, address);
        }

        env.storage().instance().set(&DataKey::Registry, &registry);

        env.events()
            .publish((symbol_short!("REG"), symbol_short!("BATCH")), names.len());
        Ok(())
    }

    /// Look up a contract address by name. Returns None if not registered.
    pub fn get(env: Env, name: Symbol) -> Option<Address> {
        let registry: Map<Symbol, Address> = env
            .storage()
            .instance()
            .get(&DataKey::Registry)
            .unwrap_or(Map::new(&env));
        registry.get(name)
    }

    /// Check if a name is registered.
    pub fn has(env: Env, name: Symbol) -> bool {
        let registry: Map<Symbol, Address> = env
            .storage()
            .instance()
            .get(&DataKey::Registry)
            .unwrap_or(Map::new(&env));
        registry.contains_key(name)
    }

    /// Return all registered names and addresses.
    pub fn get_all(env: Env) -> Map<Symbol, Address> {
        env.storage()
            .instance()
            .get(&DataKey::Registry)
            .unwrap_or(Map::new(&env))
    }

    fn require_admin(env: &Env, actor: &Address) -> Result<(), Error> {
        let admin: Address = env
            .storage()
            .instance()
            .get(&DataKey::Admin)
            .ok_or(Error::Unauthorized)?;
        if admin != *actor {
            return Err(Error::Unauthorized);
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use soroban_sdk::testutils::Address as _;

    fn setup() -> (Env, Address) {
        let env = Env::default();
        env.mock_all_auths();
        let admin = Address::generate(&env);
        let contract_id = env.register_contract(None, ContractRegistry);
        let client = ContractRegistryClient::new(&env, &contract_id);
        client.initialize(&admin);
        (env, admin)
    }

    #[test]
    fn test_initialize() {
        let (env, _) = setup();
        let contract_id = env.register_contract(None, ContractRegistry);
        let client = ContractRegistryClient::new(&env, &contract_id);
        assert!(!client.has(&symbol_short!("anything")));
    }

    #[test]
    fn test_initialize_cannot_double_init() {
        let (env, admin) = setup();
        let contract_id = env.register_contract(None, ContractRegistry);
        let client = ContractRegistryClient::new(&env, &contract_id);
        client.initialize(&admin);
        let result = client.try_initialize(&admin);
        assert_eq!(result, Err(Ok(Error::AlreadyInitialized)));
    }

    #[test]
    fn test_set_and_get() {
        let (env, admin) = setup();
        let contract_id = env.register_contract(None, ContractRegistry);
        let client = ContractRegistryClient::new(&env, &contract_id);
        client.initialize(&admin);
        let addr = Address::generate(&env);
        client.set(&admin, &symbol_short!("governor"), &addr);
        assert_eq!(client.get(&symbol_short!("governor")), Some(addr));
    }

    #[test]
    fn test_set_unauthorized() {
        let (env, _) = setup();
        let contract_id = env.register_contract(None, ContractRegistry);
        let client = ContractRegistryClient::new(&env, &contract_id);
        let admin = Address::generate(&env);
        client.initialize(&admin);
        let nobody = Address::generate(&env);
        let addr = Address::generate(&env);
        let result = client.try_set(&nobody, &symbol_short!("governor"), &addr);
        assert_eq!(result, Err(Ok(Error::Unauthorized)));
    }

    #[test]
    fn test_has() {
        let (env, admin) = setup();
        let contract_id = env.register_contract(None, ContractRegistry);
        let client = ContractRegistryClient::new(&env, &contract_id);
        client.initialize(&admin);
        let addr = Address::generate(&env);
        assert!(!client.has(&symbol_short!("escrow")));
        client.set(&admin, &symbol_short!("escrow"), &addr);
        assert!(client.has(&symbol_short!("escrow")));
    }

    #[test]
    fn test_set_many() {
        let (env, admin) = setup();
        let contract_id = env.register_contract(None, ContractRegistry);
        let client = ContractRegistryClient::new(&env, &contract_id);
        client.initialize(&admin);
        let addr1 = Address::generate(&env);
        let addr2 = Address::generate(&env);
        let names = soroban_sdk::vec![&env, symbol_short!("gov"), symbol_short!("esc")];
        let addresses = soroban_sdk::vec![&env, addr1.clone(), addr2.clone()];
        client.set_many(&admin, &names, &addresses);
        assert_eq!(client.get(&symbol_short!("gov")), Some(addr1));
        assert_eq!(client.get(&symbol_short!("esc")), Some(addr2));
    }

    #[test]
    fn test_get_all() {
        let (env, admin) = setup();
        let contract_id = env.register_contract(None, ContractRegistry);
        let client = ContractRegistryClient::new(&env, &contract_id);
        client.initialize(&admin);
        let addr = Address::generate(&env);
        client.set(&admin, &symbol_short!("test"), &addr);
        let all = client.get_all();
        assert_eq!(all.len(), 1);
        assert_eq!(all.get(symbol_short!("test")), Some(addr));
    }

    #[test]
    fn test_overwrite_existing() {
        let (env, admin) = setup();
        let contract_id = env.register_contract(None, ContractRegistry);
        let client = ContractRegistryClient::new(&env, &contract_id);
        client.initialize(&admin);
        let addr1 = Address::generate(&env);
        let addr2 = Address::generate(&env);
        client.set(&admin, &symbol_short!("key"), &addr1);
        assert_eq!(client.get(&symbol_short!("key")), Some(addr1));
        client.set(&admin, &symbol_short!("key"), &addr2);
        assert_eq!(client.get(&symbol_short!("key")), Some(addr2));
    }
}
