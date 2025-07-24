use reqwest::Client;
use serde_json::json;
use tonic::Status;

pub async fn get_transaction_info(
    rpc_url: &str,
    signature: String,
) -> Result<(String, String, String, Vec<String>), Status> {
    let payload = json!({
        "jsonrpc": "2.0",
        "id": 1,
        "method": "getTransaction",
        "params": [
            signature,
            {
                "maxSupportedTransactionVersion": 0,
                "encoding": "json"
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

    println!("Helius getTransaction response: {}", json);

    if let Some(error) = json.get("error") {
        return Err(Status::internal(format!("RPC error: {}", error)));
    }

    let result = &json["result"];
    let block_time = result["blockTime"].as_i64().unwrap_or_default().to_string();
    let slot = result["slot"].as_u64().unwrap_or_default().to_string();

    let meta = &result["meta"];
    let success = meta["err"].is_null();
    let _error_message = if success {
        "".to_string()
    } else {
        format!("{:?}", meta["err"])
    };

    let logs = meta["logMessages"]
        .as_array()
        .unwrap_or(&vec![])
        .iter()
        .filter_map(|v| v.as_str().map(|s| s.to_string()))
        .collect::<Vec<_>>();

    Ok((signature, block_time, slot, logs))
}
