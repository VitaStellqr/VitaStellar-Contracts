use soroban_sdk::{contracterror, symbol_short, Symbol};

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum Error {
    Unauthorized = 100,
    AlreadyInitialized = 301,
    NotInitialized = 300,
    MismatchedLength = 230,
}

pub fn get_suggestion(error: Error) -> Symbol {
    match error {
        Error::Unauthorized => symbol_short!("CHK_AUTH"),
        Error::AlreadyInitialized => symbol_short!("ALREADY"),
        Error::NotInitialized => symbol_short!("INIT_CTR"),
        Error::MismatchedLength => symbol_short!("CHK_LEN"),
        _ => symbol_short!("CONTACT"),
    }
}
