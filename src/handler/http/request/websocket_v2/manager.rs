use std::sync::Arc;

use anyhow::Result;
use async_trait::async_trait;

use super::event::EventExecutor;
use crate::common::meta::websocket_v2::{StreamConnection, StreamManager, WsMessage};

pub struct QuerierManager {
    event_executor: Arc<EventExecutor>,
}

impl QuerierManager {
    pub fn new(event_executor: Arc<EventExecutor>) -> Self {
        Self { event_executor }
    }
}

#[async_trait]
impl StreamManager for QuerierManager {
    async fn handle_connection(&self, mut connection: Box<dyn StreamConnection>) -> Result<()> {
        connection.connect().await?;
        connection.run_connection(None).await
    }

    async fn route_message(&self, message: WsMessage) -> Result<()> {
        // TODO: Implement route_message
        // match &message.event {
        //     Event::Search(req) => {
        //         let response = self.event_executor.execute_search(req).await?;
        //         self.message_bus
        //             .route_querier_message(
        //                 message.client_id,
        //                 Message::new(message.trace_id, Event::Response(response)),
        //             )
        //             .await
        //     }
        //     Event::Cancel(req) => {
        //         self.event_executor.execute_cancel(req).await?;
        //         Ok(())
        //     }
        //     Event::Benchmark(req) => {
        //         let response = self.event_executor.execute_benchmark(req).await?;
        //         self.message_bus
        //             .route_querier_message(
        //                 message.client_id,
        //                 Message::new(message.trace_id, Event::Response(response)),
        //             )
        //             .await
        //     }
        //     _ => {
        //         log::warn!("Unexpected message type received");
        //         Ok(())
        //     }
        // }
        todo!()
    }
}
