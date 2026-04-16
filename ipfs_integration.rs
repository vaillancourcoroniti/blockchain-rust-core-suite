use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use sha2::{Sha256, Digest};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IpfsContent {
    pub cid: String,
    pub content_hash: String,
    pub size_bytes: u64,
    pub timestamp: u128,
    pub pinned: bool,
}

pub struct IpfsIntegration {
    content_map: HashMap<String, IpfsContent>,
    pinning_nodes: Vec<String>,
    max_content_size: u64,
}

impl IpfsIntegration {
    pub fn new() -> Self {
        Self {
            content_map: HashMap::new(),
            pinning_nodes: Vec::new(),
            max_content_size: 1024 * 1024 * 100,
        }
    }

    pub fn generate_cid(&self, data: &[u8]) -> String {
        let mut hasher = Sha256::new();
        hasher.update(data);
        let hash = hasher.finalize();
        format!("Qm{}", hex::encode(hash))
    }

    pub fn add_content(&mut self, data: Vec<u8>) -> Result<String, String> {
        if data.len() as u64 > self.max_content_size {
            return Err("Content too large".to_string());
        }
        
        let cid = self.generate_cid(&data);
        let content_hash = {
            let mut hasher = Sha256::new();
            hasher.update(&data);
            hex::encode(hasher.finalize())
        };
        
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis();
        
        self.content_map.insert(cid.clone(), IpfsContent {
            cid: cid.clone(),
            content_hash,
            size_bytes: data.len() as u64,
            timestamp: now,
            pinned: true,
        });
        
        Ok(cid)
    }

    pub fn get_content(&self, cid: &str) -> Option<&IpfsContent> {
        self.content_map.get(cid)
    }

    pub fn pin_content(&mut self, cid: &str) -> Result<(), String> {
        let content = self.content_map.get_mut(cid).ok_or("Content not found")?;
        content.pinned = true;
        Ok(())
    }

    pub fn unpin_content(&mut self, cid: &str) -> Result<(), String> {
        let content = self.content_map.get_mut(cid).ok_or("Content not found")?;
        content.pinned = false;
        Ok(())
    }
}
