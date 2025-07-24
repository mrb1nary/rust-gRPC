use std::net::SocketAddr;
use tonic::transport::Server;

mod service;
mod methods;
mod indexer {
    tonic::include_proto!("indexer");
}
use indexer::solana_indexer_server::SolanaIndexerServer;
use service::IndexerService;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();
    env_logger::init();

    let addr: SocketAddr = "127.0.0.1:50051".parse()?;
    println!("gRPC listening on {}", addr);

    let indexer = IndexerService::new();

    Server::builder()
        .add_service(SolanaIndexerServer::new(indexer))
        .serve(addr)
        .await?;

    Ok(())
}
