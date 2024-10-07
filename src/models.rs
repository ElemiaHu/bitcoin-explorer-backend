#[derive(serde::Serialize)]
pub struct Block {
    pub block_hash: String,
    pub height: u64,
    pub timestamp: u64,
    pub size: u64,
    pub merkle_root: String,
    pub num_transactions: u64,
}