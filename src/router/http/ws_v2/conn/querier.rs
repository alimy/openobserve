use std::sync::Arc;

use anyhow::anyhow;
use async_trait::async_trait;
use futures_util::{SinkExt, StreamExt};
use tokio::{net::TcpStream, sync::mpsc};
use tokio_tungstenite::{MaybeTlsStream, WebSocketStream, connect_async, tungstenite};

use super::health::HealthMonitor;
use crate::common::meta::websocket_v2::{MessageBus, QuerierId, StreamConnection, WsMessage};

pub struct RouterQuerierConnection {
    querier_id: QuerierId,
    url: String,
    ws_stream: Option<WebSocketStream<MaybeTlsStream<TcpStream>>>,
    health_monitor: HealthMonitor,
}

impl RouterQuerierConnection {
    pub fn new(querier_id: QuerierId, url: String) -> Self {
        Self {
            querier_id,
            url,
            ws_stream: None,
            health_monitor: HealthMonitor::new(
                std::time::Duration::from_secs(5),
                std::time::Duration::from_secs(30),
            ),
        }
    }
}

#[async_trait]
impl StreamConnection for RouterQuerierConnection {
    async fn connect(&mut self) -> anyhow::Result<()> {
        let (ws_stream, _) = connect_async(&self.url).await?;
        self.ws_stream = Some(ws_stream);
        Ok(())
    }

    async fn disconnect(&mut self) -> anyhow::Result<()> {
        if let Some(mut ws_stream) = self.ws_stream.take() {
            ws_stream.close(None).await?;
        }
        Ok(())
    }

    async fn is_connected(&self) -> bool {
        self.ws_stream.is_some()
    }

    async fn run_connection(
        &mut self,
        message_bus: Option<Arc<dyn MessageBus>>,
    ) -> anyhow::Result<()> {
        let mut ws_stream = self
            .ws_stream
            .take()
            .ok_or_else(|| anyhow!("No WebSocket stream"))?;

        let (tx, mut rx) = mpsc::channel(32);
        if let Some(message_bus) = message_bus.clone() {
            message_bus
                .register_querier(self.querier_id.clone(), tx)
                .await?;
        }

        loop {
            tokio::select! {
                Some(msg) = ws_stream.next() => {
                    match msg? {
                        tungstenite::protocol::Message::Text(text) => {
                            let message = serde_json::from_str::<WsMessage>(&text)?;
                            if let Some(message_bus) = &message_bus {
                                message_bus.route_message(message).await?;
                            }
                        }
                        tungstenite::protocol::Message::Close(_) => break,
                        _ => {}
                    }
                }
                Some(msg) = rx.recv() => {
                    let text = serde_json::to_string(&msg)?;
                    ws_stream.send(tungstenite::protocol::Message::Text(text)).await?;
                }
            }
        }

        Ok(())
    }

    async fn reconnect(&mut self) -> anyhow::Result<()> {
        self.disconnect().await?;
        self.connect().await
    }
}
