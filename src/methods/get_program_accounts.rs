use reqwest::Client;
use serde_json::json;
use tonic::Status;

use crate::indexer::AccountInfo;

pub async fn get_program_accounts(
    rpc_url: &str,
    program_id: String,
) -> Result<Vec<AccountInfo>, Status> {
    let payload = json!({
        "jsonrpc": "2.0",
        "id": 1,
        "method": "getProgramAccounts",
        "params": [
            program_id,
            {
                "encoding": "jsonParsed"
            }
        ]
    });

    let client = Client::new();
    let res = client
        .post(rpc_url)
        .json(&payload)
        .send()
        .await
        .map_err(|e| Status::internal(format!("HTTP error: {}", e)))?;

    let json: serde_json::Value = res
        .json()
        .await
        .map_err(|e| Status::internal(format!("JSON parse error: {}", e)))?;

    println!("Helius getProgramAccounts response: {}", json);

    if let Some(error) = json.get("error") {
        return Err(Status::internal(format!("RPC error: {}", error)));
    }

    let accounts_json = json["result"].as_array().ok_or_else(|| {
        Status::internal("Malformed response: 'result' is not an array")
    })?;

    let accounts = accounts_json.iter().filter_map(|entry| {
        let pubkey = entry["pubkey"].as_str()?.to_string();
        let account = &entry["account"];
        Some(AccountInfo {
            pubkey,
            lamports: account["lamports"].as_u64().unwrap_or_default().to_string(),
            owner: account["owner"].as_str().unwrap_or_default().to_string(),
            executable: account["executable"].as_bool().unwrap_or(false),
            rent_epoch: account["rentEpoch"].as_u64().unwrap_or_default().to_string(),
            data: account["data"].get(0)?.as_str()?.to_string(), // `data` is often [string, encoding]
        })
    }).collect::<Vec<_>>();

    Ok(accounts)
}
