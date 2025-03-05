use std::sync::Arc;

use anyhow::anyhow;
use async_trait::async_trait;
use dashmap::DashMap;
use tokio::sync::broadcast;

use super::handlers::{BenchmarkHandler, CancelHandler, SearchHandler};
use crate::common::meta::websocket_v2::{
    BenchmarkRequest, CancelRequest, Event, EventResponse, SearchRequest,
};

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub enum EventType {
    Search,
    Cancel,
    Benchmark,
}

#[derive(Clone)]
pub struct CancellationToken {
    _cancel: broadcast::Sender<()>,
}

impl CancellationToken {
    pub fn new() -> Self {
        let (tx, _) = broadcast::channel(1);
        Self { _cancel: tx }
    }

    pub fn cancel(&self) {
        let _ = self._cancel.send(());
    }
}

#[async_trait]
pub trait EventHandler: Send + Sync {
    async fn handle(&self, event: Event) -> anyhow::Result<EventResponse>;
}

pub struct EventExecutor {
    handlers: DashMap<EventType, Arc<dyn EventHandler>>,
    active_searches: DashMap<String, CancellationToken>,
}

impl EventExecutor {
    pub fn new() -> Self {
        let mut executor = Self {
            handlers: DashMap::new(),
            active_searches: DashMap::new(),
        };

        // Register default handlers
        executor.register_handler(EventType::Search, Arc::new(SearchHandler::new()));
        executor.register_handler(EventType::Cancel, Arc::new(CancelHandler::new()));
        executor.register_handler(EventType::Benchmark, Arc::new(BenchmarkHandler::new()));

        executor
    }

    pub fn register_handler(&self, event_type: EventType, handler: Arc<dyn EventHandler>) {
        self.handlers.insert(event_type, handler);
    }

    pub async fn execute_search(&self, req: &SearchRequest) -> anyhow::Result<EventResponse> {
        let token = CancellationToken::new();
        self.active_searches
            .insert(req.trace_id.clone(), token.clone());

        let handler = self
            .handlers
            .get(&EventType::Search)
            .ok_or_else(|| anyhow!("No handler registered for Search"))?;

        let result = handler.handle(Event::Search(Box::new(req.clone()))).await;
        self.active_searches.remove(&req.trace_id);

        result
    }

    pub async fn execute_cancel(&self, req: &CancelRequest) -> anyhow::Result<()> {
        if let Some(token) = self.active_searches.get(&req.trace_id) {
            token.cancel();
        }
        Ok(())
    }

    pub async fn execute_benchmark(&self, req: &BenchmarkRequest) -> anyhow::Result<EventResponse> {
        let handler = self
            .handlers
            .get(&EventType::Benchmark)
            .ok_or_else(|| anyhow!("No handler registered for Benchmark"))?;

        handler.handle(Event::Benchmark(req.clone())).await
    }
}
