use anyhow::anyhow;
use async_trait::async_trait;

use super::super::executor::EventHandler;
use crate::common::meta::websocket_v2::{Event, EventResponse};

pub struct SearchHandler;

impl SearchHandler {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl EventHandler for SearchHandler {
    async fn handle(&self, event: Event) -> anyhow::Result<EventResponse> {
        match event {
            Event::Search(req) => {
                // Implement search logic
                todo!("Implement search handling")
            }
            _ => Err(anyhow!("Invalid event type for SearchHandler")),
        }
    }
}
