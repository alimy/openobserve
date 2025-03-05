use thiserror::Error;

#[derive(Error, Debug)]
pub enum WsError {
    #[error("Connection error: {0}")]
    Connection(String),
    #[error("Message error: {0}")]
    Message(String),
    #[error("Send error: {0}")]
    SendError(String),
    #[error("Receive error: {0}")]
    ReceiveError(String),
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("WebSocket error: {0}")]
    WebSocket(#[from] actix_ws::ProtocolError),
    #[error("Actix error: {0}")]
    Actix(#[from] actix_web::Error),
    #[error("Other error: {0}")]
    Other(#[from] anyhow::Error),
}

pub type WsResult<T> = Result<T, WsError>;

impl From<WsError> for actix_web::Error {
    fn from(err: WsError) -> Self {
        actix_web::error::ErrorInternalServerError(err.to_string())
    }
}
