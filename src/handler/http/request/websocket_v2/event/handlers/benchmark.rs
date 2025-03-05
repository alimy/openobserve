use anyhow::anyhow;
use async_trait::async_trait;

use super::super::executor::EventHandler;
use crate::common::meta::websocket_v2::{Event, EventResponse};

pub struct BenchmarkHandler;

impl BenchmarkHandler {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl EventHandler for BenchmarkHandler {
    async fn handle(&self, event: Event) -> anyhow::Result<EventResponse> {
        match event {
            Event::Benchmark(req) => {
                // Implement benchmark logic
                todo!("Implement benchmark handling")
            }
            _ => Err(anyhow!("Invalid event type for BenchmarkHandler")),
        }
    }
}
