use std::sync::Arc;

use actix_web::{HttpRequest, HttpResponse, web};
use async_trait::async_trait;
use dashmap::DashMap;

use super::ClientRouterConnection;
use crate::common::meta::websocket_v2::{
    ClientId, MessageBus, QuerierId, StreamConnection, StreamManager, WsMessage,
};

pub struct RouterManager {
    message_bus: Arc<dyn MessageBus>,
    querier_pool: Arc<DashMap<QuerierId, Arc<dyn StreamConnection>>>,
}

#[async_trait]
impl StreamManager for RouterManager {
    async fn handle_connection(
        &self,
        mut connection: Box<dyn StreamConnection>,
    ) -> anyhow::Result<()> {
        connection.connect().await?;
        connection
            .run_connection(Some(self.message_bus.clone()))
            .await
    }

    async fn route_message(&self, message: WsMessage) -> anyhow::Result<()> {
        self.message_bus.route_message(message).await
    }
}

impl RouterManager {
    pub fn new(message_bus: Arc<dyn MessageBus>) -> Self {
        Self {
            message_bus,
            querier_pool: Arc::new(DashMap::new()),
        }
    }

    pub async fn handle_client_connection(
        &self,
        req: HttpRequest,
        stream: web::Payload,
        client_id: ClientId,
    ) -> anyhow::Result<HttpResponse> {
        let (response, session, _) = actix_ws::handle(&req, stream)
            .map_err(|e| anyhow::anyhow!("WebSocket handshake error: {}", e))?;

        let client_conn = ClientRouterConnection::new(session, client_id);
        self.handle_connection(Box::new(client_conn)).await?;

        Ok(response)
    }
}
