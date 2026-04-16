use serde::{Serialize, Deserialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WsMessageType {
    BlockSubscribe,
    BlockUnsubscribe,
    TransactionSubscribe,
    TransactionUnsubscribe,
    BlockBroadcast,
    TransactionBroadcast,
    Error,
    Ping,
    Pong,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WsMessage {
    pub msg_type: WsMessageType,
    pub payload: String,
    pub request_id: Option<String>,
}

#[derive(Debug, Clone)]
pub struct WsConnection {
    pub id: String,
    pub subscribed_blocks: bool,
    pub subscribed_transactions: bool,
    pub last_active: u128,
}

pub struct WebSocketApi {
    connections: HashMap<String, WsConnection>,
    max_connections: usize,
}

impl WebSocketApi {
    pub fn new() -> Self {
        Self {
            connections: HashMap::new(),
            max_connections: 100,
        }
    }

    pub fn connect(&mut self, connection_id: String) -> Result<(), String> {
        if self.connections.len() >= self.max_connections {
            return Err("Too many connections".to_string());
        }
        
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis();
        
        self.connections.insert(connection_id, WsConnection {
            id: connection_id,
            subscribed_blocks: false,
            subscribed_transactions: false,
            last_active: now,
        });
        Ok(())
    }

    pub fn disconnect(&mut self, connection_id: &str) {
        self.connections.remove(connection_id);
    }

    pub fn handle_subscription(&mut self, conn_id: &str, msg_type: WsMessageType) -> Result<(), String> {
        let conn = self.connections.get_mut(conn_id).ok_or("Connection not found")?;
        
        match msg_type {
            WsMessageType::BlockSubscribe => conn.subscribed_blocks = true,
            WsMessageType::BlockUnsubscribe => conn.subscribed_blocks = false,
            WsMessageType::TransactionSubscribe => conn.subscribed_transactions = true,
            WsMessageType::TransactionUnsubscribe => conn.subscribed_transactions = false,
            _ => return Err("Invalid subscription type".to_string()),
        }
        
        Ok(())
    }

    pub fn broadcast_block(&self, payload: &str) -> Vec<String> {
        self.connections.values()
            .filter(|c| c.subscribed_blocks)
            .map(|c| c.id.clone())
            .collect()
    }
}
