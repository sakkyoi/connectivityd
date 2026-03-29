use anyhow::Result;
use connectivity_app::context::AppContext;
use std::sync::Arc;

pub struct HttpServer {
    _ctx: Arc<AppContext>,
}

impl HttpServer {
    pub fn new(ctx: Arc<AppContext>) -> Self {
        Self { _ctx: ctx }
    }

    pub async fn run(self, _listen_addr: &str) -> Result<()> {
        Ok(())
    }
}
