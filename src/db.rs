use crate::errors::MyError;
use crate::models::FungibleToken;
use deadpool_postgres::Client;
use near_contract_standards::fungible_token::metadata::FungibleTokenMetadata;

pub async fn get_all_active_token_id_list(client: &Client) -> Result<Vec<String>, MyError> {
    let sql = include_str!("../sql/all_active_fungible_tokens.sql");
    let stmt = client.prepare(&sql).await.unwrap();

    Ok(client
        .query(&stmt, &[])
        .await?
        .iter()
        .map(|row| row.get("token_id"))
        .collect::<Vec<String>>())
}

pub async fn get_fungible_tokens_burn_amount(
    client: &Client,
) -> Result<Vec<(String, String)>, MyError> {
    let sql = include_str!("../sql/get_assets__fungible_tokens_burn_amount.sql");
    let stmt = client.prepare(&sql).await.unwrap();

    Ok(client
        .query(&stmt, &[])
        .await?
        .iter()
        .map(|row| (row.get("token_id"), row.get("burn_amount")))
        .collect::<Vec<(String, String)>>())
}

pub async fn get_fungible_tokens_holder_count(
    client: &Client,
) -> Result<Vec<(String, i32)>, MyError> {
    let sql = include_str!("../sql/get_holder_count__assets_fungible_tokens.sql");
    let stmt = client.prepare(&sql).await.unwrap();

    Ok(client
        .query(&stmt, &[])
        .await?
        .iter()
        .map(|row| (row.get("token_id"), row.get("holder_count")))
        .collect::<Vec<(String, i32)>>())
}

pub async fn get_fungible_tokens_transaction_data(
    client: &Client,
) -> Result<Vec<(String, i32, String)>, MyError> {
    let sql = include_str!("../sql/get_assets__fungible_tokens_transaction_data.sql");
    let stmt = client.prepare(&sql).await.unwrap();

    Ok(client
        .query(&stmt, &[])
        .await?
        .iter()
        .map(|row| {
            (
                row.get("token_id"),
                row.get("transaction_count"),
                row.get("transaction_amount"),
            )
        })
        .collect::<Vec<(String, i32, String)>>())
}

pub async fn update_fungible_tokens_burn_amount(
    client: &Client,
    token_id: &String,
    burn_amount: &String,
) -> Result<(), MyError> {
    let mut _stmt = include_str!("../sql/update_assets__fungible_tokens_burn_amount.sql");

    let stmt = client.prepare(&_stmt).await.unwrap();

    client
        .query(&stmt, &[token_id, burn_amount])
        .await
        .map_or_else(|e| Err(MyError::PGError(e)), |_| Ok(()))
}

pub async fn update_fungible_tokens_holder_count(
    client: &Client,
    token_id: &String,
    holder_count: &i32,
) -> Result<(), MyError> {
    let mut _stmt = include_str!("../sql/update_assets__fungible_tokens_holder_count.sql");

    let stmt = client.prepare(&_stmt).await.unwrap();

    client
        .query(&stmt, &[token_id, holder_count])
        .await
        .map_or_else(|e| Err(MyError::PGError(e)), |_| Ok(()))
}

pub async fn update_fungible_tokens_transaction_data(
    client: &Client,
    token_id: &String,
    transaction_count: &i32,
    transaction_amount: &String,
) -> Result<(), MyError> {
    let mut _stmt = include_str!("../sql/update_assets__fungible_tokens_transaction_data.sql");

    let stmt = client.prepare(&_stmt).await.unwrap();

    client
        .query(&stmt, &[token_id, transaction_count, transaction_amount])
        .await
        .map_or_else(|e| Err(MyError::PGError(e)), |_| Ok(()))
}

pub async fn clear_fungible_tokens_transaction_data(
    client: &Client,
) -> Result<(), MyError> {
    let mut _stmt = include_str!("../sql/clear_assets__fungible_tokens_transaction_data.sql");

    let stmt = client.prepare(&_stmt).await.unwrap();

    client
        .query(&stmt, &[])
        .await
        .map_or_else(|e| Err(MyError::PGError(e)), |_| Ok(()))
}


pub async fn update_fungible_tokens(
    client: &Client,
    token_id: &String,
    total_supply: &String,
    metadata: &FungibleTokenMetadata,
) -> Result<(), MyError> {
    let mut _stmt = include_str!("../sql/update_assets__fungible_tokens.sql");

    let stmt = client.prepare(&_stmt).await.unwrap();

    client
        .query(
            &stmt,
            &[
                token_id,
                total_supply,
                &serde_json::to_string(metadata).unwrap(),
            ],
        )
        .await
        .map_or_else(|e| Err(MyError::PGError(e)), |_| Ok(()))
}

pub async fn insert_fungible_tokens(
    client: &Client,
    token_id: &String,
    total_supply: &String,
    metadata: &FungibleTokenMetadata,
) -> Result<(), MyError> {
    let mut _stmt = include_str!("../sql/insert_assets__fungible_tokens.sql");

    let stmt = client.prepare(&_stmt).await.unwrap();

    client
        .query(
            &stmt,
            &[
                token_id,
                total_supply,
                &serde_json::to_string(metadata).unwrap(),
            ],
        )
        .await
        .map_or_else(|e| Err(MyError::PGError(e)), |_| Ok(()))
}

pub async fn get_fungible_tokens_by_id(
    client: &Client,
    id: &String,
) -> Result<FungibleToken, MyError> {
    let _stmt = include_str!("../sql/get_assets__fungible_tokens_by_id.sql");
    let stmt = client.prepare(&_stmt).await.unwrap();

    if let Ok(row) = client.query_one(&stmt, &[id]).await {
        return Ok(FungibleToken::from_row_ref(&row));
    }
    return Err(MyError::NotFound);
}
