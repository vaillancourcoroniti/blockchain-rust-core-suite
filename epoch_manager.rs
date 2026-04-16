use serde::{Serialize, Deserialize};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Epoch {
    pub epoch_number: u64,
    pub start_time: u128,
    pub end_time: u128,
    pub block_height: u64,
    pub validator_set: Vec<String>,
    pub reward_pool: u128,
}

impl Epoch {
    pub fn new(number: u64, duration_ms: u128) -> Self {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis();
        
        Self {
            epoch_number: number,
            start_time: now,
            end_time: now + duration_ms,
            block_height: 0,
            validator_set: Vec::new(),
            reward_pool: 0,
        }
    }
}

pub struct EpochManager {
    current_epoch: Epoch,
    epoch_duration: u128,
    history: Vec<Epoch>,
}

impl EpochManager {
    pub fn new(epoch_duration_minutes: u64) -> Self {
        let duration = epoch_duration_minutes as u128 * 60 * 1000;
        let current = Epoch::new(1, duration);
        
        Self {
            current_epoch: current,
            epoch_duration: duration,
            history: Vec::new(),
        }
    }

    pub fn check_epoch_ended(&self) -> bool {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis();
        now > self.current_epoch.end_time
    }

    pub fn next_epoch(&mut self, validator_set: Vec<String>, reward_pool: u128) {
        let mut next = Epoch::new(
            self.current_epoch.epoch_number + 1,
            self.epoch_duration
        );
        next.validator_set = validator_set;
        next.reward_pool = reward_pool;
        
        let old = std::mem::replace(&mut self.current_epoch, next);
        self.history.push(old);
    }

    pub fn update_block_height(&mut self, height: u64) {
        self.current_epoch.block_height = height;
    }

    pub fn get_current_epoch(&self) -> &Epoch {
        &self.current_epoch
    }

    pub fn get_epoch_history(&self) -> &[Epoch] {
        &self.history
    }
}
