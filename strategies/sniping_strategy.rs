use crate::models::market_conditions::MarketConditions;
use crate::models::sniping_opportunity::SnipingOpportunity;
use crate::strategies::strategy::Strategy;
use async_trait::async_trait;
use solana_sdk::pubkey::Pubkey;
use std::collections::HashMap;

pub struct SnipingStrategy {
    pub rpc_client: solana_client::rpc_client::RpcClient,
    pub target_tokens: Vec<Pubkey>,
    pub max_price: f64,
}

impl SnipingStrategy {
    pub fn new(rpc_client: solana_client::rpc_client::RpcClient) -> Self {
        SnipingStrategy {
            rpc_client,
            target_tokens: Vec::new(),
            max_price: 0.0,
        }
    }

    pub fn add_target_token(&mut self, token_mint: Pubkey) {
        self.target_tokens.push(token_mint);
    }

    pub fn set_max_price(&mut self, max_price: f64) {
        self.max_price = max_price;
    }
}

#[async_trait]
impl Strategy for SnipingStrategy {
    fn update(&mut self, market_conditions: &MarketConditions) {
        self.max_price = market_conditions.max_price;
    }

    async fn find_opportunities(&self, target_accounts: &HashMap<Pubkey, crate::AccountInfo>) -> Vec<SnipingOpportunity> {
        let mut opportunities = Vec::new();
        
        for (token_mint, account_info) in target_accounts {
            if self.target_tokens.contains(token_mint) {
                let token_balance = account_info.token_balance;
                let token_price = account_info.token_price;
                
                if token_price <= self.max_price {
                    let opportunity = SnipingOpportunity {
                        target_account: *token_mint,
                        token_mint: *token_mint,
                        expected_price: token_price,
                        token_balance,
                    };
                    opportunities.push(opportunity);
                }
            }
        }

        opportunities
    }
}