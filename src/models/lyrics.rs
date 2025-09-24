use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Lyrics {
    #[serde(rename = "attributes", skip_serializing_if = "Option::is_none")]
    pub attributes: Option<LyricsAttributes>,
    /// Resource id
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "relationships", skip_serializing_if = "Option::is_none")]
    pub relationships: Option<models::LyricsRelationships>,
    /// Resource type
    #[serde(rename = "type")]
    pub r#type: String,
}

impl Lyrics {
    pub fn new(id: String, r#type: String) -> Lyrics {
        Lyrics {
            attributes: None,
            id,
            relationships: None,
            r#type,
        }
    }
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct LyricsAttributes {
    #[serde(rename = "technicalStatus")]
    pub technical_status: LyricsTechnicalStatus,
    #[serde(rename = "text", skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
}

impl LyricsAttributes {
    pub fn new(technical_status: LyricsTechnicalStatus) -> LyricsAttributes {
        LyricsAttributes {
            technical_status,
            text: None,
        }
    }
}

///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum LyricsTechnicalStatus {
    #[serde(rename = "PENDING")]
    Pending,
    #[serde(rename = "PROCESSING")]
    Processing,
    #[serde(rename = "ERROR")]
    Error,
    #[serde(rename = "OK")]
    Ok,
}

impl Default for LyricsTechnicalStatus {
    fn default() -> LyricsTechnicalStatus {
        Self::Pending
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LyricsRelationships {
    #[serde(rename = "owners")]
    pub owners: models::MultiRelationship<models::ResourceIdentifier>,
    #[serde(rename = "track")]
    pub track: models::Relationship,
}

impl LyricsRelationships {
    pub fn new(
        owners: models::MultiRelationship<models::ResourceIdentifier>,
        track: models::Relationship,
    ) -> LyricsRelationships {
        LyricsRelationships { owners, track }
    }
}
