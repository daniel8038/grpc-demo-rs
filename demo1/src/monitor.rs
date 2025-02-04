use crate::config::{SharedState, TransactionInfo};
use anyhow::{bail, Result};
use chrono::Local;
use futures::{sink::SinkExt, stream::StreamExt};
use std::sync::Arc;
use tokio::sync::broadcast;
use tracing::{error, info};
use yellowstone_grpc_client::{ClientTlsConfig, GeyserGrpcClient};
use yellowstone_grpc_proto::{
    prelude::{
        subscribe_update::UpdateOneof, SubscribeRequest, SubscribeRequestFilterTransactions,
        SubscribeRequestPing, SubscribeUpdate,
    },
    tonic::{Status, Streaming},
};

const PUMP_FUN_PROGRAM_ID: &str = "6EF8rrecthR5Dkzon8Nwu78hRvfCKubJ14M5uBEwF6P";

pub async fn run_transaction_monitor(
    state: Arc<SharedState>,
    tx_sender: broadcast::Sender<TransactionInfo>,
) -> Result<()> {
    // sub
    let mut client = GeyserGrpcClient::build_from_shared(state.config.grpc_endpoint.clone())?
        .tls_config(ClientTlsConfig::new().with_native_roots())?
        .connect()
        .await?;

    let (mut subscribe_tx, mut stream) = client.subscribe().await?;

    // request config
    let sud_req: SubscribeRequest = SubscribeRequest {
        transactions: maplit::hashmap!("test_demo".to_owned() => SubscribeRequestFilterTransactions {
             // 聪明钱地址
          account_include: state.config.target_accounts.clone(),
          // 必须 与 PumpFun程序相关
          account_required: vec![PUMP_FUN_PROGRAM_ID.to_string()],
          // 过滤掉失败的交易
          failed:false.into(),
          ..Default::default()
        }),
        commitment: Some(0),
        ..Default::default()
    };

    // sub
    subscribe_tx.send(sud_req).await?;
    // stream
    while let Some(message) = stream.next().await {
        match message {
            Ok(msg) => match msg.update_oneof {
                Some(UpdateOneof::Transaction(sut)) => {}
                Some(UpdateOneof::Ping(_)) => {
                    let _ = subscribe_tx
                        .send(SubscribeRequest {
                            ping: Some(SubscribeRequestPing { id: 1 }),
                            ..Default::default()
                        })
                        .await;
                    info!("service is ping: {}", Local::now());
                }
                Some(UpdateOneof::Pong(_)) => {
                    info!("service is pong: {}", Local::now());
                }
                _ => {}
            },
            Err(err) => {
                error!("获取信息流错误：{}", err);
                bail!(format!("获取信息流错误：{}", err))
            }
        }
    }
    Ok(())
}
