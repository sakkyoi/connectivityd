use std::sync::Arc;

use anyhow::Result;
use connectivity_app::context::AppContext;
use connectivity_backend_bluez::BluezPeripheralBackend;
use connectivity_backend_nmrs::NmrsNetworkBackend;
use connectivity_config::Config;
use connectivity_dbus::DbusServer;
use connectivity_events::EventBus;
use connectivity_grpc::GrpcServer;
use connectivity_http::HttpServer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::from_default_env())
        .with(tracing_subscriber::fmt::layer())
        .init();

    let config = Config::default();
    let events = EventBus::new(256);

    let network_backend = Arc::new(NmrsNetworkBackend::new());
    let bluetooth_backend = Arc::new(BluezPeripheralBackend::new());

    let ctx = Arc::new(AppContext::new(network_backend, bluetooth_backend, events));

    let dbus = DbusServer::new(ctx.clone());
    let http = HttpServer::new(ctx.clone());
    let grpc = GrpcServer::new(ctx.clone());

    let http_addr = config.http_listen_addr.clone();
    let grpc_addr = config.grpc_listen_addr.clone();

    tokio::try_join!(dbus.run(), http.run(&http_addr), grpc.run(&grpc_addr))?;

    Ok(())
}
