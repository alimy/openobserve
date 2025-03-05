use serde::{Deserialize, Serialize};

pub use crate::common::meta::websocket_v2::{Event, StreamConnection};
use crate::{
    common::meta::websocket_v2::{BenchmarkRequest, CancelRequest, ClientId, SearchRequest},
    handler::http::request::websocket::utils::{TimeOffset, WsClientEvents, WsServerEvents},
};

pub type SessionId = String;

// Message types for communication
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WsMessage {
    pub trace_id: String,
    pub client_id: ClientId,
    pub event: Event,
}

impl WsMessage {
    pub fn new(trace_id: &str, event: Event) -> Self {
        Self {
            trace_id: trace_id.to_string(),
            client_id: ClientId("default".to_string()),
            event,
        }
    }

    pub fn from_client_event(event: WsClientEvents) -> Self {
        match event {
            WsClientEvents::Search(req) => {
                let trace_id = req.trace_id.clone();
                let payload = serde_json::to_value(&req).unwrap_or_default();
                Self::new(
                    &trace_id,
                    Event::Search(Box::new(SearchRequest {
                        trace_id: trace_id.clone(),
                        search_event_req: req,
                    })),
                )
            }
            #[cfg(feature = "enterprise")]
            WsClientEvents::Cancel { trace_id } => Self::new(
                &trace_id,
                Event::Cancel(CancelRequest {
                    trace_id: trace_id.clone(),
                }),
            ),
            WsClientEvents::Benchmark { id } => {
                Self::new(&id, Event::Benchmark(BenchmarkRequest { id: id.clone() }))
            }
        }
    }

    pub fn to_server_event(&self) -> Option<WsServerEvents> {
        match &self.event {
            Event::Response(response) => Some(WsServerEvents::SearchResponse {
                trace_id: self.trace_id.clone(),
                results: response.response.clone(),
                time_offset: TimeOffset {
                    start_time: 0,
                    end_time: 0,
                },
                streaming_aggs: false,
            }),
            Event::End => Some(WsServerEvents::End {
                trace_id: Some(self.trace_id.clone()),
            }),
            _ => None,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct ErrorPayload {
    code: u16,
    message: String,
    error_detail: Option<String>,
}
