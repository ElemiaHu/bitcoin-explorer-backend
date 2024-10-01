extern crate bitcoincore_rpc;

use bitcoincore_rpc::{bitcoin, Auth, Client, Error, RpcApi};
use bitcoin::Transaction;

pub fn get_block_info_by_height(rpc: &Client, block_height: u64) -> Result<(), Error> {
    // Get the block hash by block height
    let block_hash = rpc.get_block_hash(block_height)?;
    println!("Block hash at height {}: {}", block_height, block_hash);

    // Fetch the block using the block hash
    let block: bitcoin::Block = rpc.get_block(&block_hash)?;

    // Extract block header information
    let block_header = block.header;
    let merkle_root = block_header.merkle_root;
    let timestamp = block_header.time;  // UNIX timestamp

    // Output the extracted information
    println!("Block height: {}", block_height);
    println!("Block hash: {}", block_hash);
    println!("Merkle root: {}", merkle_root);
    println!("Timestamp (UNIX): {}", timestamp);
    println!("Number of transactions: {}", block.txdata.len());

    // List transactions
    // for (i, tx) in block.txdata.iter().enumerate() {
    //     let tx_hash = tx.compute_txid();
    //     let tx_amount = get_transaction_amount(tx);
    //     println!("Transaction {}: ", i + 1);
    //     println!("  Transaction hash: {}", tx_hash);
    //     println!("  Transaction amount: {} BTC", tx_amount as f64 / 100_000_000.0); // Convert from satoshis to BTC
    // }

    Ok(())
}

// Function to calculate the total output amount of a transaction
pub fn get_transaction_amount(tx: &Transaction) -> u64 {
    tx.output.iter().map(|vout| vout.value.to_sat()).sum()
}
