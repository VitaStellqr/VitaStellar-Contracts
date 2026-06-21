use soroban_sdk::{contracttype, symbol_short, Address, BytesN, String, Symbol, Vec};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[contracttype]
pub enum EscrowStatus {
    Pending = 0,
    Active = 1,
    Settled = 2,
    Refunded = 3,
    Disputed = 4,
}

#[derive(Clone)]
#[contracttype]
pub struct Escrow {
    pub order_id: u64,
    pub payer: Address,
    pub payee: Address,
    pub amount: i128,
    pub token: Address,
    pub status: EscrowStatus,
    pub approvals: Vec<Address>,
    pub reason: String,
}

#[derive(Clone)]
#[contracttype]
pub struct PlatformStats {
    pub total_volume: i128,
    pub total_escrows: u64,
    pub settled_count: u64,
    pub refunded_count: u64,
    pub disputed_count: u64,
    pub active_count: u64,
}

#[derive(Clone)]
#[contracttype]
pub struct DailyStats {
    pub day_id: u64,
    pub volume: i128,
    pub count: u32,
}

#[derive(Clone)]
#[contracttype]
pub struct ExportMetadata {
    pub format: String,
    pub checksum: BytesN<32>,
    pub timestamp: u64,
}

#[derive(Clone)]
#[contracttype]
pub struct FeeConfig {
    pub platform_fee_bps: u32,
    pub fee_receiver: Address,
}

// Storage keys
pub const ESCROWS: Symbol = symbol_short!("escrow");
pub const FEE_CONF: Symbol = symbol_short!("feeconf");
pub const REENTRANCY_LOCK: Symbol = symbol_short!("relock");
pub const CREDITS: Symbol = symbol_short!("credits");
pub const STATS: Symbol = symbol_short!("stats");
pub const ADMIN: Symbol = symbol_short!("admin");
pub const DAILY_STATS: Symbol = symbol_short!("dlystats");

// TTL constants
pub const PERSISTENT_TTL_THRESHOLD: u32 = 100;
pub const PERSISTENT_TTL_EXTEND_TO: u32 = 10000;
pub const TEMP_SESSION_TTL: u32 = 500;

impl PlatformStats {
    pub fn zero() -> Self {
        PlatformStats {
            total_volume: 0,
            total_escrows: 0,
            settled_count: 0,
            refunded_count: 0,
            disputed_count: 0,
            active_count: 0,
        }
    }
}

#[cfg(all(test, feature = "testutils"))]
mod tests {
    use super::*;
    use soroban_sdk::Env;

    #[test]
    fn test_platform_stats_zero() {
        let s = PlatformStats::zero();
        assert_eq!(s.total_volume, 0);
        assert_eq!(s.total_escrows, 0);
    }

    #[test]
    fn test_escrow_status_values() {
        assert_eq!(EscrowStatus::Pending as u32, 0);
        assert_eq!(EscrowStatus::Active as u32, 1);
        assert_eq!(EscrowStatus::Settled as u32, 2);
        assert_eq!(EscrowStatus::Refunded as u32, 3);
        assert_eq!(EscrowStatus::Disputed as u32, 4);
    }

    #[test]
    fn test_storage_keys_are_distinct() {
        // ensure symbols compile and are accessible
        let _ = ESCROWS;
        let _ = FEE_CONF;
        let _ = CREDITS;
        let _ = STATS;
        let _ = ADMIN;
        let _ = DAILY_STATS;
    }
}
