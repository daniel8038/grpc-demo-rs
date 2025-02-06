use std::sync::Arc;

use anyhow::Result;
use demo1::{
    config::{Config, SharedState},
    logger,
    monitor::run_transaction_monitor,
};
use std::collections::HashMap;
use tokio::sync::broadcast;
use tokio::sync::Mutex;
use tracing::{error, info};

#[tokio::main]
async fn main() -> Result<()> {
    logger::init_logger();
    // config
    let config = Config {
        grpc_endpoint: "https://solana-yellowstone-grpc.publicnode.com:443".to_string(),
        target_accounts: vec!["9YwtWKdNczTzJHMbVdh1J3ZFWAVmYPpCPR7FwoMvZkVx".to_string()],
    };
    let state = Arc::new(SharedState {
        config,
        tx_history: Arc::new(Mutex::new(HashMap::new())),
    });

    // channel
    let (tx_sender, _) = broadcast::channel(512);
    let _tx_recv1 = tx_sender.subscribe();
    let _tx_recv2 = tx_sender.subscribe();
    info!("Spawning monitor task...");
    let monitor_handle = tokio::spawn({
        let state = state.clone();
        let tx_sender = tx_sender.clone();
        async move {
            if let Err(e) = run_transaction_monitor(state, tx_sender).await {
                error!("Monitor error: {}", e);
                return Err::<(), anyhow::Error>(e);
            }
            Ok(())
        }
    });
    info!("Waiting for monitor task...");
    match tokio::try_join!(monitor_handle) {
        Ok(_) => info!("Monitor task completed normally"),
        Err(e) => error!("Monitor task failed: {}", e),
    }

    Ok(())
}
