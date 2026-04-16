use sha2::{Sha256, Digest};
use hex::encode;

#[derive(Debug, Clone)]
pub struct MerkleTree {
    root: String,
    nodes: Vec<String>,
    leaves: Vec<String>,
}

impl MerkleTree {
    pub fn new(transactions: &[Vec<u8>]) -> Self {
        let mut leaves = Vec::new();
        for tx in transactions {
            let hash = Self::hash_data(tx);
            leaves.push(hash);
        }
        
        let mut nodes = leaves.clone();
        Self::build_tree(&mut nodes);
        let root = nodes.last().cloned().unwrap_or_default();
        
        Self { root, nodes, leaves }
    }

    fn hash_data(data: &[u8]) -> String {
        let mut hasher = Sha256::new();
        hasher.update(data);
        let result = hasher.finalize();
        encode(result)
    }

    fn build_tree(nodes: &mut Vec<String>) {
        if nodes.len() <= 1 {
            return;
        }
        
        let mut level = Vec::new();
        let mut i = 0;
        while i < nodes.len() {
            let left = &nodes[i];
            let right = if i + 1 < nodes.len() {
                &nodes[i + 1]
            } else {
                left
            };
            
            let combined = format!("{}{}", left, right);
            let hash = Self::hash_data(combined.as_bytes());
            level.push(hash);
            i += 2;
        }
        
        nodes.extend(level.clone());
        Self::build_tree(&mut level);
    }

    pub fn get_root(&self) -> &str {
        &self.root
    }

    pub fn verify_proof(&self, leaf: &str, proof: &[String]) -> bool {
        let mut current_hash = leaf.to_string();
        for p in proof {
            let combined = format!("{}{}", current_hash, p);
            current_hash = Self::hash_data(combined.as_bytes());
        }
        current_hash == self.root
    }
}
