//! Discord HTTP client implementation.

use crate::error::{DiscordError, HttpError};
use crate::ratelimit::RateLimiter;
use crate::routes::{CurrentApplication, CurrentUser, GatewayBotResponse};

use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE, USER_AGENT};
use reqwest::{Client, Method, Response, StatusCode};
use serde::de::DeserializeOwned;
use simd_json::prelude::*;
use std::sync::Arc;
use tracing::{debug, warn};

/// Discord API base URL.
const API_BASE: &str = "https://discord.com/api/v10";

/// User agent for requests.
const USER_AGENT_VALUE: &str = concat!(
    "DiscordBot (https://github.com/Sh4dowNotFound/titanium-rs, ",
    env!("CARGO_PKG_VERSION"),
    ")"
);

/// Discord REST API client.
pub struct HttpClient {
    /// Inner HTTP client.
    client: Client,
    /// Bot token.
    token: String,
    /// Rate limiter.
    rate_limiter: Arc<RateLimiter>,
}

thread_local! {
    /// Per-thread scratch buffer for HTTP responses to avoid allocations.
    /// Default 32KB is enough for almost all Discord JSON responses.
    static RESPONSE_BUFFER: std::cell::RefCell<Vec<u8>> = std::cell::RefCell::new(Vec::with_capacity(32 * 1024));
}

impl HttpClient {
    /// Create a new HTTP client with the given bot token.
    pub fn new(token: impl Into<String>) -> Result<Self, HttpError> {
        let token = token.into();

        let mut headers = HeaderMap::new();
        headers.insert(
            AUTHORIZATION,
            HeaderValue::from_str(&format!("Bot {}", token))
                .map_err(|_| HttpError::Unauthorized)?,
        );
        headers.insert(USER_AGENT, HeaderValue::from_static(USER_AGENT_VALUE));
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

        let client = Client::builder().default_headers(headers).build()?;

        Ok(Self {
            client,
            token,
            rate_limiter: Arc::new(RateLimiter::new()),
        })
    }

    /// Get the bot token.
    pub fn token(&self) -> &str {
        &self.token
    }

    // =========================================================================
    // Gateway Endpoints
    // =========================================================================

    /// Get gateway bot information.
    ///
    /// Returns the recommended number of shards, gateway URL, and session limits.
    /// This is essential for large bots to determine sharding configuration.
    pub async fn get_gateway_bot(&self) -> Result<GatewayBotResponse, HttpError> {
        self.get("/gateway/bot").await
    }

    // =========================================================================
    // User Endpoints
    // =========================================================================

    /// Get the current bot user.
    pub async fn get_current_user(&self) -> Result<CurrentUser, HttpError> {
        self.get("/users/@me").await
    }

    /// Get the current application.
    pub async fn get_current_application(&self) -> Result<CurrentApplication, HttpError> {
        self.get("/applications/@me").await
    }

    // =========================================================================
    // Internal Request Methods
    // =========================================================================

    /// Make a GET request with query parameters.
    pub(crate) async fn get_with_query<T: DeserializeOwned, Q: serde::Serialize + ?Sized>(
        &self,
        route: &str,
        query: &Q,
    ) -> Result<T, HttpError> {
        self.request_with_query(Method::GET, route, query, None::<()>, None)
            .await
    }

    /// Make a GET request.
    pub(crate) async fn get<T: DeserializeOwned>(&self, route: &str) -> Result<T, HttpError> {
        self.request(Method::GET, route, None::<()>, None).await
    }

    /// Make a POST request.
    #[allow(dead_code)]
    pub(crate) async fn post<T: DeserializeOwned, B: serde::Serialize>(
        &self,
        route: &str,
        body: B,
    ) -> Result<T, HttpError> {
        self.request(Method::POST, route, Some(body), None).await
    }

    /// Make a PUT request.
    #[allow(dead_code)]
    pub(crate) async fn put<T: DeserializeOwned, B: serde::Serialize>(
        &self,
        route: &str,
        body: Option<B>,
    ) -> Result<T, HttpError> {
        self.request(Method::PUT, route, body, None).await
    }

    /// Make a PUT request with headers (for bans etc).
    pub(crate) async fn put_with_headers<T: DeserializeOwned, B: serde::Serialize>(
        &self,
        route: &str,
        body: Option<B>,
        headers: Option<HeaderMap>,
    ) -> Result<T, HttpError> {
        self.request(Method::PUT, route, body, headers).await
    }

    /// Make a PATCH request.
    #[allow(dead_code)]
    pub(crate) async fn patch<T: DeserializeOwned, B: serde::Serialize>(
        &self,
        route: &str,
        body: B,
    ) -> Result<T, HttpError> {
        self.request(Method::PATCH, route, Some(body), None).await
    }

    /// Make a DELETE request.
    #[allow(dead_code)]
    pub(crate) async fn delete<T: DeserializeOwned>(&self, route: &str) -> Result<T, HttpError> {
        self.request(Method::DELETE, route, None::<()>, None).await
    }

    /// Make a DELETE request with headers.
    pub(crate) async fn delete_with_headers<T: DeserializeOwned>(
        &self,
        route: &str,
        headers: Option<HeaderMap>,
    ) -> Result<T, HttpError> {
        self.request(Method::DELETE, route, None::<()>, headers)
            .await
    }

    /// Make a POST request with query parameters.
    #[allow(dead_code)]
    pub(crate) async fn post_with_query<
        T: DeserializeOwned,
        B: serde::Serialize,
        Q: serde::Serialize + ?Sized,
    >(
        &self,
        route: &str,
        body: B,
        query: &Q,
    ) -> Result<T, HttpError> {
        self.request_with_query(Method::POST, route, query, Some(body), None)
            .await
    }

    /// Make an HTTP request with rate limit handling.
    async fn request<T: DeserializeOwned, B: serde::Serialize>(
        &self,
        method: Method,
        route: &str,
        body: Option<B>,
        headers: Option<HeaderMap>,
    ) -> Result<T, HttpError> {
        self.request_with_query(method, route, &(), body, headers)
            .await
    }

    /// Make an HTTP request with query params, rate limit handling, and headers.
    async fn request_with_query<
        T: DeserializeOwned,
        Q: serde::Serialize + ?Sized,
        B: serde::Serialize,
    >(
        &self,
        method: Method,
        route: &str,
        query: &Q,
        body: Option<B>,
        headers: Option<HeaderMap>,
    ) -> Result<T, HttpError> {
        let url = format!("{}{}", API_BASE, route);

        // Acquire rate limit permit
        self.rate_limiter.acquire(route).await;

        // Build request
        let mut request = self.client.request(method.clone(), &url);

        // Add query params
        // reqwest::RequestBuilder::query handles generic Serialize.
        // Unit () serializes to empty/null which is ignored by reqwest for query params.
        request = request.query(query);

        if let Some(headers) = headers {
            request = request.headers(headers);
        }

        if let Some(ref body) = body {
            let body_bytes = simd_json::to_vec(body).map_err(|e| HttpError::Discord {
                code: 0,
                message: format!("Serialization error: {}", e),
            })?;
            request = request.body(body_bytes);
        }

        debug!(method = %method, route = %route, "Making request");

        // Send request
        let response = request.send().await?;

        // Handle response
        self.handle_response(route, response).await
    }

    /// Handle an HTTP response.
    async fn handle_response<T: DeserializeOwned>(
        &self,
        route: &str,
        response: Response,
    ) -> Result<T, HttpError> {
        let status = response.status();

        // Extract rate limit headers
        if let Some(remaining) = response
            .headers()
            .get("x-ratelimit-remaining")
            .and_then(|h| h.to_str().ok())
            .and_then(|s| s.parse().ok())
        {
            let reset_after = response
                .headers()
                .get("x-ratelimit-reset-after")
                .and_then(|h| h.to_str().ok())
                .and_then(|s| s.parse::<f64>().ok())
                .map(|f| (f * 1000.0) as u64)
                .unwrap_or(1000);

            self.rate_limiter.update(route, remaining, reset_after);
        }

        // Handle errors
        match status {
            StatusCode::OK | StatusCode::CREATED | StatusCode::NO_CONTENT => {
                let bytes = response.bytes().await?;
                if bytes.is_empty() {
                    // For NO_CONTENT or empty responses
                    // simd-json might choke on empty, but "null" is better?
                    // T might be () or Option<T>.
                    // Let's assume empty body means "null" or default.
                    // But if T is a struct, from_slice(b"null") might fail if not Option.
                    // Actually, existing code used b"null".to_vec();

                    RESPONSE_BUFFER.with(|buf_cell| {
                        let mut buf = buf_cell.borrow_mut();
                        buf.clear();
                        buf.extend_from_slice(b"null");
                        simd_json::from_slice(&mut buf).map_err(|e| HttpError::Discord {
                            code: 0,
                            message: e.to_string(),
                        })
                    })
                } else {
                    RESPONSE_BUFFER.with(|buf_cell| {
                        let mut buf = buf_cell.borrow_mut();
                        buf.clear();
                        buf.extend_from_slice(&bytes);
                        // simd-json parses in-place
                        simd_json::from_slice(&mut buf).map_err(|e| HttpError::Discord {
                            code: 0,
                            message: e.to_string(),
                        })
                    })
                }
            }
            StatusCode::TOO_MANY_REQUESTS => {
                let bytes = response.bytes().await?;
                let body: simd_json::OwnedValue = RESPONSE_BUFFER.with(|buf_cell| {
                    let mut buf = buf_cell.borrow_mut();
                    buf.clear();
                    buf.extend_from_slice(&bytes);
                    simd_json::from_slice(&mut buf).map_err(|e| HttpError::Discord {
                        code: 0,
                        message: e.to_string(),
                    })
                })?;

                let retry_after = body
                    .get("retry_after")
                    .and_then(|v| v.as_f64())
                    .map(|f| (f * 1000.0) as u64)
                    .unwrap_or(5000);

                let global = body
                    .get("global")
                    .and_then(|v| v.as_bool())
                    .unwrap_or(false);

                if global {
                    warn!(retry_after_ms = retry_after, "Global rate limit hit");
                    self.rate_limiter.set_global(retry_after);
                }

                Err(HttpError::RateLimited {
                    retry_after_ms: retry_after,
                    global,
                })
            }
            StatusCode::UNAUTHORIZED => Err(HttpError::Unauthorized),
            StatusCode::FORBIDDEN => Err(HttpError::Forbidden),
            StatusCode::NOT_FOUND => Err(HttpError::NotFound),
            _ if status.is_server_error() => Err(HttpError::ServerError(status.as_u16())),
            _ => {
                let bytes = response.bytes().await?;
                let error: DiscordError = RESPONSE_BUFFER.with(|buf_cell| {
                    let mut buf = buf_cell.borrow_mut();
                    buf.clear();
                    buf.extend_from_slice(&bytes);
                    simd_json::from_slice(&mut buf).map_err(|e| HttpError::Discord {
                        code: 0,
                        message: e.to_string(),
                    })
                })?;

                Err(HttpError::Discord {
                    code: error.code,
                    message: error.message,
                })
            }
        }
    }
    // =========================================================================
    // Interaction Endpoints
    // =========================================================================

    /// Create a global application command.
    pub async fn create_global_application_command(
        &self,
        application_id: titanium_model::Snowflake,
        command: &titanium_model::ApplicationCommand,
    ) -> Result<titanium_model::ApplicationCommand, HttpError> {
        let route = format!("/applications/{}/commands", application_id);
        self.post(&route, command).await
    }

    // =========================================================================
    // Channel Endpoints
    // =========================================================================

    /// Create a message in a channel.
    pub async fn create_message(
        &self,
        channel_id: titanium_model::Snowflake,
        content: &titanium_model::CreateMessage<'_>,
    ) -> Result<titanium_model::Message<'static>, HttpError> {
        let route = format!("/channels/{}/messages", channel_id);
        self.post(&route, content).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_client_creation() {
        let client = HttpClient::new("test_token");
        assert!(client.is_ok());
    }
}
