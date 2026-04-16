use std::collections::{HashMap, BTreeSet};
use serde::{Serialize, Deserialize};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct ChainTransaction {
    pub tx_id: String,
    pub sender: String,
    pub receiver: String,
    pub amount: u128,
    pub fee: u64,
    pub nonce: u64,
    pub timestamp: u128,
    pub signature: String,
}

impl ChainTransaction {
    pub fn new(
        sender: String,
        receiver: String,
        amount: u128,
        fee: u64,
        nonce: u64,
        signature: String,
    ) -> Self {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis();
        let tx_id = Self::generate_tx_id(&sender, &receiver, amount, nonce, timestamp);
        
        Self {
            tx_id,
            sender,
            receiver,
            amount,
            fee,
            nonce,
            timestamp,
            signature,
        }
    }

    fn generate_tx_id(sender: &str, receiver: &str, amount: u128, nonce: u64, ts: u128) -> String {
        use sha2::{Sha256, Digest};
        let input = format!("{}{}{}{}{}", sender, receiver, amount, nonce, ts);
        let mut hasher = Sha256::new();
        hasher.update(input.as_bytes());
        hex::encode(hasher.finalize())
    }
}

pub struct TransactionPool {
    transactions: HashMap<String, ChainTransaction>,
    ordered_by_fee: BTreeSet<ChainTransaction>,
    max_pool_size: usize,
}

impl TransactionPool {
    pub fn new(max_size: usize) -> Self {
        Self {
            transactions: HashMap::new(),
            ordered_by_fee: BTreeSet::new(),
            max_pool_size: max_size,
        }
    }

    pub fn add_transaction(&mut self, tx: ChainTransaction) -> bool {
        if self.transactions.contains_key(&tx.tx_id) || self.transactions.len() >= self.max_pool_size {
            return false;
        }
        self.transactions.insert(tx.tx_id.clone(), tx.clone());
        self.ordered_by_fee.insert(tx);
        true
    }

    pub fn remove_transaction(&mut self, tx_id: &str) -> Option<ChainTransaction> {
        let tx = self.transactions.remove(tx_id)?;
        self.ordered_by_fee.remove(&tx);
        Some(tx)
    }

    pub fn get_top_transactions(&self, count: usize) -> Vec<ChainTransaction> {
        self.ordered_by_fee.iter()
            .rev()
            .take(count)
            .cloned()
            .collect()
    }

    pub fn clear(&mut self) {
        self.transactions.clear();
        self.ordered_by_fee.clear();
    }
}
