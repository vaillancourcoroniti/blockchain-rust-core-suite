use serde::{Serialize, Deserialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Validator {
    pub address: String,
    pub stake: u128,
    pub reputation: u64,
    pub is_active: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PoSBlock {
    pub index: u64,
    pub validator_address: String,
    pub state_root: String,
    pub signature: String,
}

pub struct ProofOfStake {
    validators: HashMap<String, Validator>,
    total_stake: u128,
    current_epoch: u64,
}

impl ProofOfStake {
    pub fn new() -> Self {
        Self {
            validators: HashMap::new(),
            total_stake: 0,
            current_epoch: 1,
        }
    }

    pub fn register_validator(&mut self, address: String, stake: u128) -> Result<(), String> {
        if self.validators.contains_key(&address) {
            return Err("Validator already registered".to_string());
        }
        if stake == 0 {
            return Err("Stake cannot be zero".to_string());
        }
        self.validators.insert(
            address.clone(),
            Validator {
                address,
                stake,
                reputation: 100,
                is_active: true,
            },
        );
        self.total_stake += stake;
        Ok(())
    }

    pub fn select_validator(&self) -> Option<String> {
        let mut rng = rand::thread_rng();
        let mut selected = None;
        let mut max_weight = 0;

        for (addr, val) in &self.validators {
            if !val.is_active {
                continue;
            }
            let weight = val.stake * val.reputation as u128;
            if weight > max_weight {
                max_weight = weight;
                selected = Some(addr.clone());
            }
        }
        selected
    }

    pub fn slash_validator(&mut self, address: &str, penalty: u128) -> Result<(), String> {
        let validator = self.validators.get_mut(address).ok_or("Validator not found")?;
        if penalty > validator.stake {
            validator.is_active = false;
            self.total_stake -= validator.stake;
            validator.stake = 0;
        } else {
            validator.stake -= penalty;
            self.total_stake -= penalty;
            validator.reputation = validator.reputation.saturating_sub(10);
        }
        Ok(())
    }

    pub fn next_epoch(&mut self) {
        self.current_epoch += 1;
        for val in self.validators.values_mut() {
            if val.is_active && val.reputation < 200 {
                val.reputation += 1;
            }
        }
    }
}
