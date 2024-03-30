use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    commitment_config::CommitmentConfig,
    pubkey::Pubkey,
    signature::{Keypair, Signature},
    signer::Signer,
    transaction::Transaction,
};

pub fn load_keypair(path: &str) -> Keypair {
    let bytes = std::fs::read(path).expect("Failed to read keypair file");
    Keypair::from_bytes(&bytes).expect("Failed to deserialize keypair")
}

pub async fn send_transaction(rpc_client: &RpcClient, transaction: &Transaction) -> Result<Signature, Box<dyn std::error::Error>> {
    let signature = rpc_client
        .send_and_confirm_transaction_with_spinner_and_commitment(transaction, CommitmentConfig::confirmed())
        .await?;
    Ok(signature)
}

pub async fn get_account_info(rpc_client: &RpcClient, pubkey: &Pubkey) -> Result<solana_client::client_error::ClientResult<solana_account_decoder::UiAccount>, Box<dyn std::error::Error>> {
    let account = rpc_client.get_account(pubkey).await?;
    Ok(account)
}