use crate::dex::dex_integration::DexIntegration;
use async_trait::async_trait;
use solana_sdk::pubkey::Pubkey;
use std::collections::HashMap;

pub struct Raydium {}

#[async_trait]
impl DexIntegration for Raydium {
    async fn get_prices(&self) -> HashMap<String, f64> {
        HashMap::new()
    }

    async fn get_account_balances(&self, _account: &Pubkey) -> HashMap<String, f64> {
        HashMap::new()
    }

    async fn place_order(&self, _market: &str, _side: &str, _size: f64, _price: f64) -> Option<String> {
        None
    }

    async fn cancel_order(&self, _order_id: &str) -> bool {
        false
    }
}