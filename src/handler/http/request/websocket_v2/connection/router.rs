use std::sync::Arc;

use anyhow::anyhow;
use async_trait::async_trait;
use futures_util::{SinkExt, StreamExt};
use tokio::{net::TcpStream, sync::Mutex};
use tokio_tungstenite::{MaybeTlsStream, WebSocketStream, connect_async, tungstenite::Message};

use crate::{
    common::meta::websocket_v2::{MessageBus, StreamConnection, WsMessage},
    handler::http::request::websocket_v2::EventExecutor,
};

pub struct RouterConnection {
    router_url: String,
    ws_stream: Arc<Mutex<Option<WebSocketStream<MaybeTlsStream<TcpStream>>>>>,
    event_executor: Arc<EventExecutor>,
}

impl RouterConnection {
    pub fn new(router_url: String, event_executor: Arc<EventExecutor>) -> Self {
        Self {
            router_url,
            ws_stream: Arc::new(Mutex::new(None)),
            event_executor,
        }
    }
}

#[async_trait]
impl StreamConnection for RouterConnection {
    async fn connect(&mut self) -> anyhow::Result<()> {
        let (ws_stream, _) = connect_async(&self.router_url).await?;
        let mut stream = self.ws_stream.lock().await;
        *stream = Some(ws_stream);
        Ok(())
    }

    async fn run_connection(
        &mut self,
        message_bus: Option<Arc<dyn MessageBus>>,
    ) -> anyhow::Result<()> {
        let mut ws_stream = self
            .ws_stream
            .lock()
            .await
            .take()
            .ok_or_else(|| anyhow!("No WebSocket stream"))?;

        loop {
            tokio::select! {
                Some(msg) = ws_stream.next() => {
                    match msg? {
                        Message::Text(text) => {
                            let message = serde_json::from_str::<WsMessage>(&text)?;
                            let response = self.handle_router_message(message).await?;

                            // Send response directly through WebSocket
                            let response_text = serde_json::to_string(&response)?;
                            ws_stream.send(Message::Text(response_text)).await?;
                        }
                        Message::Close(_) => break,
                        _ => {}
                    }
                }
            }
        }
        Ok(())
    }

    async fn is_connected(&self) -> bool {
        // TODO: Implement is_connected logic
        self.ws_stream.lock().await.is_some()
    }

    async fn disconnect(&mut self) -> anyhow::Result<()> {
        // TODO: Implement disconnect logic
        let mut stream = self.ws_stream.lock().await;
        *stream = None;
        Ok(())
    }

    async fn reconnect(&mut self) -> anyhow::Result<()> {
        // TODO: Implement reconnect logic
        todo!()
    }
}

impl RouterConnection {
    async fn handle_router_message(&self, _message: WsMessage) -> anyhow::Result<WsMessage> {
        // TODO: handle router message
        // match &message.event {
        //     Event::Search(req) => {
        //         let response = self.event_executor.execute_search(req).await?;
        //         Ok(Message::new(message.trace_id, Event::Response(response)))
        //     }
        // }
        todo!()
    }
}
