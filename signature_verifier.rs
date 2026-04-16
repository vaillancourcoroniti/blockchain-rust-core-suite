use crate::crypto_ed25519::Ed25519Crypto;
use crate::transaction_pool::ChainTransaction;
use sha2::{Sha256, Digest};

pub struct SignatureVerifier;

impl SignatureVerifier {
    pub fn verify_transaction(tx: &ChainTransaction) -> Result<bool, String> {
        let message = Self::get_transaction_signing_message(tx);
        let crypto = Ed25519Crypto::new();
        crypto.verify_message(&message, &tx.signature)
    }

    fn get_transaction_signing_message(tx: &ChainTransaction) -> Vec<u8> {
        let data = format!(
            "{}{}{}{}{}{}",
            tx.sender, tx.receiver, tx.amount, tx.fee, tx.nonce, tx.timestamp
        );
        let mut hasher = Sha256::new();
        hasher.update(data.as_bytes());
        hasher.finalize().to_vec()
    }

    pub fn verify_batch(transactions: &[ChainTransaction]) -> Vec<(String, bool)> {
        transactions.iter()
            .map(|tx| (tx.tx_id.clone(), Self::verify_transaction(tx).unwrap_or(false)))
            .collect()
    }

    pub fn is_valid_public_key(pub_key_hex: &str) -> bool {
        pub_key_hex.len() == 64 && hex::decode(pub_key_hex).is_ok()
    }

    pub fn is_valid_signature(sig_hex: &str) -> bool {
        sig_hex.len() == 128 && hex::decode(sig_hex).is_ok()
    }
}
