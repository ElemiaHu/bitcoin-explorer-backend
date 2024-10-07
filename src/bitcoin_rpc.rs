extern crate bitcoincore_rpc;

use bitcoincore_rpc::{bitcoin, Client, RpcApi, Error};
use bitcoin::Block as BitcoinBlock;
use crate::models::Block;

pub fn get_curr_block_height(rpc: &Client) -> Result<u64, Error> {
    let block_count = rpc.get_block_count()?;

    Ok(block_count)
}

// return a Block object
pub fn get_block_info_by_height(rpc: &Client, block_height: u64) -> Result<Block, Error> {

    // fetch block hash with block height and get related block info
    let block_hash = rpc.get_block_hash(block_height)?;
    let block: BitcoinBlock = rpc.get_block(&block_hash)?;

    // Extract block header information
    let block_header = block.header;
    let timestamp = block_header.time;
    let merkle_root = block_header.merkle_root;
    let serialized_block = bitcoin::consensus::encode::serialize(&block);

    let my_block = Block {
        block_hash: block_hash.to_string(),
        height: block_height as u64,
        timestamp: timestamp as u64,
        size: serialized_block.len() as u64,
        merkle_root: merkle_root.to_string(),
        num_transactions: block.txdata.len() as u64,
    };

    Ok(my_block)
}

// Function to calculate the total output amount of a transaction
// pub fn get_transaction_amount(tx: &Transaction) -> u64 {
//     tx.output.iter().map(|vout| vout.value.to_sat()).sum()
// }
