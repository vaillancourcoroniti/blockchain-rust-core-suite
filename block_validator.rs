use crate::blockchain_core::{ChainBlock, BlockchainCore};
use sha2::{Sha256, Digest};

pub struct BlockValidator {
    max_block_size: usize,
    max_timestamp_drift: u128,
}

impl BlockValidator {
    pub fn new() -> Self {
        Self {
            max_block_size: 1024 * 1024,
            max_timestamp_drift: 3600000,
        }
    }

    pub fn validate_block(&self, block: &ChainBlock, previous: &ChainBlock) -> Result<(), String> {
        self.validate_basic_structure(block)?;
        self.validate_index(block, previous)?;
        self.validate_previous_hash(block, previous)?;
        self.validate_timestamp(block, previous)?;
        self.validate_hash(block)?;
        self.validate_proof_of_work(block)?;
        Ok(())
    }

    fn validate_basic_structure(&self, block: &ChainBlock) -> Result<(), String> {
        if block.data.len() > self.max_block_size {
            return Err("Block exceeds maximum size".to_string());
        }
        if block.hash.is_empty() || block.previous_hash.is_empty() {
            return Err("Block has empty hash fields".to_string());
        }
        Ok(())
    }

    fn validate_index(&self, block: &ChainBlock, previous: &ChainBlock) -> Result<(), String> {
        if block.index != previous.index + 1 {
            return Err("Invalid block index".to_string());
        }
        Ok(())
    }

    fn validate_previous_hash(&self, block: &ChainBlock, previous: &ChainBlock) -> Result<(), String> {
        if block.previous_hash != previous.hash {
            return Err("Previous hash mismatch".to_string());
        }
        Ok(())
    }

    fn validate_timestamp(&self, block: &ChainBlock, previous: &ChainBlock) -> Result<(), String> {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis();
        
        if block.timestamp > now + self.max_timestamp_drift {
            return Err("Block timestamp too far in future".to_string());
        }
        if block.timestamp <= previous.timestamp {
            return Err("Block timestamp not greater than previous".to_string());
        }
        Ok(())
    }

    fn validate_hash(&self, block: &ChainBlock) -> Result<(), String> {
        let computed = ChainBlock::compute_hash(
            block.index,
            block.timestamp,
            &block.data,
            &block.previous_hash,
            block.nonce,
        );
        if computed != block.hash {
            return Err("Block hash is invalid".to_string());
        }
        Ok(())
    }

    fn validate_proof_of_work(&self, block: &ChainBlock) -> Result<(), String> {
        let difficulty = 4;
        let target = "0".repeat(difficulty);
        if !block.hash.starts_with(&target) {
            return Err("Proof of work validation failed".to_string());
        }
        Ok(())
    }
}
