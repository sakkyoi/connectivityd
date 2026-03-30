use anyhow::Result;
use connectivity_app::context::AppContext;
use std::sync::Arc;
use zbus::connection::Builder as ConnectionBuilder;

use crate::network::{NetworkObject, WifiObject};

pub struct DbusServer {
    ctx: Arc<AppContext>,
}

impl DbusServer {
    pub fn new(ctx: Arc<AppContext>) -> Self {
        Self { ctx }
    }

    pub async fn run(self) -> Result<()> {
        let _connection = ConnectionBuilder::system()?
            .name("com.example.Connectivity")?
            .serve_at("/com/example/Connectivity/Network", NetworkObject::new(self.ctx.clone()))?
            .serve_at("/com/example/Connectivity/WiFi", WifiObject::new(self.ctx.clone()))?
            .build()
            .await?;

        std::future::pending::<()>().await;
        #[allow(unreachable_code)]
        Ok(())
    }
}
