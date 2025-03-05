mod connection;
mod event;
mod manager;

use std::sync::Arc;

pub use connection::router::RouterConnection;
pub use event::executor::EventExecutor;
pub use manager::QuerierManager;

use crate::common::meta::websocket_v2::StreamManager;

pub async fn init_ws_querier() -> anyhow::Result<()> {
    let event_executor = Arc::new(EventExecutor::new());
    let querier_manager = Arc::new(QuerierManager::new(event_executor.clone()));

    // Initialize router connection with config
    let router_conn = RouterConnection::new("ws://router-url".to_string(), event_executor);

    querier_manager
        .handle_connection(Box::new(router_conn))
        .await?;

    Ok(())
}
