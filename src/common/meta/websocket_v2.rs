use std::sync::Arc;

use async_trait::async_trait;
use config::meta::{search::Response, websocket::SearchEventReq};
use serde::{Deserialize, Serialize};
use tokio::sync::mpsc::Sender;

pub type TraceId = String;

#[derive(Debug, Clone, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub struct ClientId(pub String);

impl ClientId {
    pub fn new(id: String) -> Self {
        Self(id)
    }
}

#[derive(Debug, Clone, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub struct QuerierId(pub String);

impl QuerierId {
    pub fn new(id: String) -> Self {
        Self(id)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchRequest {
    pub trace_id: TraceId,
    pub search_event_req: Box<SearchEventReq>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CancelRequest {
    pub trace_id: TraceId,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BenchmarkRequest {
    pub id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventResponse {
    pub trace_id: TraceId,
    pub response: Box<Response>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Event {
    Search(Box<SearchRequest>),
    Cancel(CancelRequest),
    Benchmark(BenchmarkRequest),
    Response(EventResponse),
    End,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WsMessage {
    pub trace_id: TraceId,
    pub event: Event,
}

#[async_trait]
pub trait StreamConnection: Send + Sync {
    async fn connect(&mut self) -> anyhow::Result<()>;
    async fn disconnect(&mut self) -> anyhow::Result<()>;
    async fn reconnect(&mut self) -> anyhow::Result<()>;
    async fn is_connected(&self) -> bool;
    async fn run_connection(
        &mut self,
        message_bus: Option<Arc<dyn MessageBus>>,
    ) -> anyhow::Result<()>;
}

#[async_trait]
pub trait StreamManager: Send + Sync {
    async fn handle_connection(&self, connection: Box<dyn StreamConnection>) -> anyhow::Result<()>;
    async fn route_message(&self, message: WsMessage) -> anyhow::Result<()>;
}

#[async_trait]
pub trait MessageBus: Send + Sync {
    async fn route_message(&self, message: WsMessage) -> anyhow::Result<()>;
    async fn register_client(
        &self,
        client_id: ClientId,
        tx: Sender<WsMessage>,
    ) -> anyhow::Result<()>;
    async fn register_querier(
        &self,
        querier_id: QuerierId,
        tx: Sender<WsMessage>,
    ) -> anyhow::Result<()>;
}
