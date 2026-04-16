use serde::{Serialize, Deserialize};
use crate::merkle_tree::MerkleTree;
use crate::blockchain_core::ChainBlock;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LightClientRequest {
    pub block_height: u64,
    pub tx_id: String,
    pub request_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LightClientProof {
    pub block_header: ChainBlock,
    pub merkle_proof: Vec<String>,
    pub transaction: Vec<u8>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncUpdate {
    pub latest_height: u64,
    pub block_hash: String,
    pub state_root: String,
}

pub struct LightClientProtocol {
    latest_block_height: u64,
    latest_state_root: String,
    max_proof_size: usize,
}

impl LightClientProtocol {
    pub fn new() -> Self {
        Self {
            latest_block_height: 0,
            latest_state_root: "0".to_string(),
            max_proof_size: 1024,
        }
    }

    pub fn update_sync(&mut self, update: SyncUpdate) {
        self.latest_block_height = update.latest_height;
        self.latest_state_root = update.state_root;
    }

    pub fn verify_proof(&self, proof: &LightClientProof, tx_id: &str) -> Result<bool, String> {
        if proof.merkle_proof.len() > self.max_proof_size {
            return Err("Proof too large".to_string());
        }
        
        let tx_hash = {
            use sha2::{Sha256, Digest};
            let mut hasher = Sha256::new();
            hasher.update(&proof.transaction);
            hex::encode(hasher.finalize())
        };
        
        let merkle = MerkleTree::new(&[proof.transaction.clone()]);
        let valid = merkle.verify_proof(&tx_hash, &proof.merkle_proof);
        
        Ok(valid && proof.block_header.hash.starts_with("0000"))
    }

    pub fn get_latest_state(&self) -> (u64, &str) {
        (self.latest_block_height, &self.latest_state_root)
    }

    pub fn create_request(&self, height: u64, tx_id: &str) -> LightClientRequest {
        use sha2::{Sha256, Digest};
        let mut hasher = Sha256::new();
        hasher.update(format!("{}{}", height, tx_id).as_bytes());
        let request_id = hex::encode(hasher.finalize());
        
        LightClientRequest {
            block_height: height,
            tx_id: tx_id.to_string(),
            request_id,
        }
    }
}
