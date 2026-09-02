use crate::models;
use serde::{Deserialize, Serialize};
use std::error;
use std::fmt::Display;

/// Errors that can occur when using the TidalRS library.
///
/// This enum covers all possible error conditions including network issues,
/// API errors, authentication problems, and streaming issues.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// HTTP request failed (network issues, timeouts, etc.)
    #[error("HTTP request failed: {0}")]
    Http(#[from] reqwest::Error),

    /// JSON serialization/deserialization failed
    #[error("JSON serialization/deserialization failed: {0}")]
    SerdeJson(#[from] serde_json::Error),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    /// Tidal API returned an error response
    #[error("Tidal Error: {0}")]
    TidalError(TidalError),

    /// No authorization token available for refresh
    #[error("No authz token available to refresh client authorization")]
    NoAuthzToken,

    /// No primary streaming URL available for the track
    #[error("No primary streaming URL available")]
    NoPrimaryUrl,

    /// Failed to initialize audio stream
    #[error("Stream initialization error: {0}")]
    StreamInitializationError(String),

    /// No access token available - client needs authentication
    #[error("No access token available - have you authorized the client?")]
    NoAccessTokenAvailable,

    /// User authentication required for this operation
    #[error("User authentication required - please login first")]
    UserAuthenticationRequired,

    /// Exponential backoff exceeded the maximum duration while handling rate limits
    #[error("Hit rate limit backoff ceiling of {0}ms without recovery")]
    RateLimitBackoffExceeded(u64),

    /// Client secret is required for this authorization flow
    #[error("Client secret is required for this authorization flow")]
    ClientSecretRequired,

    /// PKCE login has not been started; call `start_pkce` first
    #[error("PKCE login has not been started; call start_pkce first")]
    PkceNotStarted,

    /// Authorization code was not found in the PKCE redirect URL
    #[error("Authorization code not found in PKCE redirect URL")]
    PkceRedirectMissingCode,

    /// Device authorization timed out before the user completed login
    #[error("Device authorization timed out before the user completed login")]
    DeviceAuthorizationTimeout,

    /// OAuth device authorization was rejected or failed
    #[error("OAuth device authorization failed: {0}")]
    DeviceAuthorizationDenied(String),

    /// The OAuth token endpoint returned an error response
    #[error("OAuth error {error}: {description}")]
    OAuth { error: String, description: String },

    /// URL parsing failed
    #[error("Invalid URL: {0}")]
    Url(#[from] url::ParseError),
}

/// An error body returned by a TIDAL API.
///
/// Deserialization is shape-based rather than untagged: a JSON:API `errors`
/// array is a v2 error, a `subStatus` / `sub_status` field is a v1 error, and
/// `{"status": ..., "message": ...}` is the serialized
/// [`TidalError::UnknownError`] shape. Other unrecognized API bodies fail so
/// the caller can wrap the HTTP status and raw response.
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum TidalError {
    TidalV2Error(TidalV2Error),
    TidalV1Error(TidalV1Error),
    /// Built by callers that know the HTTP status and raw response body.
    UnknownError(TidalUnknownError),
}

impl<'de> Deserialize<'de> for TidalError {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = serde_json::Value::deserialize(deserializer)?;
        if value.get("errors").is_some_and(|e| e.is_array()) {
            return TidalV2Error::deserialize(value)
                .map(TidalError::TidalV2Error)
                .map_err(serde::de::Error::custom);
        }
        if value
            .get("sub_status")
            .or_else(|| value.get("subStatus"))
            .is_some()
        {
            return TidalV1Error::deserialize(value)
                .map(TidalError::TidalV1Error)
                .map_err(serde::de::Error::custom);
        }
        if value.get("status").is_some() && value.get("message").is_some() {
            return TidalUnknownError::deserialize(value)
                .map(TidalError::UnknownError)
                .map_err(serde::de::Error::custom);
        }
        Err(serde::de::Error::custom("unrecognized TIDAL error shape"))
    }
}

impl TidalError {
    /// Whether this error indicates the access token expired and should be refreshed.
    pub fn is_expired_token(&self) -> bool {
        match self {
            TidalError::TidalV1Error(err) => err.sub_status == 11003,
            TidalError::TidalV2Error(err) => err.errors.as_ref().is_some_and(|errors| {
                errors.iter().any(|e| {
                    e.code.as_deref() == Some("UNAUTHORIZED")
                        && e.detail.as_deref() == Some("Expired token")
                })
            }),
            TidalError::UnknownError(_) => false,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TidalUnknownError {
    pub status: u16,
    pub message: String,
}

impl Display for TidalUnknownError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Status: {}, Message: {}", self.status, self.message)
    }
}

impl Display for TidalError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TidalError::TidalV1Error(err) => write!(f, "{}", err),
            TidalError::TidalV2Error(err) => write!(f, "{}", err),
            TidalError::UnknownError(err) => write!(f, "{}", err),
        }
    }
}

/// Error response from the Tidal API.
///
/// This represents errors returned by Tidal's API endpoints and includes
/// both HTTP status codes and Tidal-specific error information.
#[derive(Debug, Serialize, Clone)]
pub struct TidalV1Error {
    /// HTTP status code, when the response includes one.
    pub status: Option<u16>,
    /// Tidal-specific sub-status code
    pub sub_status: u64,
    /// Human-readable error message
    pub user_message: String,
}

impl<'de> Deserialize<'de> for TidalV1Error {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value: serde_json::Value = serde_json::Value::deserialize(deserializer)?;

        let status = value
            .get("status")
            .and_then(|v| v.as_u64())
            .map(|v| v as u16);

        let sub_status = value
            .get("sub_status")
            .or_else(|| value.get("subStatus"))
            .and_then(|v| v.as_u64())
            .ok_or_else(|| {
                serde::de::Error::custom("Missing or invalid 'sub_status'/'subStatus' field")
            })?;

        let user_message = value
            .get("user_message")
            .or_else(|| value.get("userMessage"))
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();

        Ok(TidalV1Error {
            status,
            sub_status,
            user_message,
        })
    }
}

impl Display for TidalV1Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.status {
            Some(status) => write!(
                f,
                "Tidal API error: {} {} {}",
                status, self.sub_status, self.user_message
            ),
            None => write!(
                f,
                "Tidal API error: {} {}",
                self.sub_status, self.user_message
            ),
        }
    }
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct TidalV2Error {
    /// Array of error objects
    #[serde(rename = "errors", skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<ErrorObject>>,
    #[serde(rename = "links", skip_serializing_if = "Option::is_none")]
    pub links: Option<models::Links>,
}

impl TidalV2Error {
    pub fn new() -> TidalV2Error {
        TidalV2Error {
            errors: None,
            links: None,
        }
    }
}

impl Display for TidalV2Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match serde_json::to_string_pretty(self) {
            Ok(json) => write!(f, "Tidal V2 API error: {}", json),
            Err(_) => write!(f, "Tidal V2 API error: failed to serialize error details"),
        }
    }
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ErrorObject {
    /// application-specific error code
    #[serde(rename = "code", skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// human-readable explanation specific to this occurrence of the problem
    #[serde(rename = "detail", skip_serializing_if = "Option::is_none")]
    pub detail: Option<String>,
    /// unique identifier for this particular occurrence of the problem
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "source", skip_serializing_if = "Option::is_none")]
    pub source: Option<ErrorObjectSource>,
    /// HTTP status code applicable to this problem
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

impl ErrorObject {
    pub fn new() -> ErrorObject {
        ErrorObject {
            code: None,
            detail: None,
            id: None,
            source: None,
            status: None,
        }
    }
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ErrorObjectSource {
    /// string indicating the name of a single request header which caused the error
    #[serde(rename = "header", skip_serializing_if = "Option::is_none")]
    pub header: Option<String>,
    /// string indicating which URI query parameter caused the error.
    #[serde(rename = "parameter", skip_serializing_if = "Option::is_none")]
    pub parameter: Option<String>,
    /// a JSON Pointer (RFC6901) to the value in the request document that caused the error
    #[serde(rename = "pointer", skip_serializing_if = "Option::is_none")]
    pub pointer: Option<String>,
}

impl ErrorObjectSource {
    pub fn new() -> ErrorObjectSource {
        ErrorObjectSource {
            header: None,
            parameter: None,
            pointer: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse(json: &str) -> Result<TidalError, serde_json::Error> {
        serde_json::from_str(json)
    }

    #[test]
    fn v2_shape_parses_with_errors() {
        let err = parse(r#"{"errors":[{"code":"UNAUTHORIZED","detail":"Expired token"}]}"#)
            .expect("v2 shape");
        match err {
            TidalError::TidalV2Error(v2) => {
                assert_eq!(v2.errors.expect("errors").len(), 1);
            }
            other => panic!("expected TidalV2Error, got {other:?}"),
        }
    }

    #[test]
    fn v1_camel_case_parses_with_status() {
        let err =
            parse(r#"{"status":401,"subStatus":11003,"userMessage":"The token has expired"}"#)
                .expect("v1 camelCase");
        match err {
            TidalError::TidalV1Error(v1) => {
                assert_eq!(v1.status, Some(401));
                assert_eq!(v1.sub_status, 11003);
                assert_eq!(v1.user_message, "The token has expired");
            }
            other => panic!("expected TidalV1Error, got {other:?}"),
        }
    }

    #[test]
    fn v1_snake_case_without_status_parses() {
        let err = parse(r#"{"sub_status":11003,"user_message":"The token has expired"}"#)
            .expect("v1 snake_case without status");
        match err {
            TidalError::TidalV1Error(v1) => {
                assert_eq!(v1.status, None);
                assert_eq!(v1.sub_status, 11003);
                assert_eq!(v1.user_message, "The token has expired");
            }
            other => panic!("expected TidalV1Error, got {other:?}"),
        }
    }

    #[test]
    fn oauth_shape_is_unrecognized() {
        parse(r#"{"error":"invalid_client","error_description":"Bad client secret"}"#)
            .expect_err("OAuth body is not a TIDAL error shape");
    }

    #[test]
    fn empty_object_is_unrecognized() {
        parse("{}").expect_err("empty object is not a TIDAL error shape");
    }

    #[test]
    fn unknown_shape_parses_status_and_message() {
        let err = parse(r#"{"status":502,"message":"upstream exploded"}"#).expect("unknown shape");
        match err {
            TidalError::UnknownError(unknown) => {
                assert_eq!(unknown.status, 502);
                assert_eq!(unknown.message, "upstream exploded");
            }
            other => panic!("expected UnknownError, got {other:?}"),
        }
    }

    #[test]
    fn unknown_error_round_trips() {
        let original = TidalError::UnknownError(TidalUnknownError {
            status: 503,
            message: "gateway timeout".into(),
        });
        let json = serde_json::to_string(&original).expect("serialize");
        let parsed: TidalError = serde_json::from_str(&json).expect("deserialize");
        match parsed {
            TidalError::UnknownError(unknown) => {
                assert_eq!(unknown.status, 503);
                assert_eq!(unknown.message, "gateway timeout");
            }
            other => panic!("expected UnknownError, got {other:?}"),
        }
    }

    #[test]
    fn malformed_unknown_shape_is_rejected() {
        parse(r#"{"status":"502","message":"upstream exploded"}"#)
            .expect_err("status must be a number");
        parse(r#"{"status":502,"message":123}"#).expect_err("message must be a string");
        parse(r#"{"status":502}"#).expect_err("message is required");
        parse(r#"{"message":"upstream exploded"}"#).expect_err("status is required");
    }

    #[test]
    fn v1_discriminator_takes_precedence_over_unknown_shape() {
        let err = parse(
            r#"{"status":401,"subStatus":11003,"message":"synthetic","userMessage":"expired"}"#,
        )
        .expect("v1 with message");
        match err {
            TidalError::TidalV1Error(v1) => {
                assert_eq!(v1.status, Some(401));
                assert_eq!(v1.sub_status, 11003);
                assert_eq!(v1.user_message, "expired");
            }
            other => panic!("expected TidalV1Error, got {other:?}"),
        }
    }

    #[test]
    fn is_expired_token_v1_11003() {
        let err = parse(r#"{"status":401,"subStatus":11003,"userMessage":"expired"}"#).unwrap();
        assert!(err.is_expired_token());
    }

    #[test]
    fn is_expired_token_v2_unauthorized() {
        let err =
            parse(r#"{"errors":[{"code":"UNAUTHORIZED","detail":"Expired token"}]}"#).unwrap();
        assert!(err.is_expired_token());
    }

    #[test]
    fn is_expired_token_false_for_other_v1() {
        let err = parse(r#"{"status":400,"subStatus":1002,"userMessage":"pending"}"#).unwrap();
        assert!(!err.is_expired_token());
    }

    #[test]
    fn is_expired_token_false_for_other_v2() {
        let err = parse(r#"{"errors":[{"code":"NOT_FOUND","detail":"missing"}]}"#).unwrap();
        assert!(!err.is_expired_token());
    }

    #[test]
    fn v1_display_omits_missing_status() {
        let err = TidalV1Error {
            status: None,
            sub_status: 11003,
            user_message: "expired".into(),
        };
        assert_eq!(err.to_string(), "Tidal API error: 11003 expired");
    }
}
