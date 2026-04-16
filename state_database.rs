use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use std::fs::{File, create_dir_all};
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountState {
    pub balance: u128,
    pub nonce: u64,
    pub code_hash: String,
    pub storage_root: String,
}

impl AccountState {
    pub fn new() -> Self {
        Self {
            balance: 0,
            nonce: 0,
            code_hash: "0".to_string(),
            storage_root: "0".to_string(),
        }
    }
}

pub struct StateDatabase {
    accounts: HashMap<String, AccountState>,
    contract_storage: HashMap<String, HashMap<Vec<u8>, Vec<u8>>>,
    db_path: String,
}

impl StateDatabase {
    pub fn new(path: &str) -> Self {
        if !Path::new(path).exists() {
            create_dir_all(path).unwrap();
        }
        
        Self {
            accounts: HashMap::new(),
            contract_storage: HashMap::new(),
            db_path: path.to_string(),
        }
    }

    pub fn get_account(&self, address: &str) -> AccountState {
        self.accounts.get(address).cloned().unwrap_or_else(AccountState::new)
    }

    pub fn update_account(&mut self, address: String, state: AccountState) {
        self.accounts.insert(address, state);
    }

    pub fn set_contract_storage(&mut self, contract: String, key: Vec<u8>, value: Vec<u8>) {
        let storage = self.contract_storage.entry(contract).or_insert_with(HashMap::new);
        storage.insert(key, value);
    }

    pub fn get_contract_storage(&self, contract: &str, key: &[u8]) -> Option<Vec<u8>> {
        self.contract_storage.get(contract)?.get(key).cloned()
    }

    pub fn commit(&self) -> Result<(), String> {
        let file = File::create(format!("{}/state_snapshot.json", self.db_path))
            .map_err(|e| e.to_string())?;
        serde_json::to_writer_pretty(file, &self.accounts)
            .map_err(|e| e.to_string())?;
        Ok(())
    }
}
