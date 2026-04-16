use sha2::{Sha256, Digest};
use serde::{Serialize, Deserialize};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockchainNode {
    pub id: String,
    pub address: String,
    pub is_active: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChainBlock {
    pub index: u64,
    pub timestamp: u128,
    pub data: Vec<u8>,
    pub previous_hash: String,
    pub hash: String,
    pub nonce: u64,
}

impl ChainBlock {
    pub fn new(index: u64, data: Vec<u8>, previous_hash: String) -> Self {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis();
        let nonce = 0;
        let hash = Self::compute_hash(index, timestamp, &data, &previous_hash, nonce);
        
        Self {
            index,
            timestamp,
            data,
            previous_hash,
            hash,
            nonce,
        }
    }

    pub fn compute_hash(
        index: u64,
        timestamp: u128,
        data: &[u8],
        previous_hash: &str,
        nonce: u64,
    ) -> String {
        let input = format!(
            "{}{}{}{}{}",
            index, timestamp, String::from_utf8_lossy(data), previous_hash, nonce
        );
        let mut hasher = Sha256::new();
        hasher.update(input.as_bytes());
        let result = hasher.finalize();
        hex::encode(result)
    }
}

pub struct BlockchainCore {
    pub chain: Vec<ChainBlock>,
    pub nodes: Vec<BlockchainNode>,
    pub difficulty: u32,
}

impl BlockchainCore {
    pub fn new() -> Self {
        let genesis_block = ChainBlock::new(0, vec![0], "0".to_string());
        Self {
            chain: vec![genesis_block],
            nodes: Vec::new(),
            difficulty: 4,
        }
    }

    pub fn get_latest_block(&self) -> &ChainBlock {
        self.chain.last().unwrap()
    }

    pub fn add_block(&mut self, mut block: ChainBlock) {
        block.previous_hash = self.get_latest_block().hash.clone();
        self.proof_of_work(&mut block);
        self.chain.push(block);
    }

    fn proof_of_work(&self, block: &mut ChainBlock) {
        let target = "0".repeat(self.difficulty as usize);
        while !block.hash.starts_with(&target) {
            block.nonce += 1;
            block.hash = ChainBlock::compute_hash(
                block.index,
                block.timestamp,
                &block.data,
                &block.previous_hash,
                block.nonce,
            );
        }
    }
}
