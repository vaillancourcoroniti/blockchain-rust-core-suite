use serde::{Serialize, Deserialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockReward {
    pub block_height: u64,
    pub validator_reward: u128,
    pub community_reward: u128,
    pub total_reward: u128,
    pub halving_count: u8,
}

pub struct BlockRewardCalculator {
    base_reward: u128,
    halving_interval: u64,
    community_percent: u8,
    min_reward: u128,
}

impl BlockRewardCalculator {
    pub fn new(base: u128, halving_blocks: u64, community_pct: u8) -> Self {
        Self {
            base_reward: base,
            halving_interval: halving_blocks,
            community_percent: community_pct,
            min_reward: 10,
        }
    }

    pub fn calculate_reward(&self, height: u64) -> BlockReward {
        let halving_count = height / self.halving_interval;
        let mut current_reward = self.base_reward;
        
        for _ in 0..halving_count {
            current_reward = current_reward.saturating_div(2);
        }
        
        if current_reward < self.min_reward {
            current_reward = self.min_reward;
        }
        
        let community = current_reward * self.community_percent as u128 / 100;
        let validator = current_reward - community;
        
        BlockReward {
            block_height: height,
            validator_reward: validator,
            community_reward: community,
            total_reward: current_reward,
            halving_count: halving_count as u8,
        }
    }

    pub fn get_current_reward(&self, height: u64) -> u128 {
        self.calculate_reward(height).total_reward
    }

    pub fn estimate_total_supply(&self, max_height: u64) -> u128 {
        let mut total = 0;
        let mut height = 1;
        while height <= max_height {
            total += self.get_current_reward(height);
            height += 1;
        }
        total
    }
}
