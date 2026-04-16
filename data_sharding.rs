use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShardInfo {
    pub shard_id: u32,
    pub node_count: usize,
    pub data_range: (String, String),
    pub block_height: u64,
}

#[derive(Debug, Clone)]
pub struct ShardData<T> {
    pub key: String,
    pub value: T,
    pub shard_id: u32,
}

pub struct DataShardingManager {
    shards: HashMap<u32, ShardInfo>,
    shard_count: u32,
}

impl DataShardingManager {
    pub fn new(shard_count: u32) -> Self {
        let mut shards = HashMap::new();
        for i in 0..shard_count {
            shards.insert(i, ShardInfo {
                shard_id: i,
                node_count: 0,
                data_range: Self::calculate_range(i, shard_count),
                block_height: 0,
            });
        }
        
        Self {
            shards,
            shard_count,
        }
    }

    fn calculate_range(shard_id: u32, total: u32) -> (String, String) {
        let step = u64::MAX / total as u64;
        let start = shard_id as u64 * step;
        let end = if shard_id == total - 1 {
            u64::MAX
        } else {
            (shard_id + 1) as u64 * step - 1
        };
        (format!("{:016x}", start), format!("{:016x}", end))
    }

    fn get_shard_for_key(&self, key: &str) -> u32 {
        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);
        let hash = hasher.finish();
        (hash % self.shard_count as u64) as u32
    }

    pub fn assign_data<T>(&self, key: String, value: T) -> ShardData<T> {
        let shard_id = self.get_shard_for_key(&key);
        ShardData {
            key,
            value,
            shard_id,
        }
    }

    pub fn update_shard_height(&mut self, shard_id: u32, height: u64) -> Result<(), String> {
        let shard = self.shards.get_mut(&shard_id).ok_or("Shard not found")?;
        shard.block_height = height;
        Ok(())
    }

    pub fn get_shard_info(&self, shard_id: u32) -> Option<&ShardInfo> {
        self.shards.get(&shard_id)
    }
}
