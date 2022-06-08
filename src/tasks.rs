use crate::config::Config;
use crate::db::*;
use actix_web::rt::spawn;
use near_contract_standards::fungible_token::metadata::FungibleTokenMetadata;
use tokio::time::{self, Duration};
use tokio_postgres::NoTls;

async fn sync_fungible_tokens_metadata_task(config: &Config) {
    let near_pool = config.near_pg.create_pool(None, NoTls).unwrap();

    let near_client = near_pool.get().await.unwrap();

    let fp_pool = config.fp_pg.create_pool(None, NoTls).unwrap();

    let fp_client = fp_pool.get().await.unwrap();

    let interval = config.sync_metadata_interval;

    spawn(async move {
        let mut interval = time::interval(Duration::from_secs(interval));
        loop {
            interval.tick().await;

            match get_all_active_token_id_list(&near_client).await {
                Ok(ids) => {
                    for id in ids.iter() {
                        let mut metadata = &FungibleTokenMetadata {
                            spec: "".to_string(),
                            name: "".to_string(),
                            symbol: "".to_string(),
                            icon: None,
                            reference: None,
                            reference_hash: None,
                            decimals: 0,
                        };
                        let mut total_supply = 0u128;
                        let result = crate::rpc::get_fungible_token_metadata(id);
                        match result {
                            Ok(ref md) => metadata = md,
                            Err(err) => println!("{} metadata not find, error {}", id, err),
                        }

                        let result = crate::rpc::get_fungible_token_total_supply(id);
                        match result {
                            Ok(amount) => total_supply = amount.into(),
                            Err(err) => println!("{} total supply not find, error {}", id, err),
                        }

                        if let Ok(_) = get_fungible_tokens_by_id(&fp_client, id).await {
                            if let Err(err) = crate::db::update_fungible_tokens(
                                &fp_client,
                                id,
                                &total_supply.to_string(),
                                metadata,
                            )
                            .await
                            {
                                println!("insert fungible token {:?}", err);
                            }
                        } else {
                            if let Err(err) = crate::db::insert_fungible_tokens(
                                &fp_client,
                                id,
                                &total_supply.to_string(),
                                metadata,
                            )
                            .await
                            {
                                println!("insert fungible token {:?}", err);
                            }
                        }
                    }
                }
                Err(err) => {
                    println!("sync_fungible_tokens_metadata_task pg error {:?}", err);
                }
            }
        }
    });
}

async fn sync_fungible_tokens_burn_amount_task(config: &Config) {
    let near_pool = config.near_pg.create_pool(None, NoTls).unwrap();

    let near_client = near_pool.get().await.unwrap();

    let fp_pool = config.fp_pg.create_pool(None, NoTls).unwrap();

    let fp_client = fp_pool.get().await.unwrap();

    let interval = config.sync_burn_amount_interval;

    spawn(async move {
        let mut interval = time::interval(Duration::from_secs(interval));
        loop {
            interval.tick().await;
            if let Ok(vec) = get_fungible_tokens_burn_amount(&near_client).await {
                for (token_id, burn_amount) in vec.iter() {
                    if let Err(err) =
                        update_fungible_tokens_burn_amount(&fp_client, token_id, burn_amount).await
                    {
                        println!("insert fungible token burn amount error: {:?}", err);
                    }
                }
            } else {
                println!("sync_fungible_tokens_burn_amount_task pg error");
            }
        }
    });
}

async fn sync_fungible_tokens_holder_count_task(config: &Config) {
    let near_pool = config.near_pg.create_pool(None, NoTls).unwrap();

    let near_client = near_pool.get().await.unwrap();

    let fp_pool = config.fp_pg.create_pool(None, NoTls).unwrap();

    let fp_client = fp_pool.get().await.unwrap();

    let interval = config.sync_holder_count_interval;

    spawn(async move {
        let mut interval = time::interval(Duration::from_secs(interval));
        loop {
            interval.tick().await;
            if let Ok(vec) = get_fungible_tokens_holder_count(&near_client).await {
                for (token_id, holder_count) in vec.iter() {
                    if let Err(err) =
                        update_fungible_tokens_holder_count(&fp_client, token_id, holder_count)
                            .await
                    {
                        println!("insert fungible token holder count error: {:?}", err);
                    }
                }
            } else {
                println!("sync_fungible_tokens_holder_count_task pg error");
            }
        }
    });
}

async fn sync_fungible_tokens_transaction_data_task(config: &Config) {
    let near_pool = config.near_pg.create_pool(None, NoTls).unwrap();

    let near_client = near_pool.get().await.unwrap();

    let fp_pool = config.fp_pg.create_pool(None, NoTls).unwrap();

    let fp_client = fp_pool.get().await.unwrap();

    let interval = config.sync_transaction_data_interval;

    spawn(async move {
        let mut interval = time::interval(Duration::from_secs(interval));
        loop {
            interval.tick().await;
            if let Ok(vec) = get_fungible_tokens_transaction_data(&near_client).await {
                let _ = clear_fungible_tokens_transaction_data(&fp_client).await;
                for (token_id, transaction_count, transaction_amount) in vec.iter() {
                    if let Err(err) = update_fungible_tokens_transaction_data(
                        &fp_client,
                        token_id,
                        transaction_count,
                        transaction_amount,
                    )
                    .await
                    {
                        println!("insert fungible token transaction data  error: {:?}", err);
                    }
                }
            } else {
                println!("sync_fungible_tokens_transaction_data_task pg error");
            }
        }
    });
}

pub async fn run(config: &Config) {
    sync_fungible_tokens_metadata_task(config).await;
    sync_fungible_tokens_burn_amount_task(config).await;
    sync_fungible_tokens_holder_count_task(config).await;
    sync_fungible_tokens_transaction_data_task(config).await;
}
