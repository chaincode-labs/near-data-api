use near_contract_standards::fungible_token::metadata::FungibleTokenMetadata;
use near_sdk::serde_json;
use serde::{Deserialize, Serialize};
use tokio_postgres::Row;

#[derive(Deserialize, Serialize)]
pub struct FungibleToken {
    pub token_id: String,
    pub total_supply: String,
    pub burn_amount: String,
    pub metadata: FungibleTokenMetadata,
    pub holder_count: i32,
    pub transaction_count: i32,
    pub transaction_amount: i32,
}

impl FungibleToken {
    pub fn from_row_ref(row: &Row) -> Self {
        Self {
            token_id: row.get("token_id"),
            total_supply: row.get("total_supply"),
            burn_amount: row.get("burn_amount"),
            metadata: serde_json::from_str(row.get("metadata")).unwrap(),
            holder_count: row.get("holder_count"),
            transaction_count: 0,
            transaction_amount: 0,
        }
    }
}
