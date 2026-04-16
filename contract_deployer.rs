use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use crate::smart_contract_vm::{SmartContractVM, ContractOp};
use crate::state_database::StateDatabase;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContractDeployRequest {
    pub bytecode: Vec<ContractOp>,
    pub deployer: String,
    pub gas_limit: u64,
    pub constructor_args: Vec<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContractRecord {
    pub address: String,
    pub deployer: String,
    pub deploy_height: u64,
    pub bytecode_hash: String,
    pub is_active: bool,
}

pub struct ContractDeployer {
    vm: SmartContractVM,
    contracts: HashMap<String, ContractRecord>,
    nonce_counter: HashMap<String, u64>,
}

impl ContractDeployer {
    pub fn new() -> Self {
        Self {
            vm: SmartContractVM::new(),
            contracts: HashMap::new(),
            nonce_counter: HashMap::new(),
        }
    }

    pub fn generate_contract_address(&self, deployer: &str, nonce: u64) -> String {
        use sha2::{Sha256, Digest};
        let input = format!("{}{}", deployer, nonce);
        let mut hasher = Sha256::new();
        hasher.update(input.as_bytes());
        let hash = hex::encode(hasher.finalize());
        format!("0x{}", &hash[0..40])
    }

    pub fn deploy_contract(&mut self, req: ContractDeployRequest, height: u64, state: &mut StateDatabase) -> Result<String, String> {
        let nonce = self.nonce_counter.entry(req.deployer.clone()).or_insert(0);
        *nonce += 1;
        
        let address = self.generate_contract_address(&req.deployer, *nonce);
        
        let bytecode_hash = {
            use sha2::{Sha256, Digest};
            let mut hasher = Sha256::new();
            hasher.update(format!("{:?}", req.bytecode).as_bytes());
            hex::encode(hasher.finalize())
        };
        
        self.vm.deploy_contract(address.clone(), req.bytecode);
        
        let record = ContractRecord {
            address: address.clone(),
            deployer: req.deployer,
            deploy_height: height,
            bytecode_hash,
            is_active: true,
        };
        
        self.contracts.insert(address.clone(), record);
        Ok(address)
    }

    pub fn execute_contract(&self, address: &str, state: crate::smart_contract_vm::ContractState) -> Result<u64, String> {
        self.vm.execute_contract(address, state)
    }

    pub fn get_contract(&self, address: &str) -> Option<&ContractRecord> {
        self.contracts.get(address)
    }

    pub fn disable_contract(&mut self, address: &str) -> Result<(), String> {
        let contract = self.contracts.get_mut(address).ok_or("Contract not found")?;
        contract.is_active = false;
        Ok(())
    }
}
