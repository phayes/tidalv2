use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Artwork {
    #[serde(rename = "attributes", skip_serializing_if = "Option::is_none")]
    pub attributes: Option<ArtworkAttributes>,
    /// Resource id
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "relationships", skip_serializing_if = "Option::is_none")]
    pub relationships: Option<models::ArtworksRelationships>,
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
    pub files: Vec<models::ArtworkFile>,
    /// Media type of artwork files
    #[serde(rename = "mediaType")]
    pub media_type: MediaTypeFalse,
    #[serde(rename = "sourceFile", skip_serializing_if = "Option::is_none")]
    pub source_file: Option<models::ArtworkSourceFile>,
}

impl ArtworkAttributes {
    pub fn new(files: Vec<models::ArtworkFile>, media_type: MediaTypeFalse) -> ArtworkAttributes {
        ArtworkAttributes {
            files,
            media_type,
            source_file: None,
        }
    }
}

/// Media type of artwork files
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum MediaTypeFalse {
    #[serde(rename = "IMAGE")]
    Image,
    #[serde(rename = "VIDEO")]
    Video,
}

impl Default for MediaTypeFalse {
    fn default() -> MediaTypeFalse {
        Self::Image
    }
}
