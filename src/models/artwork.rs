use crate::models::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Artwork {
    #[serde(rename = "attributes", skip_serializing_if = "Option::is_none")]
    pub attributes: Option<ArtworkAttributes>,
    /// Resource id
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "relationships", skip_serializing_if = "Option::is_none")]
    pub relationships: Option<ArtworkRelationships>,
    /// Resource type
    #[serde(rename = "type")]
    pub r#type: String,
}

impl Artwork {
    pub fn new(id: String, r#type: String) -> Artwork {
        Artwork {
            attributes: None,
            id,
            relationships: None,
            r#type,
        }
    }
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ArtworkAttributes {
    /// Artwork files
    #[serde(rename = "files")]
    pub files: Vec<artwork_file::ArtworkFile>,
    /// Media type of artwork files
    #[serde(rename = "mediaType")]
    pub media_type: ArtworkMediaType,
}

impl ArtworkAttributes {
    pub fn new(
        files: Vec<artwork_file::ArtworkFile>,
        media_type: ArtworkMediaType,
    ) -> ArtworkAttributes {
        ArtworkAttributes { files, media_type }
    }
}

/// Media type of artwork files
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ArtworkMediaType {
    #[serde(rename = "IMAGE")]
    Image,
    #[serde(rename = "VIDEO")]
    Video,
}

impl Default for ArtworkMediaType {
    fn default() -> ArtworkMediaType {
        Self::Image
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ArtworkRelationships {
    #[serde(rename = "owners")]
    pub owners: MultiRelationship<ResourceIdentifier>,
}

impl ArtworkRelationships {
    pub fn new(owners: MultiRelationship<ResourceIdentifier>) -> ArtworkRelationships {
        ArtworkRelationships { owners }
    }
}
