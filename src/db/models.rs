use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Block {
    pub block_hash: String,
    pub height: i64,
    pub time: i64,
    pub size: i64,
    pub merkleroot: String,
    pub num_transactions: i64,
    pub transactions: Vec<String>,
}