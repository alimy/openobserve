use std::sync::Arc;

use actix_ws::Session;
use anyhow::anyhow;
use async_trait::async_trait;
use futures_util::StreamExt;
use tokio::sync::mpsc;

use crate::common::meta::websocket_v2::{ClientId, MessageBus, StreamConnection, WsMessage};

pub struct ClientRouterConnection {
    session: Option<Session>,
    client_id: ClientId,
}

impl ClientRouterConnection {
    pub fn new(session: Session, client_id: ClientId) -> Self {
        Self {
            session: Some(session),
            client_id,
        }
    }
}

#[async_trait]
impl StreamConnection for ClientRouterConnection {
    async fn connect(&mut self) -> anyhow::Result<()> {
        // TODO: Implement connection logic
        Ok(())
    }

    async fn disconnect(&mut self) -> anyhow::Result<()> {
        if let Some(session) = self.session.take() {
            session
                .close(None)
                .await
                .map_err(|e| anyhow!("Failed to close session: {}", e))
        } else {
            log::warn!("No session to disconnect");
            Ok(())
        }
    }

    async fn is_connected(&self) -> bool {
        // TODO: Implement connection state check
        true
    }

    async fn run_connection(
        &mut self,
        message_bus: Option<Arc<dyn MessageBus>>,
    ) -> anyhow::Result<()> {
        let (tx, mut rx) = mpsc::channel(32);
        if let Some(message_bus) = message_bus {
            message_bus
                .register_client(self.client_id.clone(), tx)
                .await?;
        }

        loop {
            tokio::select! {
                Some(msg) = self.session.next() => {
                    match msg? {
                        actix_ws::Message::Text(text) => {
                            let message = serde_json::from_str::<WsMessage>(&text)?;
                            if let Some(message_bus) = &message_bus {
                                message_bus.route_message(message).await?;
                            }
                        }
                        actix_ws::Message::Close(_) => break,
                        _ => {}
                    }
                }
                Some(msg) = rx.recv() => {
                    let text = serde_json::to_string(&msg)?;
                    self.session.text(text).await?;
                }
                else => break,
            }
        }

        Ok(())
    }

    async fn reconnect(&mut self) -> anyhow::Result<()> {
        // TODO: Implement reconnection logic
        Ok(())
    }
}
