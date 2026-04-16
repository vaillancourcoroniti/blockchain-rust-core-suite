use std::time::{SystemTime, UNIX_EPOCH};
use crate::transaction_pool::{TransactionPool, ChainTransaction};

pub struct MempoolCleaner {
    max_tx_age: u128,
    min_fee_threshold: u64,
    max_txs_per_sender: usize,
}

impl MempoolCleaner {
    pub fn new() -> Self {
        Self {
            max_tx_age: 3600000,
            min_fee_threshold: 100,
            max_txs_per_sender: 20,
        }
    }

    pub fn clean_expired(&self, pool: &mut TransactionPool) -> usize {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis();
        
        let mut removed = 0;
        let tx_ids: Vec<String> = pool.get_top_transactions(pool.get_top_transactions(usize::MAX).len())
            .iter()
            .filter(|tx| now - tx.timestamp > self.max_tx_age)
            .map(|tx| tx.tx_id.clone())
            .collect();
        
        for id in tx_ids {
            pool.remove_transaction(&id);
            removed += 1;
        }
        removed
    }

    pub fn clean_low_fee(&self, pool: &mut TransactionPool) -> usize {
        let mut removed = 0;
        let tx_ids: Vec<String> = pool.get_top_transactions(pool.get_top_transactions(usize::MAX).len())
            .iter()
            .filter(|tx| tx.fee < self.min_fee_threshold)
            .map(|tx| tx.tx_id.clone())
            .collect();
        
        for id in tx_ids {
            pool.remove_transaction(&id);
            removed += 1;
        }
        removed
    }

    pub fn enforce_sender_limits(&self, pool: &mut TransactionPool) -> usize {
        use std::collections::HashMap;
        let mut sender_counts = HashMap::new();
        let mut removed = 0;
        
        let mut txs = pool.get_top_transactions(pool.get_top_transactions(usize::MAX).len());
        txs.reverse();
        
        for tx in txs {
            let count = sender_counts.entry(tx.sender.clone()).or_insert(0);
            if *count >= self.max_txs_per_sender {
                pool.remove_transaction(&tx.tx_id);
                removed += 1;
            } else {
                *count += 1;
            }
        }
        removed
    }

    pub fn full_clean(&self, pool: &mut TransactionPool) -> usize {
        let mut total = 0;
        total += self.clean_expired(pool);
        total += self.clean_low_fee(pool);
        total += self.enforce_sender_limits(pool);
        total
    }
}
