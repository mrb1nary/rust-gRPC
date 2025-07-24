use crate::indexer::solana_indexer_server::SolanaIndexer;
use crate::indexer::{
    AccountRequest, AccountResponse, ProgramAccountsRequest, ProgramAccountsResponse, SlotRequest, SlotResponse, TransactionRequest, TransactionResponse
};
use crate::methods::{get_account_info, get_program_accounts, get_slot_info_handler, get_transaction_info};
use std::env;
use tonic::{Request, Response, Status};

pub struct IndexerService {
    rpc_url: String,
}

impl IndexerService {
    pub fn new() -> Self {
        let rpc_url = env::var("HELIUS_RPC_URL").expect("HELIUS_RPC_URL must be set");
        IndexerService { rpc_url }
    }
}

#[tonic::async_trait]
impl SolanaIndexer for IndexerService {
    //----------------------------------------------------------------//
    async fn get_slot_info(
        &self,
        request: Request<SlotRequest>,
    ) -> Result<Response<SlotResponse>, Status> {
        let slot = request.into_inner().slot;
        get_slot_info_handler(&self.rpc_url, slot).await
    }

    //----------------------------------------------------------------//
    async fn get_transaction_info(
        &self,
        request: Request<TransactionRequest>,
    ) -> Result<Response<TransactionResponse>, Status> {
        let signature = request.into_inner().signature;

        match get_transaction_info(&self.rpc_url, signature).await {
            Ok((signature, block_time, slot, logs)) => {
                let response = TransactionResponse {
                    signature,
                    block_time,
                    slot,
                    logs,
                };
                Ok(Response::new(response))
            }
            Err(e) => Err(Status::internal(format!(
                "Failed to fetch transaction info: {}",
                e
            ))),
        }
    }

    //----------------------------------------------------------------//
    async fn get_account_info(
        &self,
        request: Request<AccountRequest>,
    ) -> Result<Response<AccountResponse>, Status> {
        let pubkey = request.into_inner().pubkey;
        let (lamports, owner, executable, rent_epoch, data) =
            get_account_info(&self.rpc_url, pubkey).await?;

        Ok(Response::new(AccountResponse {
            lamports,
            owner,
            executable,
            rent_epoch,
            data,
        }))
    }

    //----------------------------------------------------------------//
    async fn get_program_accounts(
        &self,
        request: Request<ProgramAccountsRequest>,
    ) -> Result<Response<ProgramAccountsResponse>, Status> {
        let program_id = request.into_inner().program_id;
        let accounts = get_program_accounts(&self.rpc_url, program_id).await?;

        Ok(Response::new(ProgramAccountsResponse { accounts }))
    }
}
