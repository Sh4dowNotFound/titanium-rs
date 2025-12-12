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

    /// Errors from the Context (Interaction handling).
    #[error("Context error: {0}")]
    Context(#[from] ContextError),

    /// Generic errors from user code or other sources.
    #[error("Error: {0}")]
    Other(#[from] Box<dyn std::error::Error + Send + Sync>),
}

/// Errors occurring within Context operations.
#[derive(Debug, Error)]
pub enum ContextError {
    #[error("No interaction present in context")]
    NoInteraction,
    #[error("Already responded to interaction")]
    AlreadyResponded,
    #[error("HTTP error: {0}")]
    Http(#[from] titanium_http::error::HttpError),
}
