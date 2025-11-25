use serde::Serialize;

pub type Address = [u8; 20];

#[derive(Serialize, Debug, Clone)]
pub struct Tx {
    pub from: Address,
    pub to: Address,
    pub amount: u64,
    pub data: String,
    pub signature: Option<Vec<u8>>,
}