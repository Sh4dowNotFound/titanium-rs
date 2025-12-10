use thiserror::Error;

/// Unified error type for the Titanium framework.
#[derive(Debug, Error)]
pub enum TitaniumError {
    /// Errors from the Gateway (WebSocket, Sharding).
    #[error("Gateway error: {0}")]
    Gateway(#[from] titanium_gateway::error::GatewayError),

    /// Errors from the HTTP client (REST API).
    #[error("HTTP error: {0}")]
    Http(#[from] titanium_http::error::HttpError),

    /// Errors from the Framework (Command handling).
    #[error("Framework error: {0}")]
    Framework(String),

    /// Generic errors from user code or other sources.
    #[error("Error: {0}")]
    Other(#[from] Box<dyn std::error::Error + Send + Sync>),
}
