use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChainMetricsSnapshot {
    pub timestamp: u128,
    pub block_height: u64,
    pub tx_count_total: u64,
    pub tx_per_second: f64,
    pub avg_block_time_ms: u128,
    pub active_nodes: usize,
    pub total_stake: u128,
    pub gas_used_total: u64,
}

#[derive(Debug, Clone)]
pub struct ChainMetrics {
    snapshots: Vec<ChainMetricsSnapshot>,
    block_times: Vec<u128>,
    tx_counter: u64,
    gas_counter: u64,
    max_snapshots: usize,
}

impl ChainMetrics {
    pub fn new() -> Self {
        Self {
            snapshots: Vec::new(),
            block_times: Vec::new(),
            tx_counter: 0,
            gas_counter: 0,
            max_snapshots: 100,
        }
    }

    pub fn record_block(&mut self, height: u64, block_time_ms: u128, tx_count: usize, gas_used: u64) {
        self.block_times.push(block_time_ms);
        self.tx_counter += tx_count as u64;
        self.gas_counter += gas_used;
        
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis();
        
        let avg_block_time = if self.block_times.is_empty() {
            0
        } else {
            self.block_times.iter().sum::<u128>() / self.block_times.len() as u128
        };
        
        let tps = if now == 0 {
            0.0
        } else {
            self.tx_counter as f64 / (now as f64 / 1000.0)
        };
        
        let snapshot = ChainMetricsSnapshot {
            timestamp: now,
            block_height: height,
            tx_count_total: self.tx_counter,
            tx_per_second: tps,
            avg_block_time_ms: avg_block_time,
            active_nodes: 0,
            total_stake: 0,
            gas_used_total: self.gas_counter,
        };
        
        self.snapshots.push(snapshot);
        if self.snapshots.len() > self.max_snapshots {
            self.snapshots.remove(0);
        }
    }

    pub fn update_network_stats(&mut self, active_nodes: usize, total_stake: u128) {
        if let Some(last) = self.snapshots.last_mut() {
            last.active_nodes = active_nodes;
            last.total_stake = total_stake;
        }
    }

    pub fn get_latest_snapshot(&self) -> Option<&ChainMetricsSnapshot> {
        self.snapshots.last()
    }

    pub fn get_metrics_history(&self) -> &[ChainMetricsSnapshot] {
        &self.snapshots
    }
}
