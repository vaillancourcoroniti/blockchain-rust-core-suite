use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ChainType {
    Ethereum,
    Solana,
    Bitcoin,
    Custom(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrossChainTransfer {
    pub transfer_id: String,
    pub source_chain: ChainType,
    pub target_chain: ChainType,
    pub sender: String,
    pub receiver: String,
    pub amount: u128,
    pub asset_symbol: String,
    pub status: TransferStatus,
    pub timestamp: u128,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TransferStatus {
    Pending,
    Locked,
    Bridged,
    Completed,
    Failed,
}

pub struct CrossChainBridge {
    transfers: HashMap<String, CrossChainTransfer>,
    chain_relays: HashMap<ChainType, String>,
    locked_assets: HashMap<(ChainType, String), u128>,
}

impl CrossChainBridge {
    pub fn new() -> Self {
        Self {
            transfers: HashMap::new(),
            chain_relays: HashMap::new(),
            locked_assets: HashMap::new(),
        }
    }

    pub fn register_relay(&mut self, chain: ChainType, relay_address: String) {
        self.chain_relays.insert(chain, relay_address);
    }

    pub fn initiate_transfer(&mut self, mut transfer: CrossChainTransfer) -> String {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis();
        transfer.timestamp = now;
        transfer.status = TransferStatus::Pending;
        
        let id = transfer.transfer_id.clone();
        self.transfers.insert(id.clone(), transfer);
        id
    }

    pub fn update_transfer_status(&mut self, id: &str, status: TransferStatus) -> Result<(), String> {
        let transfer = self.transfers.get_mut(id).ok_or("Transfer not found")?;
        transfer.status = status;
        
        if status == TransferStatus::Locked {
            let key = (transfer.source_chain.clone(), transfer.asset_symbol.clone());
            *self.locked_assets.entry(key).or_insert(0) += transfer.amount;
        }
        Ok(())
    }

    pub fn get_transfer(&self, id: &str) -> Option<&CrossChainTransfer> {
        self.transfers.get(id)
    }
}
