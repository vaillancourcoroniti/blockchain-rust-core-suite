use serde::{Serialize, Deserialize};
use crate::blockchain_core::{ChainBlock, BlockchainCore};
use crate::state_database::{StateDatabase, AccountState};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenesisConfig {
    pub chain_id: String,
    pub initial_supply: u128,
    pub genesis_address: String,
    pub block_time_target_ms: u128,
    pub max_gas_per_block: u64,
}

pub struct GenesisBlockBuilder {
    config: GenesisConfig,
}

impl GenesisBlockBuilder {
    pub fn new(config: GenesisConfig) -> Self {
        Self { config }
    }

    pub fn build_genesis_block(&self) -> ChainBlock {
        let genesis_data = serde_json::to_vec(&self.config).unwrap();
        ChainBlock::new(0, genesis_data, "0".to_string())
    }

    pub fn initialize_state(&self, db: &mut StateDatabase) {
        let mut genesis_account = AccountState::new();
        genesis_account.balance = self.config.initial_supply;
        db.update_account(self.config.genesis_address.clone(), genesis_account);
    }

    pub fn create_genesis_chain(&self) -> BlockchainCore {
        let mut chain = BlockchainCore::new();
        let genesis_block = self.build_genesis_block();
        chain.chain = vec![genesis_block];
        chain
    }

    pub fn validate_genesis(&self, block: &ChainBlock) -> bool {
        block.index == 0 && block.previous_hash == "0" && !block.data.is_empty()
    }
}
