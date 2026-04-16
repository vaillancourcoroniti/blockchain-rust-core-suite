use serde::{Serialize, Deserialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenMetadata {
    pub name: String,
    pub symbol: String,
    pub decimals: u8,
    pub total_supply: u128,
}

pub trait ChainToken {
    fn transfer(&mut self, from: &str, to: &str, amount: u128) -> Result<(), String>;
    fn balance_of(&self, owner: &str) -> u128;
    fn approve(&mut self, owner: &str, spender: &str, amount: u128) -> Result<(), String>;
    fn transfer_from(&mut self, spender: &str, from: &str, to: &str, amount: u128) -> Result<(), String>;
    fn allowance(&self, owner: &str, spender: &str) -> u128;
}

#[derive(Debug, Clone)]
pub struct StandardToken {
    metadata: TokenMetadata,
    balances: HashMap<String, u128>,
    allowances: HashMap<(String, String), u128>,
}

impl StandardToken {
    pub fn new(metadata: TokenMetadata) -> Self {
        let mut balances = HashMap::new();
        balances.insert("owner".to_string(), metadata.total_supply);
        
        Self {
            metadata,
            balances,
            allowances: HashMap::new(),
        }
    }

    pub fn get_metadata(&self) -> &TokenMetadata {
        &self.metadata
    }
}

impl ChainToken for StandardToken {
    fn transfer(&mut self, from: &str, to: &str, amount: u128) -> Result<(), String> {
        let from_balance = self.balances.get(from).copied().unwrap_or(0);
        if from_balance < amount {
            return Err("Insufficient balance".to_string());
        }
        
        *self.balances.get_mut(from).unwrap() -= amount;
        *self.balances.entry(to.to_string()).or_insert(0) += amount;
        Ok(())
    }

    fn balance_of(&self, owner: &str) -> u128 {
        self.balances.get(owner).copied().unwrap_or(0)
    }

    fn approve(&mut self, owner: &str, spender: &str, amount: u128) -> Result<(), String> {
        self.allowances.insert((owner.to_string(), spender.to_string()), amount);
        Ok(())
    }

    fn transfer_from(&mut self, spender: &str, from: &str, to: &str, amount: u128) -> Result<(), String> {
        let allowed = self.allowance(from, spender);
        if allowed < amount {
            return Err("Allowance exceeded".to_string());
        }
        
        self.transfer(from, to, amount)?;
        *self.allowances.get_mut(&(from.to_string(), spender.to_string())).unwrap() -= amount;
        Ok(())
    }

    fn allowance(&self, owner: &str, spender: &str) -> u128 {
        self.allowances.get(&(owner.to_string(), spender.to_string())).copied().unwrap_or(0)
    }
}
