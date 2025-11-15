use std::collections::HashMap;
use std::time::Instant;

use reqwest::blocking::Client;
use reqwest::header::{CONTENT_TYPE, HeaderMap, HeaderName, HeaderValue};
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::localization::detect_locale;

/// Supported HTTP methods for API calls.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum ApiMethod {
    Get,
    Post,
    Put,
    Patch,
    Delete,
    Head,
}

impl From<ApiMethod> for reqwest::Method {
    fn from(method: ApiMethod) -> Self {
        match method {
            ApiMethod::Get => reqwest::Method::GET,
            ApiMethod::Post => reqwest::Method::POST,
            ApiMethod::Put => reqwest::Method::PUT,
            ApiMethod::Patch => reqwest::Method::PATCH,
            ApiMethod::Delete => reqwest::Method::DELETE,
            ApiMethod::Head => reqwest::Method::HEAD,
        }
    }
}

/// Authentication strategies supported by the builtin client.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ApiAuth {
    Bearer { token: String },
    Basic { username: String, password: String },
    ApiKey { header: String, value: String },
}

/// High-level API request description.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiRequest {
    pub method: ApiMethod,
    pub url: String,
    #[serde(default)]
    pub headers: HashMap<String, String>,
    #[serde(default)]
    pub query: HashMap<String, String>,
    pub body: Option<String>,
    pub timeout_ms: Option<u64>,
    pub auth: Option<ApiAuth>,
}

impl Default for ApiRequest {
    fn default() -> Self {
        Self {
            method: ApiMethod::Get,
            url: String::new(),
            headers: HashMap::new(),
            query: HashMap::new(),
            body: None,
            timeout_ms: Some(15_000),
            auth: None,
        }
    }
}

/// Result of an API call.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiResponse {
    pub status: u16,
    pub headers: HashMap<String, String>,
    pub body: String,
    pub elapsed_ms: u64,
}

#[derive(Debug, Error)]
pub enum ApiError {
    #[error("Ungültige URL: {0}")]
    InvalidUrl(String),
    #[error("Netzwerkfehler: {0}")]
    Network(String),
    #[error("Serialisierungsfehler: {0}")]
    Serialization(String),
}

impl ApiError {
    /// Returns a localized error string.
    pub fn to_localized_string(&self, locale: Option<&str>) -> String {
        let locale = detect_locale(locale);
        match (locale.language(), self) {
            ("de", ApiError::InvalidUrl(url)) => format!("Ungültige URL: {url}"),
            ("de", ApiError::Network(msg)) => format!("Netzwerkfehler: {msg}"),
            ("de", ApiError::Serialization(msg)) => format!("Serialisierungsfehler: {msg}"),
            (_, ApiError::InvalidUrl(url)) => format!("Invalid URL: {url}"),
            (_, ApiError::Network(msg)) => format!("Network error: {msg}"),
            (_, ApiError::Serialization(msg)) => format!("Serialization error: {msg}"),
        }
    }
}

impl From<reqwest::Error> for ApiError {
    fn from(value: reqwest::Error) -> Self {
        if value.is_builder() || value.url().is_none() {
            ApiError::InvalidUrl(value.to_string())
        } else {
            ApiError::Network(value.to_string())
        }
    }
}

impl From<serde_json::Error> for ApiError {
    fn from(value: serde_json::Error) -> Self {
        ApiError::Serialization(value.to_string())
    }
}

fn build_client(timeout_ms: Option<u64>) -> Result<Client, ApiError> {
    let mut builder = Client::builder();
    if let Some(timeout) = timeout_ms {
        builder = builder.timeout(std::time::Duration::from_millis(timeout));
    }
    builder.build().map_err(ApiError::from)
}

fn apply_headers(
    header_map: &mut HeaderMap,
    headers: &HashMap<String, String>,
) -> Result<(), ApiError> {
    for (name, value) in headers {
        let header_name = HeaderName::from_bytes(name.as_bytes())
            .map_err(|_| ApiError::InvalidUrl(format!("Invalid header name: {name}")))?;
        let header_value = HeaderValue::from_str(value)
            .map_err(|_| ApiError::InvalidUrl(format!("Invalid header value for {name}")))?;
        header_map.insert(header_name, header_value);
    }
    Ok(())
}

fn build_request(
    client: &Client,
    request: &ApiRequest,
) -> Result<reqwest::blocking::RequestBuilder, ApiError> {
    let method: reqwest::Method = request.method.into();
    let mut builder = client.request(method, &request.url);

    if !request.query.is_empty() {
        builder = builder.query(&request.query);
    }

    if let Some(body) = &request.body {
        builder = builder.body(body.clone());
    }

    if let Some(auth) = &request.auth {
        builder = match auth {
            ApiAuth::Bearer { token } => builder.bearer_auth(token),
            ApiAuth::Basic { username, password } => builder.basic_auth(username, Some(password)),
            ApiAuth::ApiKey { header, value } => builder.header(header, value),
        };
    }

    if !request.headers.is_empty() {
        let mut header_map = HeaderMap::new();
        apply_headers(&mut header_map, &request.headers)?;
        builder = builder.headers(header_map);
    }

    Ok(builder)
}

/// HTTP/JSON helper builtins.
pub struct ApiBuiltins;

impl ApiBuiltins {
    /// Executes a high-level API request and returns the captured response.
    pub fn send(request: ApiRequest) -> Result<ApiResponse, ApiError> {
        if request.url.is_empty() {
            return Err(ApiError::InvalidUrl("URL is empty".to_string()));
        }
        let client = build_client(request.timeout_ms)?;
        let builder = build_request(&client, &request)?;

        let started = Instant::now();
        let response = builder.send()?;
        let elapsed_ms = started.elapsed().as_millis() as u64;
        let status = response.status();
        let mut headers = HashMap::new();
        for (name, value) in response.headers() {
            if let Ok(value_str) = value.to_str() {
                headers.insert(name.to_string(), value_str.to_string());
            }
        }
        let body = response.text()?;

        Ok(ApiResponse {
            status: status.as_u16(),
            headers,
            body,
            elapsed_ms,
        })
    }

    /// Performs a GET request and parses the body as JSON.
    pub fn get_json(url: &str) -> Result<serde_json::Value, ApiError> {
        let mut request = ApiRequest {
            url: url.to_string(),
            ..ApiRequest::default()
        };
        request
            .headers
            .insert("Accept".to_string(), "application/json".to_string());
        let response = Self::send(request)?;
        Ok(serde_json::from_str(&response.body)?)
    }

    /// Performs a JSON POST request.
    pub fn post_json(
        url: &str,
        payload: &serde_json::Value,
    ) -> Result<serde_json::Value, ApiError> {
        let mut request = ApiRequest {
            method: ApiMethod::Post,
            url: url.to_string(),
            ..ApiRequest::default()
        };
        request
            .headers
            .insert("Accept".to_string(), "application/json".to_string());
        request.headers.insert(
            CONTENT_TYPE.as_str().to_string(),
            "application/json".to_string(),
        );
        request.body = Some(payload.to_string());
        let response = Self::send(request)?;
        Ok(serde_json::from_str(&response.body)?)
    }

    /// Simple health check helper returning whether a status code matches expectations.
    pub fn health_check(url: &str, expected_status: u16) -> Result<bool, ApiError> {
        let response = Self::send(ApiRequest {
            url: url.to_string(),
            ..ApiRequest::default()
        })?;
        Ok(response.status == expected_status)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::{Read, Write};
    use std::net::{TcpListener, TcpStream};
    use std::thread;

    fn start_test_server(response_body: &'static str) -> String {
        let listener = TcpListener::bind("127.0.0.1:0").expect("bind");
        let addr = listener.local_addr().unwrap();
        thread::spawn(move || {
            if let Ok((mut stream, _)) = listener.accept() {
                handle_connection(&mut stream, response_body);
            }
        });
        format!("http://{}", addr)
    }

    fn handle_connection(stream: &mut TcpStream, body: &str) {
        let mut buffer = [0u8; 512];
        let _ = stream.read(&mut buffer);
        let response = format!(
            "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\n\r\n{}",
            body.len(),
            body
        );
        let _ = stream.write_all(response.as_bytes());
        let _ = stream.flush();
    }

    #[test]
    fn get_json_parses_response() {
        let url = start_test_server("{\"ok\":true}");
        let value = ApiBuiltins::get_json(&url).unwrap();
        assert_eq!(value["ok"], serde_json::Value::Bool(true));
    }
}
