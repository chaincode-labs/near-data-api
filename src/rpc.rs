use crate::errors::TaskError;
use curl::easy::{Easy, List};
use near_contract_standards::fungible_token::metadata::FungibleTokenMetadata;
use near_sdk::json_types::U128;
use near_sdk::serde_json::json;
use serde::{Deserialize};
use std::io::Read;

#[derive(Deserialize)]
struct BinData {
    pub result: Vec<u8>,
}

#[derive(Deserialize)]
struct JsonData {
    pub result: BinData,
}

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn request_json_rpc_with_body(body: &[u8]) -> Result<Vec<u8>> {
    let mut list = List::new();
    list.append("Content-Type: application/json")?;
    let mut easy = Easy::new();
    easy.url("https://rpc.mainnet.near.org")?;
    easy.http_headers(list)?;
    easy.post(true)?;
    easy.post_field_size(body.len() as u64)?;

    let mut data = Vec::new();
    {
        // Create transfer in separate scope ...
        let mut transfer = easy.transfer();

        // Request body
        transfer.read_function(|buf| Ok(body.as_ref().read(buf).unwrap_or(0)))?;

        // Response body
        transfer.write_function(|new_data| {
            data.extend_from_slice(new_data);
            Ok(new_data.len())
        })?;

        transfer.perform()?;
    }

    println!("{}", easy.response_code()?);
    println!("Received {} bytes", data.len());

    Ok(data)
}

pub fn get_fungible_token_metadata(id: &String) -> Result<FungibleTokenMetadata> {
    let json = json!({
            "jsonrpc": "2.0",
            "id": "dontcare",
            "method": "query",
            "params": {
            "request_type": "call_function",
            "finality": "final",
            "account_id":  id,
            "method_name": "ft_metadata",
             "args_base64": ""
            }
    });

    let str = json.to_string();
    let body = str.as_bytes();
    let data = request_json_rpc_with_body(body)?;
    if !data.is_empty() {
        if let Ok(json_value) = serde_json::from_slice::<JsonData>(data.as_slice()) {
            if let Ok(ft) =
                serde_json::from_slice::<FungibleTokenMetadata>(json_value.result.result.as_slice())
            {
                return Ok(ft);
            }
        }
    }
    return Err(TaskError::FungibleTokenMetadataNotFound.into());
}

pub fn get_fungible_token_total_supply(id: &String) -> Result<U128> {
    let json = json!({
            "jsonrpc": "2.0",
            "id": "dontcare",
            "method": "query",
            "params": {
            "request_type": "call_function",
            "finality": "final",
            "account_id":  id,
            "method_name": "ft_total_supply",
             "args_base64": ""
            }
    });

    let str = json.to_string();
    let body = str.as_bytes();
    let data = request_json_rpc_with_body(body)?;
    if !data.is_empty() {
        if let Ok(json_value) = serde_json::from_slice::<JsonData>(data.as_slice()) {
            if let Ok(b) = serde_json::from_slice::<U128>(json_value.result.result.as_slice()) {
                return Ok(b);
            }
        }
    }
    return Err(TaskError::FungibleTokenTotalSupplyNotFound.into());
}
