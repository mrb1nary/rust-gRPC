use reqwest::Client;
use serde_json::json;
use tonic::{Response, Status};

use crate::indexer::SlotResponse;

pub async fn get_slot_info_handler(
    rpc_url: &str,
    slot: u64,
) -> Result<Response<SlotResponse>, Status> {
    let payload = json!({
        "jsonrpc": "2.0",
        "id": 1,
        "method": "getBlock",
        "params": [
            slot,
            { "maxSupportedTransactionVersion": 0 }
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

    println!("Helius JSON-RPC response: {}", json);

    if let Some(err) = json.get("error") {
        return Err(Status::internal(format!("RPC error: {}", err)));
    }

    let result = &json["result"];
    let blockhash = result["blockhash"]
        .as_str()
        .unwrap_or("unknown")
        .to_string();
    let previous_blockhash = result["previousBlockhash"]
        .as_str()
        .unwrap_or("unknown")
        .to_string();

    let resp = SlotResponse {
        slot,
        blockhash,
        previous_blockhash,
    };

    Ok(Response::new(resp))
}
