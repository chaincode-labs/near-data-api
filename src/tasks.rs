use crate::config::Config;
use crate::db::{get_all_active_token_id_list, get_fungible_tokens_burn_amount_by_id};
use crate::errors::TaskError;
use curl::easy::{Easy, List};
use deadpool_postgres::Client;
use near_contract_standards::fungible_token::metadata::FungibleTokenMetadata;
use near_sdk::json_types::U128;
use near_sdk::serde_json::json;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::io::Read;
use std::sync::Arc;
use std::thread;
use std::thread::sleep;
use tokio::spawn;
use tokio::time::{self, Duration};
use tokio_postgres::NoTls;


async fn sync_fungible_tokens_metadata_task(config: &Config) {
    let pool = config.pg.create_pool(None, NoTls).unwrap();

    let client = pool.get().await.unwrap();
    spawn( async move {
        let mut interval = time::interval(Duration::from_secs(10));
        loop {
            interval.tick().await;

            match get_all_active_token_id_list(&client).await {
                Ok(ids) => {
                    for id in ids.iter() {
                        let result = crate::rpc::get_fungible_token_metadata(id);
                        match result {
                            Ok(ft) => println!("{}", ft.name),
                            Err(err) => println!("{} metadata not find, error {}", id, err),
                        }

                        let result = crate::rpc::get_fungible_token_total_supply(id);
                        match result {
                            Ok(amount) => {
                                println!("{}:{:?}", id, serde_json::to_string(&amount).unwrap())
                            }
                            Err(err) => println!("{} total supply not find, error {}", id, err),
                        }
                    }
                }
                Err(err) => {
                    println!("pg error {:?}", err);
                }
            }
        }
    });
}

async fn sync_fungible_tokens_burn_amount_task(config: &Config) {
    let pool = config.pg.create_pool(None, NoTls).unwrap();
    let client = pool.get().await.unwrap();

    spawn( async move {
        let mut interval = time::interval(Duration::from_secs(10));
        loop {
            interval.tick().await;
            println!("sync_fungible_tokens_burn_amount_task");

        }
    });
}


async fn sync_fungible_tokens_holder_count_task(config: &Config) {
    let pool = config.pg.create_pool(None, NoTls).unwrap();
    let client = pool.get().await.unwrap();

   spawn( async move {
        let mut interval = time::interval(Duration::from_secs(10));
        loop {
            interval.tick().await;
            println!("sync_fungible_tokens_holder_count_task");

        }
    });

}


async fn sync_fungible_tokens_transaction_data_task(config: &Config) {
    let pool = config.pg.create_pool(None, NoTls).unwrap();
    let client = pool.get().await.unwrap();
    spawn( async move {
        let mut interval = time::interval(Duration::from_secs(10));
        loop {
            interval.tick().await;
            println!("sync_fungible_tokens_transaction_data_task");

        }
    });
}

pub async fn run(config: &Config) {
    sync_fungible_tokens_metadata_task(config).await;
    sync_fungible_tokens_holder_count_task(config).await;
    sync_fungible_tokens_burn_amount_task(config).await;
    sync_fungible_tokens_transaction_data_task(config).await;
}