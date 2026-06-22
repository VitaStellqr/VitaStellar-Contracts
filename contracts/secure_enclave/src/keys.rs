use soroban_sdk::{contracttype, BytesN, Env};

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct KeyState {
    pub current_key: BytesN<32>,
    pub previous_key: BytesN<32>,
    pub has_previous_key: bool,
    pub previous_expires_at: u64,
}

#[contracttype]
pub enum DataKey {
    SigningKeyState,
}

impl KeyState {
    /// Initializes the state engine with a primary genesis key.
    pub fn init(env: &Env, key: BytesN<32>) -> Self {
        let zero_key = BytesN::from_array(env, &[0u8; 32]);
        let state = KeyState {
            current_key: key,
            previous_key: zero_key,
            has_previous_key: false,
            previous_expires_at: 0,
        };
        env.storage()
            .instance()
            .set(&DataKey::SigningKeyState, &state);
        state
    }

    /// Loads the active state configuration from contract instance storage.
    pub fn load(env: &Env) -> Option<Self> {
        env.storage().instance().get(&DataKey::SigningKeyState)
    }

    /// Atomically rotates the keys and sets an absolute ledger expiration timestamp.
    pub fn rotate(&mut self, env: &Env, new_key: BytesN<32>, grace_period: u64) {
        let current_ledger_time = env.ledger().timestamp();

        // Safe atomic migration using structural flags
        self.previous_key = self.current_key.clone();
        self.has_previous_key = true;
        self.previous_expires_at = current_ledger_time.saturating_add(grace_period);
        self.current_key = new_key;

        env.storage()
            .instance()
            .set(&DataKey::SigningKeyState, self);
    }

    /// Assesses whether a given public key is valid under current ledger state rules.
    pub fn verify_key_validity(&self, env: &Env, public_key: &BytesN<32>) -> bool {
        // Condition 1: Always accept the current key
        if &self.current_key == public_key {
            return true;
        }

        // Condition 2: Evaluate the previous key against ledger time boundaries if active
        if self.has_previous_key && &self.previous_key == public_key {
            let current_ledger_time = env.ledger().timestamp();
            return current_ledger_time < self.previous_expires_at;
        }

        false
    }

    /// Purges the historical tracking slot if it has passed its expiration deadline.
    pub fn purge_expired(&mut self, env: &Env) -> bool {
        if self.has_previous_key && env.ledger().timestamp() >= self.previous_expires_at {
            let zero_key = BytesN::from_array(env, &[0u8; 32]);
            self.previous_key = zero_key;
            self.has_previous_key = false;
            self.previous_expires_at = 0;
            env.storage()
                .instance()
                .set(&DataKey::SigningKeyState, self);
            return true; // Pruned
        }
        false
    }
}
