use tokio_postgres::{Client, Error};
use std::sync::Arc;
use crate::models::Block;

pub async fn get_tip_db(client: Arc<Client>) -> Result<Option<u64>, Error> {
    let query = "
        SELECT MAX(height)
        FROM blocks
    ";

    let row = client.query_one(query, &[]).await?;
    let max_height: Option<i64> = row.get(0);
    
    Ok(max_height.map(|h| h as u64))
}

pub async fn insert_block(client: Arc<Client>, block: Block) -> Result<u64, Error> {
    let query = "
        INSERT INTO blocks (block_hash, height, timestamp, size, merkle_root, num_transactions)
        VALUES ($1, $2, $3, $4, $5, $6)
    ";

    client.execute(query,
        &[
            &block.block_hash,
            &(block.height as i64),
            &(block.timestamp as i64),
            &(block.size as i64),  
            &block.merkle_root,
            &(block.num_transactions as i64),
        ]
    ).await
}

pub async fn get_latest_10_blocks(client: Arc<Client>) -> Result<Vec<Block>, Error> {
    let query = "
        SELECT block_hash, height, timestamp, size, merkle_root, num_transactions
        FROM blocks
        ORDER BY height DESC
        LIMIT 10
    ";

    let rows = client.query(query, &[]).await?;
    let blocks = rows
        .iter()
        .map(|row| Block {
            block_hash: row.get(0),
            height: row.get::<_, i64>(1) as u64,
            timestamp: row.get::<_, i64>(2) as u64,
            size: row.get::<_, i64>(3) as u64,
            merkle_root: row.get(4),
            num_transactions: row.get::<_, i64>(5) as u64,
        })
        .collect();

    Ok(blocks)
}

pub async fn get_blocks_from_height(client: Arc<Client>, height: u64) -> Result<Vec<Block>, Error> {
    let query = "
        SELECT block_hash, height, timestamp, size, merkle_root, num_transactions
        FROM blocks
        WHERE height BETWEEN $1::BIGINT - 9 AND $1::BIGINT
        ORDER BY height DESC
    ";

    let rows = client.query(query, &[&(height as i64)]).await?;
    let blocks = rows
        .iter()
        .map(|row| Block {
            block_hash: row.get(0),
            height: row.get::<_, i64>(1) as u64,  
            timestamp: row.get::<_, i64>(2) as u64,  
            size: row.get::<_, i64>(3) as u64,  
            merkle_root: row.get(4),
            num_transactions: row.get::<_, i64>(5) as u64,  
        })
        .collect();

    Ok(blocks)
}