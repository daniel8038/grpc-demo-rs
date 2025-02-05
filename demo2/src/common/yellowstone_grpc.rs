use {
    std::time::Duration,
    tokio::fs,
    tonic::transport::Certificate,
    yellowstone_grpc_client::{ClientTlsConfig, GeyserGrpcClient, Interceptor},
};

#[derive(Debug)]
pub struct GrpcConfig {
    // 认证相关
    pub ca_certificate: Option<String>,
    pub x_token: Option<String>,
    // 消息大小和缓冲区
    pub max_decoding_message_size: usize,
    pub buffer_size: Option<usize>,
    // 连接超时设置
    pub connect_timeout_ms: Option<u64>,
    pub timeout_ms: Option<u64>,
    // HTTP2 相关设置
    pub http2_adaptive_window: Option<bool>,
    pub http2_keep_alive_interval_ms: Option<u64>,
    pub initial_connection_window_size: Option<u32>,
    pub initial_stream_window_size: Option<u32>,
    // TCP 相关设置
    pub keep_alive_timeout_ms: Option<u64>,
    pub keep_alive_while_idle: Option<bool>,
    pub tcp_keepalive_ms: Option<u64>,
    pub tcp_nodelay: Option<bool>,
}
impl Default for GrpcConfig {
    fn default() -> Self {
        Self {
            ca_certificate: None,
            x_token: None,
            max_decoding_message_size: 16 * 1024 * 1024,
            buffer_size: None,
            connect_timeout_ms: None,
            timeout_ms: None,
            http2_adaptive_window: None,
            http2_keep_alive_interval_ms: None,
            initial_connection_window_size: None,
            initial_stream_window_size: None,
            keep_alive_timeout_ms: None,
            keep_alive_while_idle: None,
            tcp_keepalive_ms: None,
            tcp_nodelay: None,
        }
    }
}
pub struct Client {
    endpoint: String,
    config: GrpcConfig,
}
impl Client {
    pub fn new(endpoint: String, config: GrpcConfig) -> Self {
        Self { endpoint, config }
    }
    // client
    pub async fn connect(&self) -> anyhow::Result<GeyserGrpcClient<impl Interceptor>> {
        let mut tls_config = ClientTlsConfig::new().with_native_roots();
        if let Some(path) = &self.config.ca_certificate {
            let bytes = fs::read(path).await?;
            tls_config = tls_config.ca_certificate(Certificate::from_pem(bytes));
        }
        let mut builder = GeyserGrpcClient::build_from_shared(self.endpoint.clone())?
            .x_token(self.config.x_token.clone())?
            .tls_config(tls_config)?
            .max_decoding_message_size(self.config.max_decoding_message_size);

        if let Some(duration) = self.config.connect_timeout_ms {
            builder = builder.connect_timeout(Duration::from_millis(duration));
        }
        if let Some(sz) = self.config.buffer_size {
            builder = builder.buffer_size(sz);
        }
        if let Some(enabled) = self.config.http2_adaptive_window {
            builder = builder.http2_adaptive_window(enabled);
        }
        if let Some(duration) = self.config.http2_keep_alive_interval_ms {
            builder = builder.http2_keep_alive_interval(Duration::from_millis(duration));
        }
        if let Some(sz) = self.config.initial_connection_window_size {
            builder = builder.initial_connection_window_size(sz);
        }
        if let Some(sz) = self.config.initial_stream_window_size {
            builder = builder.initial_stream_window_size(sz);
        }
        if let Some(duration) = self.config.keep_alive_timeout_ms {
            builder = builder.keep_alive_timeout(Duration::from_millis(duration));
        }
        if let Some(enabled) = self.config.keep_alive_while_idle {
            builder = builder.keep_alive_while_idle(enabled);
        }
        if let Some(duration) = self.config.tcp_keepalive_ms {
            builder = builder.tcp_keepalive(Some(Duration::from_millis(duration)));
        }
        if let Some(enabled) = self.config.tcp_nodelay {
            builder = builder.tcp_nodelay(enabled);
        }
        if let Some(duration) = self.config.timeout_ms {
            builder = builder.timeout(Duration::from_millis(duration));
        }

        builder.connect().await.map_err(Into::into)
    }
}
