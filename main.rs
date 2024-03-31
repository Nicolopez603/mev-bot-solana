use log::*;

fn main() {
    env_logger::init();
    
    info!("Starting Solana MEV Bot");
    
    let mut bot = SolanaMevBot::new(
        "https://api.mainnet-beta.solana.com",
        utils::solana::load_keypair("path/to/keypair.json"),
        HashMap::new(),
        0.01,
        vec![
            Box::new(dex::raydium::Raydium::new()),
            Box::new(dex::serum::Serum::new()),
            Box::new(dex::orca::Orca::new()),
        ],
    ).await;

    bot.run().await;
}