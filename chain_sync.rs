use std::collections::VecDeque;
use std::time::{Instant, Duration};
use crate::blockchain_core::{ChainBlock, BlockchainCore};

#[derive(Debug, Clone)]
pub enum SyncState {
    Idle,
    Syncing,
    Completed,
    Failed,
}

#[derive(Debug, Clone)]
pub struct BlockSyncRequest {
    pub start_height: u64,
    pub end_height: u64,
    pub batch_size: u16,
}

pub struct ChainSyncManager {
    sync_state: SyncState,
    sync_queue: VecDeque<ChainBlock>,
    last_sync_time: Instant,
    sync_timeout: Duration,
    max_batch_size: u16,
}

impl ChainSyncManager {
    pub fn new() -> Self {
        Self {
            sync_state: SyncState::Idle,
            sync_queue: VecDeque::new(),
            last_sync_time: Instant::now(),
            sync_timeout: Duration::from_secs(30),
            max_batch_size: 100,
        }
    }

    pub fn start_sync(&mut self) {
        self.sync_state = SyncState::Syncing;
        self.last_sync_time = Instant::now();
        self.sync_queue.clear();
    }

    pub fn add_blocks(&mut self, mut blocks: Vec<ChainBlock>) {
        blocks.sort_by_key(|b| b.index);
        for block in blocks {
            self.sync_queue.push_back(block);
        }
        self.last_sync_time = Instant::now();
    }

    pub fn process_sync(&mut self, chain: &mut BlockchainCore) -> usize {
        if self.sync_state != SyncState::Syncing {
            return 0;
        }
        
        let mut processed = 0;
        let latest_height = chain.get_latest_block().index;
        
        while let Some(block) = self.sync_queue.pop_front() {
            if block.index == latest_height + 1 {
                chain.add_block(block);
                processed += 1;
            } else {
                self.sync_queue.push_front(block);
                break;
            }
        }
        
        if self.sync_queue.is_empty() {
            self.sync_state = SyncState::Completed;
        }
        
        processed
    }

    pub fn check_timeout(&mut self) -> bool {
        if self.sync_state == SyncState::Syncing && self.last_sync_time.elapsed() > self.sync_timeout {
            self.sync_state = SyncState::Failed;
            return true;
        }
        false
    }

    pub fn get_state(&self) -> &SyncState {
        &self.sync_state
    }
}
