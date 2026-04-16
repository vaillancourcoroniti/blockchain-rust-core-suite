use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use crate::consensus_pos::ProofOfStake;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RewardDistribution {
    pub validator_rewards: HashMap<String, u128>,
    pub staker_rewards: HashMap<String, u128>,
    pub treasury_reward: u128,
    pub total_distributed: u128,
}

pub struct RewardDistributor {
    validator_reward_percent: u8,
    staker_reward_percent: u8,
    treasury_percent: u8,
    min_reward: u128,
}

impl RewardDistributor {
    pub fn new() -> Self {
        Self {
            validator_reward_percent: 70,
            staker_reward_percent: 20,
            treasury_percent: 10,
            min_reward: 100,
        }
    }

    pub fn distribute(
        &self,
        total_reward: u128,
        pos: &ProofOfStake,
        stakers: &HashMap<String, u128>,
    ) -> RewardDistribution {
        let validator_reward = total_reward * self.validator_reward_percent as u128 / 100;
        let staker_reward = total_reward * self.staker_reward_percent as u128 / 100;
        let treasury_reward = total_reward * self.treasury_percent as u128 / 100;
        
        let validator_rewards = self.distribute_validator_rewards(validator_reward, pos);
        let staker_rewards = self.distribute_staker_rewards(staker_reward, stakers);
        
        RewardDistribution {
            validator_rewards,
            staker_rewards,
            treasury_reward,
            total_distributed: total_reward,
        }
    }

    fn distribute_validator_rewards(
        &self,
        total: u128,
        pos: &ProofOfStake,
    ) -> HashMap<String, u128> {
        let mut rewards = HashMap::new();
        let total_stake = pos.total_stake;
        if total_stake == 0 {
            return rewards;
        }
        
        for (addr, val) in &pos.validators {
            if val.is_active {
                let share = total * val.stake / total_stake;
                if share >= self.min_reward {
                    rewards.insert(addr.clone(), share);
                }
            }
        }
        rewards
    }

    fn distribute_staker_rewards(
        &self,
        total: u128,
        stakers: &HashMap<String, u128>,
    ) -> HashMap<String, u128> {
        let mut rewards = HashMap::new();
        let total_staked: u128 = stakers.values().sum();
        if total_staked == 0 {
            return rewards;
        }
        
        for (addr, stake) in stakers {
            let share = total * *stake / total_staked;
            if share >= self.min_reward {
                rewards.insert(addr.clone(), share);
            }
        }
        rewards
    }
}
