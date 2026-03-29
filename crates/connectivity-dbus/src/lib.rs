use anyhow::Result;
use connectivity_app::context::AppContext;
use std::sync::Arc;

pub struct DbusServer {
    _ctx: Arc<AppContext>,
}

impl DbusServer {
    pub fn new(ctx: Arc<AppContext>) -> Self {
        Self { _ctx: ctx }
    }

    pub async fn run(self) -> Result<()> {
        Ok(())
    }
}
