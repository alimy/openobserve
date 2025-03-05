use std::sync::Arc;

use once_cell::sync::OnceCell;

mod conn;
mod error;
mod manager;
mod message_bus;
mod types;

pub use conn::{ClientRouterConnection, HealthMonitor, RouterQuerierConnection};
pub use error::{WsError, WsResult};
pub use manager::RouterManager;
pub use message_bus::RouterMessageBus;
pub use types::{Event, WsMessage};

static ROUTER_MANAGER: OnceCell<Arc<RouterManager>> = OnceCell::new();

pub async fn init() -> WsResult<()> {
    let message_bus = Arc::new(RouterMessageBus::new());
    let router_manager = Arc::new(RouterManager::new(message_bus));

    // Check if the ROUTER_MANAGER is already set
    if ROUTER_MANAGER.get().is_some() {
        return Ok(());
    }

    // Set the ROUTER_MANAGER if not already set
    ROUTER_MANAGER
        .set(router_manager)
        .map_err(|_| WsError::Connection("Failed to set ROUTER_MANAGER".into()))?;

    Ok(())
}

pub fn get_router_manager() -> Arc<RouterManager> {
    ROUTER_MANAGER
        .get()
        .expect("RouterManager not initialized")
        .clone()
}
