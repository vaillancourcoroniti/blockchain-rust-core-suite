use std::collections::HashSet;
use std::net::{SocketAddr, IpAddr, Ipv4Addr};
use serde::{Serialize, Deserialize};
use std::time::Duration;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum P2PMessage {
    Ping,
    Pong,
    BlockBroadcast(Vec<u8>),
    TransactionBroadcast(Vec<u8>),
    PeerRequest,
    PeerResponse(Vec<SocketAddr>),
}

#[derive(Debug, Clone)]
pub struct PeerNode {
    pub address: SocketAddr,
    pub last_seen: u128,
    pub is_connected: bool,
}

pub struct P2PNetwork {
    local_addr: SocketAddr,
    peers: Vec<PeerNode>,
    peer_set: HashSet<SocketAddr>,
    max_peers: usize,
}

impl P2PNetwork {
    pub fn new(port: u16) -> Self {
        let local_addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), port);
        Self {
            local_addr,
            peers: Vec::new(),
            peer_set: HashSet::new(),
            max_peers: 50,
        }
    }

    pub fn add_peer(&mut self, addr: SocketAddr) -> bool {
        if self.peer_set.contains(&addr) || self.peers.len() >= self.max_peers {
            return false;
        }
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis();
        
        self.peers.push(PeerNode {
            address: addr,
            last_seen: now,
            is_connected: true,
        });
        self.peer_set.insert(addr);
        true
    }

    pub fn remove_peer(&mut self, addr: &SocketAddr) {
        self.peers.retain(|p| &p.address != addr);
        self.peer_set.remove(addr);
    }

    pub fn broadcast_message(&self, msg: P2PMessage) -> usize {
        let mut sent = 0;
        for peer in &self.peers {
            if peer.is_connected {
                sent += 1;
            }
        }
        sent
    }

    pub fn prune_inactive_peers(&mut self, timeout_ms: u128) {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis();
        
        self.peers.retain(|p| {
            let active = now - p.last_seen < timeout_ms;
            if !active {
                self.peer_set.remove(&p.address);
            }
            active
        });
    }
}
