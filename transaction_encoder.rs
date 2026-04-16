use serde::{Serialize, Deserialize};
use crate::transaction_pool::ChainTransaction;
use base64::{encode, decode};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EncodedTransaction {
    pub raw: String,
    pub tx_id: String,
    pub version: u8,
}

pub struct TransactionEncoder;

impl TransactionEncoder {
    pub fn encode(tx: &ChainTransaction) -> EncodedTransaction {
        let json = serde_json::to_string(tx).unwrap();
        let raw = encode(json);
        
        EncodedTransaction {
            raw,
            tx_id: tx.tx_id.clone(),
            version: 1,
        }
    }

    pub fn decode(encoded: &EncodedTransaction) -> Result<ChainTransaction, String> {
        let json_bytes = decode(&encoded.raw).map_err(|e| e.to_string())?;
        let tx = serde_json::from_slice(&json_bytes)
            .map_err(|e| e.to_string())?;
        Ok(tx)
    }

    pub fn encode_batch(txs: &[ChainTransaction]) -> Vec<EncodedTransaction> {
        txs.iter().map(Self::encode).collect()
    }

    pub fn decode_batch(encoded: &[EncodedTransaction]) -> Result<Vec<ChainTransaction>, String> {
        encoded.iter().map(Self::decode).collect()
    }

    pub fn get_raw_size(encoded: &EncodedTransaction) -> usize {
        encoded.raw.len()
    }
}
