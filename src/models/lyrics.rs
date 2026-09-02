use crate::models::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Lyrics {
    #[serde(rename = "attributes", default)]
    pub attributes: LyricsAttributes,
    /// Resource id
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "relationships", skip_serializing_if = "Option::is_none")]
    pub relationships: Option<LyricsRelationships>,
    /// Resource type
    #[serde(rename = "type")]
    pub r#type: String,
}

impl Lyrics {
    pub fn new(id: String, r#type: String) -> Lyrics {
        Lyrics {
            attributes: LyricsAttributes::default(),
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

#[derive(
    Clone, Copy, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize,
)]
pub enum LyricsTechnicalStatus {
    #[serde(rename = "PENDING")]
    #[default]
    Pending,
    #[serde(rename = "PROCESSING")]
    Processing,
    #[serde(rename = "ERROR")]
    Error,
    #[serde(rename = "OK")]
    Ok,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LyricsRelationships {
    #[serde(rename = "owners")]
    pub owners: MultiRelationship<ResourceIdentifier>,
    #[serde(rename = "track")]
    pub track: Relationship,
}

impl LyricsRelationships {
    pub fn new(
        owners: MultiRelationship<ResourceIdentifier>,
        track: Relationship,
    ) -> LyricsRelationships {
        LyricsRelationships { owners, track }
    }
}
