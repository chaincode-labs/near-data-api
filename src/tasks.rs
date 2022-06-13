use crate::config::Config;
use crate::db::*;
use actix_web::rt::spawn;
use deadpool_postgres::Object;
use near_contract_standards::fungible_token::metadata::FungibleTokenMetadata;
use std::sync::Arc;
use tokio::time::{self, Duration};
use tokio_postgres::NoTls;

pub struct TaskManager {
    config: Arc<Config>,
    near_client: Arc<Object>,
    fp_client: Arc<Object>,
}

impl TaskManager {
    pub async fn new(config: Arc<Config>) -> Self {
        let near_pool = config.near_pg.create_pool(None, NoTls).unwrap();
        let fp_pool = config.fp_pg.create_pool(None, NoTls).unwrap();

        Self {
            config,
            near_client: Arc::new(near_pool.get().await.unwrap()),
            fp_client: Arc::new(fp_pool.get().await.unwrap()),
        }
    }

    pub async fn run(&self) {
        self.sync_fungible_tokens_metadata_task().await;
        self.sync_fungible_tokens_burn_amount_task().await;
        self.sync_fungible_tokens_holder_count_task().await;
        self.sync_fungible_tokens_transaction_data_task().await;
    }

    async fn sync_fungible_tokens_metadata_task(&self) {
        let interval = self.config.sync_metadata_interval;
        let near_client = self.near_client.clone();
        let fp_client = self.fp_client.clone();

        spawn(async move {
            let mut interval = time::interval(Duration::from_secs(interval));
            loop {
                interval.tick().await;

                match get_all_active_token_id_list(near_client.as_ref()).await {
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

                            if let Ok(_) = get_fungible_tokens_by_id(fp_client.as_ref(), id).await {
                                if let Err(err) = crate::db::update_fungible_tokens(
                                    fp_client.as_ref(),
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
                                    fp_client.as_ref(),
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

    async fn sync_fungible_tokens_burn_amount_task(&self) {
        let interval = self.config.sync_burn_amount_interval;
        let near_client = self.near_client.clone();
        let fp_client = self.fp_client.clone();

        spawn(async move {
            let mut interval = time::interval(Duration::from_secs(interval));
            loop {
                interval.tick().await;
                if let Ok(vec) = get_fungible_tokens_burn_amount(near_client.as_ref()).await {
                    for (token_id, burn_amount) in vec.iter() {
                        if let Err(err) = update_fungible_tokens_burn_amount(
                            fp_client.as_ref(),
                            token_id,
                            burn_amount,
                        )
                        .await
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

    async fn sync_fungible_tokens_holder_count_task(&self) {
        let interval = self.config.sync_holder_count_interval;
        let near_client = self.near_client.clone();
        let fp_client = self.fp_client.clone();

        spawn(async move {
            let mut interval = time::interval(Duration::from_secs(interval));
            loop {
                interval.tick().await;
                if let Ok(vec) = get_fungible_tokens_holder_count(near_client.as_ref()).await {
                    for (token_id, holder_count) in vec.iter() {
                        if let Err(err) = update_fungible_tokens_holder_count(
                            fp_client.as_ref(),
                            token_id,
                            holder_count,
                        )
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

    async fn sync_fungible_tokens_transaction_data_task(&self) {
        let interval = self.config.sync_transaction_data_interval;
        let near_client = self.near_client.clone();
        let fp_client = self.fp_client.clone();

        spawn(async move {
            let mut interval = time::interval(Duration::from_secs(interval));
            loop {
                interval.tick().await;
                if let Ok(vec) = get_fungible_tokens_transaction_data(near_client.as_ref()).await {
                    let _ = clear_fungible_tokens_transaction_data(fp_client.as_ref()).await;
                    for (token_id, transaction_count, transaction_amount) in vec.iter() {
                        if let Err(err) = update_fungible_tokens_transaction_data(
                            fp_client.as_ref(),
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

}
