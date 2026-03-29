use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub http_listen_addr: String,
    pub grpc_listen_addr: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            http_listen_addr: "127.0.0.1:8080".to_string(),
            grpc_listen_addr: "127.0.0.1:50051".to_string(),
        }
    }
}
