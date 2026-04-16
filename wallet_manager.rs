use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use crate::crypto_ed25519::Ed25519Crypto;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Wallet {
    pub address: String,
    pub public_key: String,
    pub encrypted_private_key: String,
    pub balance: u128,
    pub nonce: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WalletCreateRequest {
    pub password: String,
    pub alias: Option<String>,
}

pub struct WalletManager {
    wallets: HashMap<String, Wallet>,
    key_store: HashMap<String, String>,
}

impl WalletManager {
    pub fn new() -> Self {
        Self {
            wallets: HashMap::new(),
            key_store: HashMap::new(),
        }
    }

    pub fn create_wallet(&mut self, password: &str) -> Result<String, String> {
        let crypto = Ed25519Crypto::new();
        let public_key = crypto.get_public_key_hex();
        let private_key = crypto.get_private_key_hex();
        let address = format!("0x{}", &public_key[0..40]);
        
        let encrypted_private = self.encrypt_private_key(&private_key, password);
        
        let wallet = Wallet {
            address: address.clone(),
            public_key,
            encrypted_private_key: encrypted_private,
            balance: 0,
            nonce: 0,
        };
        
        self.wallets.insert(address.clone(), wallet);
        self.key_store.insert(address.clone(), private_key);
        
        Ok(address)
    }

    fn encrypt_private_key(&self, private_key: &str, password: &str) -> String {
        use sha2::{Sha256, Digest};
        let mut hasher = Sha256::new();
        hasher.update(format!("{}{}", private_key, password));
        hex::encode(hasher.finalize())
    }

    pub fn get_wallet(&self, address: &str) -> Option<&Wallet> {
        self.wallets.get(address)
    }

    pub fn update_balance(&mut self, address: &str, amount: i128) -> Result<(), String> {
        let wallet = self.wallets.get_mut(address).ok_or("Wallet not found")?;
        if amount.is_negative() {
            let abs = amount.abs() as u128;
            if wallet.balance < abs {
                return Err("Insufficient balance".to_string());
            }
            wallet.balance -= abs;
        } else {
            wallet.balance += amount as u128;
        }
        Ok(())
    }

    pub fn sign_transaction(&self, address: &str, message: &[u8]) -> Result<String, String> {
        let private_key = self.key_store.get(address).ok_or("Wallet not unlocked")?;
        let crypto = Ed25519Crypto::from_seed(private_key)?;
        Ok(crypto.sign_message(message))
    }
}
