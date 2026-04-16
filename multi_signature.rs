use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use crate::crypto_ed25519::Ed25519Crypto;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MultiSigWallet {
    pub wallet_id: String,
    pub owners: Vec<String>,
    pub required_signatures: u8,
    pub balance: u128,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MultiSigTransaction {
    pub tx_id: String,
    pub wallet_id: String,
    pub destination: String,
    pub amount: u128,
    pub signatures: Vec<String>,
    pub executed: bool,
}

pub struct MultiSignatureManager {
    wallets: HashMap<String, MultiSigWallet>,
    transactions: HashMap<String, MultiSigTransaction>,
}

impl MultiSignatureManager {
    pub fn new() -> Self {
        Self {
            wallets: HashMap::new(),
            transactions: HashMap::new(),
        }
    }

    pub fn create_wallet(&mut self, owners: Vec<String>, required: u8) -> Result<String, String> {
        if required == 0 || required > owners.len() as u8 {
            return Err("Invalid signature requirement".to_string());
        }
        
        let wallet_id = {
            use sha2::{Sha256, Digest};
            let mut hasher = Sha256::new();
            hasher.update(format!("{:?}{}", owners, required).as_bytes());
            hex::encode(hasher.finalize())
        };
        
        self.wallets.insert(wallet_id.clone(), MultiSigWallet {
            wallet_id: wallet_id.clone(),
            owners,
            required_signatures: required,
            balance: 0,
        });
        
        Ok(wallet_id)
    }

    pub fn create_transaction(&mut self, wallet_id: String, destination: String, amount: u128) -> Result<String, String> {
        let wallet = self.wallets.get(&wallet_id).ok_or("Wallet not found")?;
        if wallet.balance < amount {
            return Err("Insufficient balance".to_string());
        }
        
        let tx_id = {
            use sha2::{Sha256, Digest};
            let mut hasher = Sha256::new();
            hasher.update(format!("{}{}{}", wallet_id, destination, amount).as_bytes());
            hex::encode(hasher.finalize())
        };
        
        self.transactions.insert(tx_id.clone(), MultiSigTransaction {
            tx_id: tx_id.clone(),
            wallet_id,
            destination,
            amount,
            signatures: Vec::new(),
            executed: false,
        });
        
        Ok(tx_id)
    }

    pub fn add_signature(&mut self, tx_id: &str, signer: &str, signature: String) -> Result<(), String> {
        let tx = self.transactions.get_mut(tx_id).ok_or("Transaction not found")?;
        let wallet = self.wallets.get(&tx.wallet_id).ok_or("Wallet not found")?;
        
        if !wallet.owners.contains(&signer.to_string()) {
            return Err("Not a wallet owner".to_string());
        }
        if tx.signatures.contains(&signature) {
            return Err("Signature already added".to_string());
        }
        
        tx.signatures.push(signature);
        Ok(())
    }

    pub fn execute_transaction(&mut self, tx_id: &str) -> Result<(), String> {
        let tx = self.transactions.get_mut(tx_id).ok_or("Transaction not found")?;
        let wallet = self.wallets.get(&tx.wallet_id).ok_or("Wallet not found")?;
        
        if tx.executed {
            return Err("Already executed".to_string());
        }
        if tx.signatures.len() < wallet.required_signatures as usize {
            return Err("Insufficient signatures".to_string());
        }
        
        tx.executed = true;
        Ok(())
    }
}
