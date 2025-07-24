use tokio::time::{sleep, Duration};
use tonic::{Request, Response, Status};
use tokio_stream::{wrappers::ReceiverStream, Stream};
use std::pin::Pin;

use crate::{indexer::{SlotResponse, StreamSlotRequest}, methods::get_slot_info_handler};



pub type SlotStream = Pin<Box<dyn Stream<Item = Result<SlotResponse, Status>> + Send>>;

pub async fn stream_slot_info(
    rpc_url: String,
    request: Request<StreamSlotRequest>,
) -> Result<Response<SlotStream>, Status> {
    let interval_ms = request.into_inner().interval_ms;
    let (tx, rx) = tokio::sync::mpsc::channel(10);

    tokio::spawn(async move {
        let mut last_slot = 0;

        loop {
            match get_slot_height(&rpc_url).await {
                Ok(current_slot) => {
                    if current_slot > last_slot {
                        last_slot = current_slot;

                        // Fetch full slot info
                        match get_slot_info_handler(&rpc_url, current_slot).await {
                            Ok(response) => {
                                if tx.send(Ok(response.into_inner())).await.is_err() {
                                    break;
                                }
                            }
                            Err(e) => {
                                let _ = tx.send(Err(Status::internal(format!(
                                    "Slot info error: {}",
                                    e
                                ))))
                                .await;
                                break;
                            }
                        }
                    }
                }
                Err(_) => {
                    // Optional: handle RPC error or continue silently
                }
            }

            sleep(Duration::from_millis(interval_ms)).await;
        }
    });

    Ok(Response::new(Box::pin(ReceiverStream::new(rx)) as SlotStream))
}

// You can implement a basic `get_slot_height` using `getSlot` RPC
async fn get_slot_height(rpc_url: &str) -> Result<u64, Status> {
    let payload = serde_json::json!({
        "jsonrpc": "2.0",
        "id": 1,
        "method": "getSlot"
    });

    let res = reqwest::Client::new()
        .post(rpc_url)
        .json(&payload)
        .send()
        .await
        .map_err(|e| Status::internal(format!("Slot height HTTP error: {}", e)))?;

    let json: serde_json::Value = res
        .json()
        .await
        .map_err(|e| Status::internal(format!("Slot height JSON error: {}", e)))?;

    if let Some(error) = json.get("error") {
        return Err(Status::internal(format!("RPC error: {}", error)));
    }

    Ok(json["result"].as_u64().unwrap_or(0))
}
