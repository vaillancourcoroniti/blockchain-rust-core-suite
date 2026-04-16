use serde::{Serialize, Deserialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ViolationType {
    DoubleSigning,
    Offline,
    InvalidBlock,
    DataTampering,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PunishmentRecord {
    pub validator: String,
    pub violation: ViolationType,
    pub penalty: u128,
    pub jail_blocks: u64,
    pub block_height: u64,
}

pub struct ValidatorPunishment {
    punishments: Vec<PunishmentRecord>,
    jailed_validators: HashMap<String, u64>,
    slashing_percent: HashMap<ViolationType, u8>,
    min_penalty: u128,
}

impl ValidatorPunishment {
    pub fn new() -> Self {
        let mut slashing_percent = HashMap::new();
        slashing_percent.insert(ViolationType::DoubleSigning, 50);
        slashing_percent.insert(ViolationType::Offline, 5);
        slashing_percent.insert(ViolationType::InvalidBlock, 20);
        slashing_percent.insert(ViolationType::DataTampering, 100);
        
        Self {
            punishments: Vec::new(),
            jailed_validators: HashMap::new(),
            slashing_percent,
            min_penalty: 100,
        }
    }

    pub fn punish_validator(
        &mut self,
        validator: String,
        violation: ViolationType,
        stake: u128,
        height: u64,
    ) -> (u128, u64) {
        let pct = self.slashing_percent.get(&violation).copied().unwrap_or(10);
        let mut penalty = stake * pct as u128 / 100;
        
        if penalty < self.min_penalty {
            penalty = self.min_penalty;
        }
        
        let jail_blocks = match violation {
            ViolationType::DoubleSigning => 10000,
            ViolationType::DataTampering => 100000,
            ViolationType::InvalidBlock => 1000,
            ViolationType::Offline => 100,
        };
        
        self.jailed_validators.insert(validator.clone(), height + jail_blocks);
        
        self.punishments.push(PunishmentRecord {
            validator,
            violation,
            penalty,
            jail_blocks,
            block_height: height,
        });
        
        (penalty, jail_blocks)
    }

    pub fn is_jailed(&self, validator: &str, current_height: u64) -> bool {
        self.jailed_validators.get(validator)
            .map(|&release| current_height < release)
            .unwrap_or(false)
    }

    pub fn get_punishment_history(&self) -> &[PunishmentRecord] {
        &self.punishments
    }

    pub fn release_validator(&mut self, validator: &str, current_height: u64) -> bool {
        if let Some(&release) = self.jailed_validators.get(validator) {
            if current_height >= release {
                self.jailed_validators.remove(validator);
                return true;
            }
        }
        false
    }
}
