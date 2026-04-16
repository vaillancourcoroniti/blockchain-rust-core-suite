use std::collections::{HashMap, HashSet};
use std::net::IpAddr;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone, PartialEq)]
pub enum ThreatType {
    DDoS,
    SybilAttack,
    DoubleSpend,
    InvalidBlock,
    Unknown,
}

#[derive(Debug, Clone)]
pub struct SecurityEvent {
    pub source: IpAddr,
    pub threat: ThreatType,
    pub timestamp: u128,
    pub severity: u8,
}

pub struct NetworkSecurity {
    banned_ips: HashSet<IpAddr>,
    ip_request_counts: HashMap<IpAddr, (u32, u128)>,
    max_requests_per_minute: u32,
    security_events: Vec<SecurityEvent>,
}

impl NetworkSecurity {
    pub fn new() -> Self {
        Self {
            banned_ips: HashSet::new(),
            ip_request_counts: HashMap::new(),
            max_requests_per_minute: 60,
            security_events: Vec::new(),
        }
    }

    pub fn validate_request(&mut self, ip: IpAddr) -> bool {
        if self.banned_ips.contains(&ip) {
            return false;
        }
        
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis();
        
        let (count, window_start) = self.ip_request_counts.entry(ip).or_insert((0, now));
        
        if now - *window_start > 60000 {
            *count = 1;
            *window_start = now;
        } else {
            *count += 1;
        }
        
        if *count > self.max_requests_per_minute {
            self.ban_ip(ip, ThreatType::DDoS);
            return false;
        }
        
        true
    }

    pub fn ban_ip(&mut self, ip: IpAddr, threat: ThreatType) {
        self.banned_ips.insert(ip);
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis();
        
        self.security_events.push(SecurityEvent {
            source: ip,
            threat,
            timestamp: now,
            severity: 10,
        });
    }

    pub fn unban_ip(&mut self, ip: &IpAddr) {
        self.banned_ips.remove(ip);
    }

    pub fn get_security_events(&self) -> &[SecurityEvent] {
        &self.security_events
    }
}
