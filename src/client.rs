use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;

use reqwest::{Method, StatusCode};
use serde_json::{Value, json};
use tokio::time::sleep;

use crate::error::{ErrorCode, NbxError, NbxResult};
use crate::generated::endpoints::HttpMethod;

/// `NetBox` release that the pinned schema and generated code track.
///
/// Tied directly to the crate version because nbx releases mirror `NetBox`
/// releases — every `cargo install nbx@X.Y.Z` is built against the schema
/// from `netbox-community/netbox` tag `vX.Y.Z`. To target a new `NetBox`
/// release: update `Cargo.toml`'s `version`, drop the new schema into
/// `schema/`, and regenerate.
pub const TARGET_NETBOX_VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Debug, Clone)]
pub struct NetBoxClient {
    base_url: String,
    authentication: ClientAuthentication,
    client: reqwest::Client,
    version_checked: Arc<AtomicBool>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum ClientAuthentication {
    ApiToken(String),
    Unauthenticated,
}

impl ClientAuthentication {
    pub fn from_optional_token(token: Option<String>) -> Self {
        token.map_or(Self::Unauthenticated, Self::ApiToken)
    }

    fn authorization_header_value(&self) -> Option<String> {
        match self {
            Self::ApiToken(token) => Some(authorization_header_value(token)),
            Self::Unauthenticated => None,
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum VersionCheckMode {
    WarnOnMismatch,
    Skip,
}

impl NetBoxClient {
    pub fn new(base_url: String, token: String, timeout_seconds: u64) -> NbxResult<Self> {
        Self::new_with_authentication(
            base_url,
            ClientAuthentication::ApiToken(token),
            timeout_seconds,
        )
    }

    pub fn new_with_authentication(
        base_url: String,
        authentication: ClientAuthentication,
        timeout_seconds: u64,
    ) -> NbxResult<Self> {
        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(timeout_seconds))
            .build()
            .map_err(|error| NbxError::general(format!("failed to build HTTP client: {error}")))?;

        Ok(Self {
            base_url,
            authentication,
            client,
            version_checked: Arc::new(AtomicBool::new(false)),
        })
    }

    pub async fn request(
        &self,
        method: HttpMethod,
        api_path: &str,
        query_parameters: &[(String, String)],
        body: Option<Value>,
    ) -> NbxResult<Value> {
        self.request_with_version_check(
            method,
            api_path,
            query_parameters,
            body,
            VersionCheckMode::WarnOnMismatch,
        )
        .await
    }

    pub async fn request_without_version_check(
        &self,
        method: HttpMethod,
        api_path: &str,
        query_parameters: &[(String, String)],
        body: Option<Value>,
    ) -> NbxResult<Value> {
        self.request_with_version_check(
            method,
            api_path,
            query_parameters,
            body,
            VersionCheckMode::Skip,
        )
        .await
    }

    async fn request_with_version_check(
        &self,
        method: HttpMethod,
        api_path: &str,
        query_parameters: &[(String, String)],
        body: Option<Value>,
        version_check_mode: VersionCheckMode,
    ) -> NbxResult<Value> {
        if version_check_mode == VersionCheckMode::WarnOnMismatch {
            self.warn_on_version_mismatch_once().await;
        }

        let method = reqwest_method(method);
        let url = format!("{}{}", self.base_url, api_path);
        let mut attempt = 0_u8;

        loop {
            attempt += 1;
            let mut request = self
                .client
                .request(method.clone(), &url)
                .query(query_parameters);

            if let Some(authorization_header_value) =
                self.authentication.authorization_header_value()
            {
                request =
                    request.header(reqwest::header::AUTHORIZATION, authorization_header_value);
            }

            if let Some(body) = &body {
                request = request.json(body);
            }

            tracing::debug!(%method, %url, attempt, "sending NetBox request");

            let response = request
                .send()
                .await
                .map_err(|error| NbxError::general(format!("request failed: {error}")))?;
            let status = response.status();

            tracing::debug!(%status, %url, attempt, "received NetBox response");

            if status == StatusCode::TOO_MANY_REQUESTS && attempt < 3 {
                let delay = retry_delay(&response, attempt);
                sleep(delay).await;
                continue;
            }

            return parse_response(status, response).await;
        }
    }

    async fn warn_on_version_mismatch_once(&self) {
        if std::env::var_os("NBX_SKIP_VERSION_CHECK").is_some() {
            return;
        }
        if self.version_checked.swap(true, Ordering::SeqCst) {
            return;
        }

        let url = format!("{}/api/status/", self.base_url);
        let mut request = self.client.get(&url);
        if let Some(authorization_header_value) = self.authentication.authorization_header_value() {
            request = request.header(reqwest::header::AUTHORIZATION, authorization_header_value);
        }

        let Ok(response) = request.send().await else {
            return;
        };
        if !response.status().is_success() {
            return;
        }
        let Ok(payload) = response.json::<Value>().await else {
            return;
        };
        let Some(running_version) = payload.get("netbox-version").and_then(Value::as_str) else {
            return;
        };

        if !same_major_minor(running_version, TARGET_NETBOX_VERSION) {
            eprintln!(
                "warning: nbx is built against NetBox {TARGET_NETBOX_VERSION}, but the server reports {running_version}; behavior is best-effort outside the pinned major.minor (set NBX_SKIP_VERSION_CHECK=1 to silence)"
            );
        }
    }
}

fn same_major_minor(a: &str, b: &str) -> bool {
    major_minor(a) == major_minor(b)
}

fn major_minor(version: &str) -> Option<(u32, u32)> {
    let mut parts = version.trim_start_matches('v').split('.');
    let major = parts.next()?.parse::<u32>().ok()?;
    let minor = parts.next()?.parse::<u32>().ok()?;
    Some((major, minor))
}

fn authorization_header_value(token: &str) -> String {
    if token.starts_with("nbt_") && token.contains('.') {
        format!("Bearer {token}")
    } else {
        format!("Token {token}")
    }
}

fn reqwest_method(method: HttpMethod) -> Method {
    match method {
        HttpMethod::Get => Method::GET,
        HttpMethod::Post => Method::POST,
        HttpMethod::Patch => Method::PATCH,
        HttpMethod::Delete => Method::DELETE,
    }
}

async fn parse_response(status: StatusCode, response: reqwest::Response) -> NbxResult<Value> {
    let response_text = response
        .text()
        .await
        .map_err(|error| NbxError::general(format!("failed to read response body: {error}")))?;
    let response_value = if response_text.trim().is_empty() {
        Value::Null
    } else {
        serde_json::from_str(&response_text).unwrap_or_else(|_| json!({ "body": response_text }))
    };

    if status.is_success() {
        return Ok(response_value);
    }

    let code = match status {
        StatusCode::NOT_FOUND => ErrorCode::NotFound,
        StatusCode::UNAUTHORIZED | StatusCode::FORBIDDEN => ErrorCode::AuthFailed,
        StatusCode::BAD_REQUEST | StatusCode::UNPROCESSABLE_ENTITY => ErrorCode::ValidationError,
        _ => ErrorCode::General,
    };

    let message = match code {
        ErrorCode::NotFound => "NetBox resource not found",
        ErrorCode::AuthFailed => "NetBox authentication failed",
        ErrorCode::ValidationError => "NetBox validation failed",
        ErrorCode::General | ErrorCode::StreamError => "NetBox request failed",
    };

    Err(NbxError::new(
        code,
        message,
        json!({
            "status": status.as_u16(),
            "response": response_value,
        }),
    ))
}

fn retry_delay(response: &reqwest::Response, attempt: u8) -> Duration {
    response
        .headers()
        .get(reqwest::header::RETRY_AFTER)
        .and_then(|value| value.to_str().ok())
        .and_then(|value| value.parse::<u64>().ok())
        .map_or_else(
            || Duration::from_secs(2_u64.pow(u32::from(attempt - 1))),
            Duration::from_secs,
        )
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use wiremock::matchers::{header, method, path, query_param};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    /// Disable the live `/api/status/` probe inside this test module so wiremock
    /// expectations stay scoped to the routes each test sets up.
    fn disable_runtime_version_probe() {
        // SAFETY: Tests in this module run sequentially on a single thread because
        // each spawns its own #[tokio::test] runtime; setting a process-wide env
        // var here is safe for the duration of the binary.
        unsafe {
            std::env::set_var("NBX_SKIP_VERSION_CHECK", "1");
        }
    }

    #[test]
    fn parses_major_minor_for_pinned_target() {
        assert_eq!(major_minor(TARGET_NETBOX_VERSION), Some((4, 5)));
        assert_eq!(major_minor("v4.6.0"), Some((4, 6)));
        assert!(same_major_minor("4.5.7", TARGET_NETBOX_VERSION));
        assert!(!same_major_minor("4.6.0", TARGET_NETBOX_VERSION));
    }

    #[test]
    fn uses_bearer_auth_for_netbox_v2_tokens() {
        assert_eq!(
            authorization_header_value("nbt_abc123.secret"),
            "Bearer nbt_abc123.secret",
        );
    }

    #[test]
    fn uses_token_auth_for_legacy_tokens() {
        assert_eq!(
            authorization_header_value("0123456789abcdef0123456789abcdef01234567"),
            "Token 0123456789abcdef0123456789abcdef01234567",
        );
    }

    fn build_test_client(server: &MockServer) -> NetBoxClient {
        disable_runtime_version_probe();
        NetBoxClient::new(server.uri(), "legacy-token".into(), 5)
            .expect("client should build with mock server uri")
    }

    #[tokio::test]
    async fn sends_authorization_header_and_returns_parsed_body() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/dcim/devices/"))
            .and(header("authorization", "Token legacy-token"))
            .and(query_param("site", "dc1"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({ "count": 0 })))
            .expect(1)
            .mount(&server)
            .await;

        let client = build_test_client(&server);
        let response = client
            .request(
                HttpMethod::Get,
                "/api/dcim/devices/",
                &[("site".into(), "dc1".into())],
                None,
            )
            .await
            .expect("request should succeed");

        assert_eq!(response, json!({ "count": 0 }));
    }

    #[tokio::test]
    async fn maps_404_to_not_found_with_response_detail() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/dcim/devices/missing/"))
            .respond_with(
                ResponseTemplate::new(404).set_body_json(json!({ "detail": "Not found." })),
            )
            .mount(&server)
            .await;

        let client = build_test_client(&server);
        let error = client
            .request(HttpMethod::Get, "/api/dcim/devices/missing/", &[], None)
            .await
            .expect_err("404 should map to NbxError");

        assert_eq!(error.code, ErrorCode::NotFound);
        assert_eq!(error.detail["status"], json!(404));
        assert_eq!(error.detail["response"]["detail"], json!("Not found."));
    }

    #[tokio::test]
    async fn maps_401_to_auth_failed() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/dcim/devices/"))
            .respond_with(ResponseTemplate::new(401))
            .mount(&server)
            .await;

        let client = build_test_client(&server);
        let error = client
            .request(HttpMethod::Get, "/api/dcim/devices/", &[], None)
            .await
            .expect_err("401 should map to NbxError");

        assert_eq!(error.code, ErrorCode::AuthFailed);
    }

    #[tokio::test]
    async fn maps_422_to_validation_error() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/api/dcim/devices/"))
            .respond_with(
                ResponseTemplate::new(422)
                    .set_body_json(json!({ "name": ["This field is required."] })),
            )
            .mount(&server)
            .await;

        let client = build_test_client(&server);
        let error = client
            .request(HttpMethod::Post, "/api/dcim/devices/", &[], Some(json!({})))
            .await
            .expect_err("422 should map to NbxError");

        assert_eq!(error.code, ErrorCode::ValidationError);
        assert_eq!(
            error.detail["response"]["name"],
            json!(["This field is required."])
        );
    }

    #[tokio::test]
    async fn maps_500_to_general_error() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/dcim/devices/"))
            .respond_with(ResponseTemplate::new(503))
            .mount(&server)
            .await;

        let client = build_test_client(&server);
        let error = client
            .request(HttpMethod::Get, "/api/dcim/devices/", &[], None)
            .await
            .expect_err("503 should map to NbxError");

        assert_eq!(error.code, ErrorCode::General);
    }

    #[tokio::test]
    async fn retries_on_429_then_succeeds() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/dcim/devices/"))
            .respond_with(
                ResponseTemplate::new(429)
                    .insert_header("retry-after", "0")
                    .set_body_string(""),
            )
            .up_to_n_times(1)
            .expect(1)
            .mount(&server)
            .await;
        Mock::given(method("GET"))
            .and(path("/api/dcim/devices/"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({ "count": 1 })))
            .expect(1)
            .mount(&server)
            .await;

        let client = build_test_client(&server);
        let response = client
            .request(HttpMethod::Get, "/api/dcim/devices/", &[], None)
            .await
            .expect("retry should ultimately succeed");

        assert_eq!(response, json!({ "count": 1 }));
    }

    #[tokio::test]
    async fn stops_retrying_after_three_429s_and_returns_last_response() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/dcim/devices/"))
            .respond_with(
                ResponseTemplate::new(429)
                    .insert_header("retry-after", "0")
                    .set_body_string(""),
            )
            .expect(3)
            .mount(&server)
            .await;

        let client = build_test_client(&server);
        let error = client
            .request(HttpMethod::Get, "/api/dcim/devices/", &[], None)
            .await
            .expect_err("third 429 should bubble up as an error");

        assert_eq!(error.code, ErrorCode::General);
        assert_eq!(error.detail["status"], json!(429));
    }

    #[tokio::test]
    async fn sends_json_body_on_post() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/api/dcim/devices/"))
            .and(wiremock::matchers::body_json(
                json!({ "name": "srv-db-07" }),
            ))
            .respond_with(ResponseTemplate::new(201).set_body_json(json!({ "id": 42 })))
            .expect(1)
            .mount(&server)
            .await;

        let client = build_test_client(&server);
        let response = client
            .request(
                HttpMethod::Post,
                "/api/dcim/devices/",
                &[],
                Some(json!({ "name": "srv-db-07" })),
            )
            .await
            .expect("create should succeed");

        assert_eq!(response, json!({ "id": 42 }));
    }

    #[tokio::test]
    async fn empty_response_body_is_treated_as_null() {
        let server = MockServer::start().await;
        Mock::given(method("DELETE"))
            .and(path("/api/dcim/devices/42/"))
            .respond_with(ResponseTemplate::new(204).set_body_string(""))
            .mount(&server)
            .await;

        let client = build_test_client(&server);
        let response = client
            .request(HttpMethod::Delete, "/api/dcim/devices/42/", &[], None)
            .await
            .expect("204 with empty body should succeed");

        assert!(response.is_null());
    }
}
