use curl::easy::{Easy, List};
use near_contract_standards::fungible_token::metadata::FungibleTokenMetadata;
use near_sdk::json_types::U128;
use near_sdk::serde_json::json;
use serde::{Deserialize};
use std::io::Read;
use serde::de::DeserializeOwned;

#[derive(Deserialize)]
struct BinData {
    pub result: Vec<u8>,
}

#[derive(Deserialize)]
struct JsonData {
    pub result: BinData,
}

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn request_json_rpc_with_body<D: DeserializeOwned >(body: &[u8]) -> Result<D> {
    let mut list = List::new();
    list.append("Content-Type: application/json")?;
    let mut easy = Easy::new();
    easy.url("https://rpc.mainnet.near.org")?;
    easy.http_headers(list)?;
    easy.post(true)?;
    easy.post_field_size(body.len() as u64)?;

    let mut data = Vec::new();
    {
        let mut transfer = easy.transfer();
        transfer.read_function(|buf| Ok(body.as_ref().read(buf).unwrap_or(0)))?;
        transfer.write_function(|new_data| {
            data.extend_from_slice(new_data);
            Ok(new_data.len())
        })?;

        transfer.perform()?;
    }

    println!("{}", easy.response_code()?);
    println!("Received {} bytes", data.len());
    let v = serde_json::from_slice::<JsonData>(data.as_slice())?;
    Ok(serde_json::from_slice::<D>(v.result.result.as_slice())?)
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
    request_json_rpc_with_body::<FungibleTokenMetadata>(body)
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
    request_json_rpc_with_body::<U128>(body)
}
