use std::error;
use serde::{Deserialize, Serialize};
use crate::models;
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
}


#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum TidalError {
    TidalV2Error(TidalV2Error),
    TidalV1Error(TidalV1Error),
    UnknownError(TidalUnknownError),
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
    /// HTTP status code
    pub status: u16,
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
        // First deserialize to a generic Value
        let value: serde_json::Value = serde_json::Value::deserialize(deserializer)?;
        
        // Extract status (should be consistent)
        // TODO: Apparently this *isn't* consistent, so we need to handle it better
        let status = value.get("status")
            .and_then(|v| v.as_u64())
            .ok_or_else(|| serde::de::Error::custom("Missing or invalid 'status' field"))?
            as u16;
        
        // Extract sub_status - try both snake_case and camelCase
        let sub_status = value.get("sub_status")
            .or_else(|| value.get("subStatus"))
            .and_then(|v| v.as_u64())
            .ok_or_else(|| serde::de::Error::custom("Missing or invalid 'sub_status'/'subStatus' field"))?;
        
        // Extract user_message - try both snake_case and camelCase, default to empty string
        let user_message = value.get("user_message")
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
        write!(
            f,
            "Tidal API error: {} {} {}",
            self.status, self.sub_status, self.user_message
        )
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