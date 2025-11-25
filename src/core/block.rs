use chrono::{DateTime, Utc};
use crate::core::transactions::Tx;

pub struct Block {
    pub hash: [u8; 32],
    pub previous_hash : [u8; 32],
    pub txs : Vec<Tx>,
    pub timestamp: DateTime<Utc>,
}