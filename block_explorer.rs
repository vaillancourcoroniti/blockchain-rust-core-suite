use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use crate::blockchain_core::{ChainBlock, BlockchainCore};
use crate::transaction_pool::ChainTransaction;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockSummary {
    pub height: u64,
    pub hash: String,
    pub timestamp: u128,
    pub tx_count: usize,
    pub validator: String,
    pub size: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionSummary {
    pub tx_id: String,
    pub block_height: u64,
    pub sender: String,
    pub receiver: String,
    pub amount: u128,
    pub fee: u64,
    pub status: String,
}

pub struct BlockExplorer {
    block_index: HashMap<u64, BlockSummary>,
    tx_index: HashMap<String, TransactionSummary>,
    address_txs: HashMap<String, Vec<String>>,
}

impl BlockExplorer {
    pub fn new() -> Self {
        Self {
            block_index: HashMap::new(),
            tx_index: HashMap::new(),
            address_txs: HashMap::new(),
        }
    }

    pub fn index_block(&mut self, block: &ChainBlock, txs: &[ChainTransaction]) {
        let summary = BlockSummary {
            height: block.index,
            hash: block.hash.clone(),
            timestamp: block.timestamp,
            tx_count: txs.len(),
            validator: "system".to_string(),
            size: block.data.len(),
        };
        
        self.block_index.insert(block.index, summary);
        
        for tx in txs {
            let tx_summary = TransactionSummary {
                tx_id: tx.tx_id.clone(),
                block_height: block.index,
                sender: tx.sender.clone(),
                receiver: tx.receiver.clone(),
                amount: tx.amount,
                fee: tx.fee,
                status: "confirmed".to_string(),
            };
            
            self.tx_index.insert(tx.tx_id.clone(), tx_summary);
            self.address_txs.entry(tx.sender.clone()).or_default().push(tx.tx_id.clone());
            self.address_txs.entry(tx.receiver.clone()).or_default().push(tx.tx_id.clone());
        }
    }

    pub fn get_block_by_height(&self, height: u64) -> Option<&BlockSummary> {
        self.block_index.get(&height)
    }

    pub fn get_transaction(&self, tx_id: &str) -> Option<&TransactionSummary> {
        self.tx_index.get(tx_id)
    }

    pub fn get_address_transactions(&self, address: &str) -> Vec<&TransactionSummary> {
        self.address_txs.get(address)
            .unwrap_or(&vec![])
            .iter()
            .filter_map(|tx_id| self.tx_index.get(tx_id))
            .collect()
    }
}
