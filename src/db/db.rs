use mongodb::{ 
	bson::{Document, doc},
	Client,
	Collection 
};
use super::models::{Block};

pub async fn connect_to_mongo(uri: &str) -> mongodb::error::Result<Client> {
    let client = Client::with_uri_str(uri).await?;
    Ok(client)
}

pub async fn get_block(client: &Client, block_hash: &str) -> mongodb::error::Result<Option<Block>> {
    let database = client.database("BitcoinCore");
    let collection: Collection<Block> = database.collection("Blocks");

    let block = collection.find_one(doc! {"block_hash": block_hash}).await?;
    Ok(block)
}

// pub async fn insert_block(client: &Client, block: Block) -> mongodb::error::Result<()> {
//     let db = client.database("bitcoin_explorer");
//     let blocks_collection = db.collection("blocks");

//     blocks_collection.insert_one(block, None).await?;
//     Ok(())
// }
