use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Debug, Clone)]
pub struct Config {
    pub grpc_endpoint: String,
    pub target_accounts: Vec<String>,
}
#[derive(Debug, Clone)]
pub struct TransactionInfo {
    pub signature: String,
}
pub struct SharedState {
    pub config: Config,
    pub tx_history: Arc<Mutex<HashMap<String, TransactionInfo>>>,
}
