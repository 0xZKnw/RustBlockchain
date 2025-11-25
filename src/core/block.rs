use chrono::{DateTime, Utc};
use sha3::{Digest, Keccak256};
use serde::Serialize;
use crate::core::{transactions::Tx};

#[derive(Serialize, Debug)]
pub struct Block {
    pub hash: [u8; 32],
    pub previous_hash : [u8; 32],
    pub txs : Vec<Tx>,
    pub timestamp: DateTime<Utc>,
    pub data: String,
}

impl Block {
    pub fn calcul_hash(&self) -> [u8; 32] {
        let mut hasher = Keccak256::new();

        let data_to_hash = (
            &self.previous_hash,
            &self.txs,
            &self.timestamp.timestamp(),
            &self.data,
        );

        let serialized_data = bincode::serialize(&data_to_hash).unwrap();

        hasher.update(&serialized_data);

        let result = hasher.finalize();
        result.into()
    }
}

pub fn new_block(txs : Vec<Tx>, previous_hash : [u8; 32], data: String) -> Block {
    let now_utc = Utc::now();

    let mut new_block = Block {
        hash: [0; 32],
        previous_hash,
        txs,
        timestamp: now_utc,
        data,
    };
    new_block.hash = new_block.calcul_hash();

    new_block
}