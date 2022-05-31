use crate::{errors::MyError, models::FTRank};
use deadpool_postgres::Client;
use tokio_pg_mapper::FromTokioPostgresRow;

pub async fn get_top_list(client: &Client) -> Result<Vec<FTRank>, MyError> {
    let sql = include_str!("../sql/rank.sql");
    let stmt = client.prepare(&sql).await.unwrap();

    Ok(client.query(&stmt, &[])
        .await?
        .iter()
        .map(|row| FTRank::from_row_ref(row).unwrap())
        .collect::<Vec<FTRank>>()
    )

}