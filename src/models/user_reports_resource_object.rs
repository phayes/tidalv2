use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct UserReportAttributes {
    /// Description
    #[serde(rename = "description")]
    pub description: String,
    /// Reason
    #[serde(rename = "reason")]
    pub reason: ReasonFalse,
}

impl UserReportAttributes {
    pub fn new(description: String, reason: ReasonFalse) -> UserReportAttributes {
        UserReportAttributes {
            description,
            reason,
        }
    }
}

/// Reason
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ReasonFalse {
    #[serde(rename = "SEXUAL_CONTENT_OR_NUDITY")]
    SexualContentOrNudity,
    #[serde(rename = "VIOLENT_OR_DANGEROUS_CONTENT")]
    ViolentOrDangerousContent,
    #[serde(rename = "HATEFUL_OR_ABUSIVE_CONTENT")]
    HatefulOrAbusiveContent,
    #[serde(rename = "HARASSMENT")]
    Harassment,
    #[serde(rename = "PRIVACY_VIOLATION")]
    PrivacyViolation,
    #[serde(rename = "SCAMS_OR_FRAUD")]
    ScamsOrFraud,
    #[serde(rename = "SPAM")]
    Spam,
    #[serde(rename = "COPYRIGHT_INFRINGEMENT")]
    CopyrightInfringement,
    #[serde(rename = "UNKNOWN")]
    Unknown,
}

impl Default for ReasonFalse {
    fn default() -> ReasonFalse {
        Self::SexualContentOrNudity
    }
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct UserReport {
    #[serde(rename = "attributes", skip_serializing_if = "Option::is_none")]
    pub attributes: Option<UserReportAttributes>,
    /// Resource id
    #[serde(rename = "id")]
    pub id: String,
    /// Resource type
    #[serde(rename = "type")]
    pub r#type: String,
}

impl UserReport {
    pub fn new(id: String, r#type: String) -> UserReport {
        UserReport {
            attributes: None,
            id,
            r#type,
        }
    }
}
