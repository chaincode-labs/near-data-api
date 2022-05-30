mod models {
    use serde::{Deserialize, Serialize};
    use tokio_pg_mapper_derive::PostgresMapper;

    #[derive(Deserialize, PostgresMapper, Serialize)]
    #[pg_mapper(table = "assets__fungible_token_events")] // singular 'user' is a keyword..
    pub(crate) struct FTRank {
        pub token_id: String,
        pub count: u32,
    }
}