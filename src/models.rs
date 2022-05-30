use serde::{Deserialize, Serialize};
use tokio_pg_mapper_derive::PostgresMapper;

#[derive(Deserialize, PostgresMapper, Serialize)]
#[pg_mapper(table = "assets__fungible_token_events")]
pub struct FTRank {
    pub token_id: String,
    pub transaction_count: i64,
}