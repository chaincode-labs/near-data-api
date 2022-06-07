use bigdecimal::BigDecimal;
use crate::models::FungibleToken;
use deadpool_postgres::Client;
use tokio_pg_mapper::FromTokioPostgresRow;
use tokio_postgres::Statement;
use crate::errors::MyError;

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

pub async fn update_fungible_tokens(
    client: &Client,
    token: FungibleToken,
) -> Result<FungibleToken, MyError> {
    let mut _stmt = include_str!("../sql/insert_assets__fungible_tokens.sql");

    if let Ok(t) = get_fungible_tokens_by_id(client, token.token_id.clone()).await {
        _stmt = include_str!("../sql/update_assets__fungible_tokens.sql");
    }

    let _stmt = _stmt.replace("$table_fields", &FungibleToken::sql_table_fields());
    let stmt = client.prepare(&_stmt).await.unwrap();

    if let Ok(row) = client
        .query_one(
            &stmt,
            &[
                &token.token_id,
                &token.total_supply,
                &token.burn_amount,
                //&token.metadata,
                &token.holder_count,
            ],
        )
        .await
    {
        //FungibleToken::from_row_ref(&row).map_or(|t| Ok(t))
    }
    return Err(MyError::NotFound);
}

pub async fn get_fungible_tokens_by_id(
    client: &Client,
    id: String,
) -> Result<FungibleToken, MyError> {
    let _stmt = include_str!("../sql/get_assets__fungible_tokens.sql");
    let _stmt = _stmt.replace("$table_fields", &FungibleToken::sql_table_fields());
    let stmt = client.prepare(&_stmt).await.unwrap();

    if let Ok(row) = client.query_one(&stmt, &[&id]).await {
        //FungibleToken::from_row_ref(&row).map_or(|t| Ok(t))
    }
    return Err(MyError::NotFound);
}

pub async fn get_fungible_tokens_burn_amount_by_id(
    client: &Client,
    id: String,
) -> Result<String, MyError> {
    let _stmt = include_str!("../sql/get_assets__fungible_tokens_burn_amount.sql");
    let stmt = client.prepare(&_stmt).await.unwrap();

    if let Ok(row) = client.query_one(&stmt, &[&id]).await {
       return Ok(row.get("burn_amount"))
    }
    return Err(MyError::NotFound);
}