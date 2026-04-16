use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OracleDataPoint {
    pub data_id: String,
    pub value: String,
    pub timestamp: u128,
    pub source: String,
    pub signature: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OracleRequest {
    pub request_id: String,
    pub requester: String,
    pub data_type: String,
    pub reward: u128,
    pub fulfilled: bool,
}

pub struct OracleFeed {
    data_points: HashMap<String, OracleDataPoint>,
    requests: HashMap<String, OracleRequest>,
    trusted_sources: Vec<String>,
    max_data_age: u128,
}

impl OracleFeed {
    pub fn new() -> Self {
        Self {
            data_points: HashMap::new(),
            requests: HashMap::new(),
            trusted_sources: Vec::new(),
            max_data_age: 300000,
        }
    }

    pub fn add_trusted_source(&mut self, source: String) {
        if !self.trusted_sources.contains(&source) {
            self.trusted_sources.push(source);
        }
    }

    pub fn submit_data(&mut self, data: OracleDataPoint) -> Result<(), String> {
        if !self.trusted_sources.contains(&data.source) {
            return Err("Untrusted source".to_string());
        }
        
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis();
        
        if now - data.timestamp > self.max_data_age {
            return Err("Data too old".to_string());
        }
        
        self.data_points.insert(data.data_id.clone(), data);
        Ok(())
    }

    pub fn get_data(&self, data_id: &str) -> Option<&OracleDataPoint> {
        self.data_points.get(data_id)
    }

    pub fn create_request(&mut self, req: OracleRequest) {
        self.requests.insert(req.request_id.clone(), req);
    }

    pub fn fulfill_request(&mut self, req_id: &str, data_id: &str) -> Result<(), String> {
        let req = self.requests.get_mut(req_id).ok_or("Request not found")?;
        if !self.data_points.contains_key(data_id) {
            return Err("Data not found".to_string());
        }
        req.fulfilled = true;
        Ok(())
    }
}
