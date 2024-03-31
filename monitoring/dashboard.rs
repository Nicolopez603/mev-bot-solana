use crate::monitoring::metrics::Metrics;
use std::time::Duration;
use tokio::time;

pub struct Dashboard {
    pub metrics: Metrics,
}

impl Dashboard {
    pub fn new(metrics: Metrics) -> Self {
        Dashboard { metrics }
    }

    pub async fn run(&mut self) {
        loop {
            self.render_dashboard().await;
            time::sleep(Duration::from_secs(1)).await;
        }
    }

    async fn render_dashboard(&self) {
        let metrics = self.metrics.snapshot().await;

        println!("\n=== Solana MEV Bot Dashboard ===");
        println!("Timestamp: {}", metrics.timestamp);
        println!("Processed Blocks: {}", metrics.processed_blocks);
        println!("Processed Transactions: {}", metrics.processed_transactions);
        println!("Profitable Transactions: {}", metrics.profitable_transactions);
        println!("Unprofitable Transactions: {}", metrics.unprofitable_transactions);
        println!("Total Profit: {} SOL", metrics.total_profit);
        println!("Total Fees: {} SOL", metrics.total_fees);
        println!("Average Latency: {} ms", metrics.average_latency);
    }
}