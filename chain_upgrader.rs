use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChainUpgrade {
    pub upgrade_id: String,
    pub name: String,
    pub version: String,
    pub activation_height: u64,
    pub proposer: String,
    pub changes: Vec<String>,
    pub is_active: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum UpgradeStatus {
    Pending,
    Active,
    Completed,
    Failed,
}

pub struct ChainUpgrader {
    upgrades: HashMap<String, ChainUpgrade>,
    current_version: String,
    upgrade_threshold: u64,
}

impl ChainUpgrader {
    pub fn new(initial_version: &str) -> Self {
        Self {
            upgrades: HashMap::new(),
            current_version: initial_version.to_string(),
            upgrade_threshold: 67,
        }
    }

    pub fn schedule_upgrade(&mut self, upgrade: ChainUpgrade) -> Result<(), String> {
        if self.upgrades.contains_key(&upgrade.upgrade_id) {
            return Err("Upgrade already exists".to_string());
        }
        self.upgrades.insert(upgrade.upgrade_id.clone(), upgrade);
        Ok(())
    }

    pub fn check_activation(&mut self, current_height: u64) -> Vec<&ChainUpgrade> {
        let mut activated = Vec::new();
        
        for upgrade in self.upgrades.values_mut() {
            if !upgrade.is_active && upgrade.activation_height <= current_height {
                upgrade.is_active = true;
                self.current_version = upgrade.version.clone();
                activated.push(upgrade);
            }
        }
        
        activated
    }

    pub fn get_active_upgrades(&self) -> Vec<&ChainUpgrade> {
        self.upgrades.values()
            .filter(|u| u.is_active)
            .collect()
    }

    pub fn get_pending_upgrades(&self) -> Vec<&ChainUpgrade> {
        self.upgrades.values()
            .filter(|u| !u.is_active)
            .collect()
    }

    pub fn get_current_version(&self) -> &str {
        &self.current_version
    }
}
