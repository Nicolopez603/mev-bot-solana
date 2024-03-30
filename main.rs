mod bot;
mod dex;
mod strategies;
mod models;
mod utils;

use bot::solana_mev_bot::SolanaMevBot;

#[tokio::main]
async fn main() {
    // Configuración inicial y puesta en marcha del bot de MEV de Solana mejorado
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