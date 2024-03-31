use solana_sdk::pubkey::Pubkey;

pub struct SnipingOpportunity {
    pub target_account: Pubkey,
    pub token_mint: Pubkey,
    pub expected_price: f64,
    pub token_balance: f64,
}