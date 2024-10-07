mod db;
mod models;
mod bitcoin_rpc;

use tokio;
use tokio_postgres;
use tokio::signal;
use actix_web::{App, HttpServer, web, get, Responder, HttpResponse};
use serde::Serialize;
use std::sync::Arc;
use tokio::time::{sleep, Duration};
use bitcoincore_rpc::{Auth, Client as BitcoinClient};
use models::Block;
use bitcoin_rpc::{get_curr_block_height, get_block_info_by_height};
use db::{get_tip_db, insert_block, get_latest_10_blocks, get_blocks_from_height};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // change the db connection string
    let (db_client_instance, db_connection) = tokio_postgres::connect("host=localhost user=username dbname=bitcoincore password=password", tokio_postgres::NoTls).await?;
    let db_client = Arc::new(db_client_instance);
    // Spawn the connection in a separate task
    tokio::spawn(async move {
        if let Err(e) = db_connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    // connect to bitcoin core node
    let rpc_url = "http://127.0.0.1:8332";

    let rpc = BitcoinClient::new(rpc_url,
            Auth::UserPass("username".to_string(),
                        "password".to_string()))?;
    
    let db_client_cloned = Arc::clone(&db_client);
    tokio::spawn(async move {
        loop {
            // let db_client_cloned_loop = Arc::clone(&db_client);
            if let Err(e) = update_blocks(Arc::clone(&db_client_cloned), &rpc).await {
                eprintln!("Error updating blocks: {}", e);
            }
            sleep(Duration::from_secs(10 * 60)).await;
        }
    });

    let db_client_cloned_server = Arc::clone(&db_client);
    let server = HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(Arc::clone(&db_client_cloned_server)))
            .service(hello)
            .service(get_blocks)
            .service(get_blocks_in_range)
    })
    .bind("127.0.0.1:8080")?
    .run();

    let shutdown_signal = async {
        signal::ctrl_c().await.expect("Failed to listen for Ctrl+C");
        println!("Shutting down gracefully...");
    };

    tokio::select! {
        _ = server => {
            println!("Server has stopped.");
        }
        _ = shutdown_signal => {
            println!("Shutdown signal received, stopping server...");
        }
    }

    Ok(())
}

async fn update_blocks(
    db_client: Arc<tokio_postgres::Client>,
    rpc_client: &BitcoinClient) -> Result<(), Box<dyn std::error::Error>> {
    let tip = get_curr_block_height(rpc_client)?;
    let db_tip = get_tip_db(db_client.clone()).await?;
    let start_height = db_tip.unwrap_or(0) + 1;

    for height in start_height..=tip {
        let block = get_block_info_by_height(rpc_client, height)?;
        insert_block(db_client.clone(), block).await?;
        println!("inserted block {}", height);
    }

    Ok(())
}

#[get("/")]
async fn hello() -> impl Responder {
    "Hello, Actix Web!"
}

#[get("/blocks/latest")]
async fn get_blocks(client: web::Data<Arc<tokio_postgres::Client>>) -> impl Responder {
    match get_latest_10_blocks(client.get_ref().clone()).await {
        Ok(blocks) => HttpResponse::Ok().json(blocks),
        Err(err) => HttpResponse::InternalServerError().body(format!("Error: {}", err)),
    }
}

#[get("/blocks/range/{height}")]
async fn get_blocks_in_range(
    db_client: web::Data<Arc<tokio_postgres::Client>>,
    height: web::Path<u64>,
) -> impl Responder {
    let height = height.into_inner();
    match get_blocks_from_height(db_client.get_ref().clone(), height).await {
        Ok(blocks) => HttpResponse::Ok().json(blocks),
        Err(err) => HttpResponse::InternalServerError().body(format!("Error: {}", err)),
    }
}