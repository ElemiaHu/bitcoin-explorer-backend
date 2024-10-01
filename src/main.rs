// use bitcoincore_rpc::{Auth, Client, RpcApi};
mod db;

use tokio;
use db::db::{connect_to_mongo, get_block};

#[tokio::main]
async fn main() -> mongodb::error::Result<()> {

    // database endpoint
    let db_uri = "mongodb+srv://elemiahu:1YhnIfpV4xY2OBY1@cloud-db.v1eom.mongodb.net/?retryWrites=true&w=majority&appName=cloud-db";
    let db_client = connect_to_mongo(db_uri).await?;

    let block_hash = "0000000000000000019a92de18d9d2f3aa3bd59220a7bc615c109992993c58e7".to_string();
    if let Some(block) = get_block(&db_client, &block_hash).await? {
        println!("Block found: {:?}", block);
    } else {
        println!("Block not found");
    }

    // let rpc_url = "http://127.0.0.1:8332";

    // let rpc = Client::new(rpc_url,
    //         Auth::UserPass("pinkypunch".to_string(),
    //                     "radcat-xyqbo6-wimXut".to_string())).unwrap();
    // let best_block_hash = rpc.get_best_block_hash().unwrap();
    // println!("best block hash: {}", best_block_hash);

    // match rpc.get_block_count() {
    //     Ok(block_count) => {
    //         println!("Current block height: {}", block_count);
    //     },
    //     Err(e) => {
    //         eprintln!("Error fetching block count: {}", e);
    //     },
    // }

    // let tip = rpc.get_block_count().unwrap();
    // // let block = rpc.get_block(&tip).unwrap();
    // let stats = rpc.get_block_stats(tip).unwrap();
    // println!("block count: {}", tip);
    // println!("{}", stats.block_hash);

    // let db_uri = "mongodb+srv://elemiahu:1YhnIfpV4xY2OBY1@cloud-db.v1eom.mongodb.net/?retryWrites=true&w=majority&appName=cloud-db";
    // let client = Client::with_uri_str(db_uri).await?;
    // let database = client.database("BitcoinCore");
    // let collection: Collection<Document> = database.collection("Blocks");
    // let myblock = collection.find_one(doc! {"block_hash": "0000000000000000019a92de18d9d2f3aa3bd59220a7bc615c109992993c58e7"}).await?;
    // println!("Found a block: \n{:#?}", myblock);
    Ok(())
}
