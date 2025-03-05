use anyhow::anyhow;
use async_trait::async_trait;

use super::super::executor::EventHandler;
use crate::common::meta::websocket_v2::{Event, EventResponse};

pub struct CancelHandler;

impl CancelHandler {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl EventHandler for CancelHandler {
    async fn handle(&self, event: Event) -> anyhow::Result<EventResponse> {
        match event {
            Event::Cancel(req) => {
                // Implement cancel logic
                todo!("Implement cancel handling")
            }
            _ => Err(anyhow!("Invalid event type for CancelHandler")),
        }
    }
}
