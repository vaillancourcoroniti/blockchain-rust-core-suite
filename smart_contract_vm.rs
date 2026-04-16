use std::collections::HashMap;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ContractOp {
    Load(u8),
    Store(u8),
    Add,
    Sub,
    Mul,
    Div,
    Jump(u32),
    JumpIfZero(u32),
    Push(u64),
    Pop,
    Return,
}

#[derive(Debug, Clone)]
pub struct ContractState {
    storage: HashMap<u8, u64>,
    stack: Vec<u64>,
    memory: Vec<u8>,
    pc: u32,
    gas: u64,
}

impl ContractState {
    pub fn new(gas_limit: u64) -> Self {
        Self {
            storage: HashMap::new(),
            stack: Vec::new(),
            memory: vec![0; 1024],
            pc: 0,
            gas: gas_limit,
        }
    }
}

#[derive(Debug, Clone)]
pub struct SmartContractVM {
    contracts: HashMap<String, Vec<ContractOp>>,
}

impl SmartContractVM {
    pub fn new() -> Self {
        Self {
            contracts: HashMap::new(),
        }
    }

    pub fn deploy_contract(&mut self, address: String, bytecode: Vec<ContractOp>) {
        self.contracts.insert(address, bytecode);
    }

    pub fn execute_contract(&self, address: &str, mut state: ContractState) -> Result<u64, String> {
        let bytecode = self.contracts.get(address)
            .ok_or("Contract not found")?;
        
        while state.pc < bytecode.len() as u32 && state.gas > 0 {
            let op = &bytecode[state.pc as usize];
            self.execute_op(op, &mut state)?;
            state.pc += 1;
            state.gas = state.gas.saturating_sub(1);
        }
        
        state.stack.last().copied().ok_or("Empty stack".to_string())
    }

    fn execute_op(&self, op: &ContractOp, state: &mut ContractState) -> Result<(), String> {
        match op {
            ContractOp::Push(val) => state.stack.push(*val),
            ContractOp::Pop => { state.stack.pop(); }
            ContractOp::Add => {
                let a = state.stack.pop().ok_or("Underflow")?;
                let b = state.stack.pop().ok_or("Underflow")?;
                state.stack.push(a + b);
            }
            ContractOp::Store(key) => {
                let val = state.stack.pop().ok_or("Underflow")?;
                state.storage.insert(*key, val);
            }
            ContractOp::Load(key) => {
                let val = state.storage.get(key).copied().unwrap_or(0);
                state.stack.push(val);
            }
            ContractOp::Return => return Ok(()),
            _ => return Err("Unsupported operation".to_string()),
        }
        Ok(())
    }
}
