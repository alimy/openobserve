use async_trait::async_trait;
use dashmap::DashMap;
use tokio::sync::mpsc::{self, Sender};

use super::error::{WsError, WsResult};
use crate::common::{
    infra::cluster,
    meta::websocket_v2::{ClientId, MessageBus, QuerierId, WsMessage},
};

pub struct RouterMessageBus {
    client_channels: DashMap<ClientId, mpsc::Sender<WsMessage>>,
    querier_channels: DashMap<QuerierId, mpsc::Sender<WsMessage>>,
}

impl RouterMessageBus {
    pub fn new() -> Self {
        Self {
            client_channels: DashMap::new(),
            querier_channels: DashMap::new(),
        }
    }

    // TODO: potentially where load balancing can be applied to select least busy querier
    async fn select_querier(&self, message: &WsMessage) -> Option<mpsc::Sender<WsMessage>> {
        // Use consistent hashing to select querier
        // CONFIRM: map all messages with the same trace_id to the same querier
        let node = cluster::get_node_from_consistent_hash(
            &message.trace_id,
            &config::meta::cluster::Role::Querier,
            None, // TODO: Add role group support if needed
        )
        .await?;

        self.querier_channels
            .get(&QuerierId::new(node))
            .map(|sender| sender.clone())
    }

    pub async fn route_client_message(
        &self,
        // TODO: Check if this is needed
        _client_id: ClientId,
        message: WsMessage,
    ) -> WsResult<()> {
        // Route based on message and consistent hashing
        let querier = self.select_querier(&message).await.ok_or_else(|| {
            WsError::Connection(format!(
                "No querier available for trace_id: {}",
                message.trace_id
            ))
        })?;

        querier
            .send(message)
            .await
            .map_err(|e| WsError::SendError(e.to_string()))?;

        Ok(())
    }

    pub async fn route_querier_message(
        &self,
        client_id: ClientId,
        message: WsMessage,
    ) -> WsResult<()> {
        if let Some(client) = self.client_channels.get(&client_id) {
            client
                .send(message)
                .await
                .map_err(|e| WsError::SendError(e.to_string()))?;
        }
        Ok(())
    }
}

#[async_trait]
impl MessageBus for RouterMessageBus {
    async fn route_message(&self, message: WsMessage) -> anyhow::Result<()> {
        // TODO: route message to client or querier on message type
        todo!()
    }

    async fn register_client(
        &self,
        client_id: ClientId,
        tx: Sender<WsMessage>,
    ) -> anyhow::Result<()> {
        self.client_channels.insert(client_id, tx);
        Ok(())
    }

    async fn register_querier(
        &self,
        querier_id: QuerierId,
        tx: Sender<WsMessage>,
    ) -> anyhow::Result<()> {
        self.querier_channels.insert(querier_id, tx);
        Ok(())
    }
}
