use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkStats {
    pub timestamp: u128,
    pub active_connections: usize,
    pub bytes_sent: u64,
    pub bytes_received: u64,
    pub avg_latency_ms: u128,
    pub error_count: u32,
}

#[derive(Debug, Clone)]
pub struct PeerMetrics {
    pub address: String,
    pub latency_ms: u128,
    pub last_message: u128,
    pub message_count: u64,
}

pub struct NetworkMonitor {
    stats_history: Vec<NetworkStats>,
    peer_metrics: HashMap<String, PeerMetrics>,
    bytes_sent: u64,
    bytes_received: u64,
    error_count: u32,
}

impl NetworkMonitor {
    pub fn new() -> Self {
        Self {
            stats_history: Vec::new(),
            peer_metrics: HashMap::new(),
            bytes_sent: 0,
            bytes_received: 0,
            error_count: 0,
        }
    }

    pub fn record_bytes_sent(&mut self, bytes: u64) {
        self.bytes_sent += bytes;
    }

    pub fn record_bytes_received(&mut self, bytes: u64) {
        self.bytes_received += bytes;
    }

    pub fn record_error(&mut self) {
        self.error_count += 1;
    }

    pub fn update_peer_latency(&mut self, peer: String, latency: u128) {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis();
        
        let metrics = self.peer_metrics.entry(peer).or_insert_with(|| PeerMetrics {
            address: String::new(),
            latency_ms: 0,
            last_message: 0,
            message_count: 0,
        });
        
        metrics.latency_ms = latency;
        metrics.last_message = now;
        metrics.message_count += 1;
    }

    pub fn take_snapshot(&mut self) -> NetworkStats {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis();
        
        let avg_latency = if self.peer_metrics.is_empty() {
            0
        } else {
            self.peer_metrics.values().map(|p| p.latency_ms).sum::<u128>() / self.peer_metrics.len() as u128
        };
        
        let stats = NetworkStats {
            timestamp: now,
            active_connections: self.peer_metrics.len(),
            bytes_sent: self.bytes_sent,
            bytes_received: self.bytes_received,
            avg_latency_ms: avg_latency,
            error_count: self.error_count,
        };
        
        self.stats_history.push(stats.clone());
        if self.stats_history.len() > 50 {
            self.stats_history.remove(0);
        }
        
        stats
    }

    pub fn get_latest_stats(&self) -> Option<&NetworkStats> {
        self.stats_history.last()
    }
}
