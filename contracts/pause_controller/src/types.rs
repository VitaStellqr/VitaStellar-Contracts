use soroban_sdk::{contracttype, Address, Symbol};

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RegisteredContract {
    pub name: Symbol,
    pub address: Address,
    pub method: Symbol,
}
