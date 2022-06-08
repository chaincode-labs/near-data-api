pub use ::config::ConfigError;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    pub server_addr: String,
    pub near_pg: deadpool_postgres::Config,
    pub fp_pg: deadpool_postgres::Config,
    pub sync_metadata_interval: u64,
    pub sync_burn_amount_interval: u64,
    pub sync_holder_count_interval: u64,
    pub sync_transaction_data_interval: u64,
}

impl Config {
    pub fn from_env() -> Result<Self, ConfigError> {
        let mut cfg = ::config::Config::new();
        cfg.merge(::config::Environment::new())?;
        cfg.try_into()
    }
}
