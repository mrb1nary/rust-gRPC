use reqwest::Client;
use serde_json::json;
use tonic::Status;

pub async fn get_account_info(
    rpc_url: &str,
    pubkey: String,
) -> Result<(String, String, bool, u64, String), Status> {
    let payload = json!({
        "jsonrpc": "2.0",
        "id": 1,
        "method": "getAccountInfo",
        "params": [
            pubkey,
            {
                "encoding": "base64"
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

    println!("Helius getAccountInfo response: {}", json);

    if let Some(error) = json.get("error") {
        return Err(Status::internal(format!("RPC error: {}", error)));
    }

    let result = &json["result"]["value"];
    if result.is_null() {
        return Err(Status::not_found("Account not found"));
    }

    let lamports = result["lamports"].as_u64().unwrap_or_default().to_string();
    let owner = result["owner"].as_str().unwrap_or_default().to_string();
    let executable = result["executable"].as_bool().unwrap_or(false);
    let rent_epoch = result["rentEpoch"].as_u64().unwrap_or_default();
    let data = result["data"][0].as_str().unwrap_or_default().to_string();

    Ok((lamports, owner, executable, rent_epoch, data))
}
