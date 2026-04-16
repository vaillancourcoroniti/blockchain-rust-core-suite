use std::collections::HashMap;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StakePosition {
    pub amount: u128,
    pub unlock_time: u128,
    pub reward_debt: u128,
    pub is_locked: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PoolStats {
    pub total_staked: u128,
    pub total_rewards: u128,
    pub reward_rate: u128,
    pub staker_count: usize,
}

pub struct StakingPool {
    stakes: HashMap<String, StakePosition>,
    total_staked: u128,
    reward_rate_per_second: u128,
    min_stake: u128,
    lock_period: u128,
}

impl StakingPool {
    pub fn new(reward_rate: u128, lock_hours: u64) -> Self {
        Self {
            stakes: HashMap::new(),
            total_staked: 0,
            reward_rate_per_second: reward_rate,
            min_stake: 100,
            lock_period: lock_hours as u128 * 3600 * 1000,
        }
    }

    pub fn stake(&mut self, user: String, amount: u128) -> Result<(), String> {
        if amount < self.min_stake {
            return Err("Stake below minimum".to_string());
        }
        
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis();
        
        let unlock_time = now + self.lock_period;
        
        let stake = self.stakes.entry(user).or_insert_with(|| StakePosition {
            amount: 0,
            unlock_time: 0,
            reward_debt: 0,
            is_locked: true,
        });
        
        stake.amount += amount;
        stake.unlock_time = unlock_time;
        stake.is_locked = true;
        self.total_staked += amount;
        
        Ok(())
    }

    pub fn unstake(&mut self, user: &str) -> Result<u128, String> {
        let stake = self.stakes.get_mut(user).ok_or("No stake found")?;
        if stake.is_locked {
            return Err("Stake still locked".to_string());
        }
        
        let amount = stake.amount;
        stake.amount = 0;
        self.total_staked -= amount;
        Ok(amount)
    }

    pub fn claim_rewards(&mut self, user: &str) -> Result<u128, String> {
        let stake = self.stakes.get_mut(user).ok_or("No stake found")?;
        let rewards = stake.reward_debt;
        stake.reward_debt = 0;
        Ok(rewards)
    }

    pub fn get_stats(&self) -> PoolStats {
        PoolStats {
            total_staked: self.total_staked,
            total_rewards: 0,
            reward_rate: self.reward_rate_per_second,
            staker_count: self.stakes.len(),
        }
    }
}
