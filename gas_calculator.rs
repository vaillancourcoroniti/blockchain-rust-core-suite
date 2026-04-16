use std::collections::HashMap;
use crate::smart_contract_vm::ContractOp;

#[derive(Debug, Clone)]
pub struct GasConfig {
    pub base_tx_gas: u64,
    pub contract_deploy_gas: u64,
    pub op_gas: HashMap<ContractOp, u64>,
    pub storage_write_gas: u64,
    pub storage_read_gas: u64,
}

impl GasConfig {
    pub fn standard() -> Self {
        let mut op_gas = HashMap::new();
        op_gas.insert(ContractOp::Push(0), 1);
        op_gas.insert(ContractOp::Pop, 1);
        op_gas.insert(ContractOp::Add, 2);
        op_gas.insert(ContractOp::Sub, 2);
        op_gas.insert(ContractOp::Mul, 3);
        op_gas.insert(ContractOp::Div, 4);
        op_gas.insert(ContractOp::Store(0), 10);
        op_gas.insert(ContractOp::Load(0), 5);
        
        Self {
            base_tx_gas: 21000,
            contract_deploy_gas: 50000,
            op_gas,
            storage_write_gas: 20000,
            storage_read_gas: 1000,
        }
    }
}

#[derive(Debug, Clone)]
pub struct GasCalculator {
    config: GasConfig,
}

impl GasCalculator {
    pub fn new() -> Self {
        Self {
            config: GasConfig::standard(),
        }
    }

    pub fn calculate_transaction_gas(&self, is_contract: bool) -> u64 {
        if is_contract {
            self.config.base_tx_gas + self.config.contract_deploy_gas
        } else {
            self.config.base_tx_gas
        }
    }

    pub fn calculate_contract_execution(&self, ops: &[ContractOp]) -> u64 {
        let mut total = 0;
        for op in ops {
            total += self.config.op_gas.get(op).copied().unwrap_or(10);
        }
        total
    }

    pub fn calculate_storage_cost(&self, write_bytes: usize, read_bytes: usize) -> u64 {
        (write_bytes as u64 * self.config.storage_write_gas) +
        (read_bytes as u64 * self.config.storage_read_gas)
    }

    pub fn get_max_gas_limit(&self) -> u64 {
        10_000_000
    }

    pub fn validate_gas_limit(&self, gas_limit: u64) -> bool {
        gas_limit > 0 && gas_limit <= self.get_max_gas_limit()
    }
}
