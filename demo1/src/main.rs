use std::sync::{mpsc::Sender, Arc};

use anyhow::Result;
use grpc_demo_rs::{
    config::{Config, SharedState, TransactionInfo},
    logger,
    monitor::run_transaction_monitor,
};
use std::collections::HashMap;
use tokio::sync::broadcast;
use tokio::sync::Mutex;
use yellowstone_grpc_client::{ClientTlsConfig, GeyserGrpcClient};

#[tokio::main]
async fn main() -> Result<()> {
    logger::init_logger();
    // config
    let config = Config {
        grpc_endpoint: "solana-yellowstone-grpc.publicnode.com:443".to_string(),
        target_accounts: vec!["YOUR_TARGET_ACCOUNT".to_string()],
    };
    let state = Arc::new(SharedState {
        config,
        tx_history: Arc::new(Mutex::new(HashMap::new())),
    });

    // channel
    let (tx_sender, _) = broadcast::channel(512);
    let tx_recv1 = tx_sender.subscribe();
    let tx_recv2 = tx_sender.subscribe();
    let monitor_handle = tokio::spawn(run_transaction_monitor(state, tx_sender));

    tokio::try_join!(monitor_handle);
    Ok(())
}
