use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use std::net::SocketAddr;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum NodeRole {
    Validator,
    FullNode,
    LightNode,
    ArchiveNode,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeInfo {
    pub node_id: String,
    pub address: SocketAddr,
    pub role: NodeRole,
    pub version: String,
    pub height: u64,
    pub last_heartbeat: u128,
    pub is_synced: bool,
}

pub struct NodeManager {
    nodes: HashMap<String, NodeInfo>,
    max_node_count: usize,
    heartbeat_timeout: u128,
}

impl NodeManager {
    pub fn new() -> Self {
        Self {
            nodes: HashMap::new(),
            max_node_count: 100,
            heartbeat_timeout: 30000,
        }
    }

    pub fn register_node(&mut self, mut node: NodeInfo) -> Result<(), String> {
        if self.nodes.len() >= self.max_node_count {
            return Err("Node registry full".to_string());
        }
        node.last_heartbeat = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis();
        self.nodes.insert(node.node_id.clone(), node);
        Ok(())
    }

    pub fn update_heartbeat(&mut self, node_id: &str) -> Result<(), String> {
        let node = self.nodes.get_mut(node_id).ok_or("Node not found")?;
        node.last_heartbeat = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis();
        Ok(())
    }

    pub fn update_sync_status(&mut self, node_id: &str, height: u64, synced: bool) -> Result<(), String> {
        let node = self.nodes.get_mut(node_id).ok_or("Node not found")?;
        node.height = height;
        node.is_synced = synced;
        Ok(())
    }

    pub fn prune_offline_nodes(&mut self) {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis();
        
        self.nodes.retain(|_, node| {
            now - node.last_heartbeat < self.heartbeat_timeout
        });
    }

    pub fn get_active_nodes(&self) -> Vec<&NodeInfo> {
        self.nodes.values().collect()
    }
}
