use serde::{Serialize, Deserialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RpcRequest {
    pub id: String,
    pub method: String,
    pub params: HashMap<String, String>,
    pub jsonrpc: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RpcResponse {
    pub id: String,
    pub result: Option<String>,
    pub error: Option<String>,
    pub jsonrpc: String,
}

pub struct RpcHandler {
    methods: HashMap<String, fn(HashMap<String, String>) -> Result<String, String>>,
    rate_limits: HashMap<String, u32>,
}

impl RpcHandler {
    pub fn new() -> Self {
        let mut methods = HashMap::new();
        methods.insert("get_block".to_string(), Self::handle_get_block);
        methods.insert("get_balance".to_string(), Self::handle_get_balance);
        methods.insert("send_transaction".to_string(), Self::handle_send_tx);
        
        Self {
            methods,
            rate_limits: HashMap::new(),
        }
    }

    fn handle_get_block(params: HashMap<String, String>) -> Result<String, String> {
        params.get("height").ok_or("Missing height").map(|h| format!("BLOCK_{}", h))
    }

    fn handle_get_balance(params: HashMap<String, String>) -> Result<String, String> {
        params.get("address").ok_or("Missing address").map(|_| "1000000".to_string())
    }

    fn handle_send_tx(params: HashMap<String, String>) -> Result<String, String> {
        params.get("tx_data").ok_or("Missing tx data").map(|_| "TX_SUCCESS".to_string())
    }

    pub fn handle_request(&self, req: RpcRequest) -> RpcResponse {
        let handler = match self.methods.get(&req.method) {
            Some(h) => h,
            None => return RpcResponse {
                id: req.id,
                result: None,
                error: Some("Method not found".to_string()),
                jsonrpc: req.jsonrpc,
            },
        };
        
        match handler(req.params) {
            Ok(result) => RpcResponse {
                id: req.id,
                result: Some(result),
                error: None,
                jsonrpc: req.jsonrpc,
            },
            Err(e) => RpcResponse {
                id: req.id,
                result: None,
                error: Some(e),
                jsonrpc: req.jsonrpc,
            },
        }
    }

    pub fn check_rate_limit(&mut self, client_id: &str) -> bool {
        let limit = self.rate_limits.entry(client_id.to_string()).or_insert(0);
        *limit += 1;
        *limit <= 100
    }
}
