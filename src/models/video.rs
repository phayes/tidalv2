use crate::models::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Video {
    #[serde(rename = "attributes", skip_serializing_if = "Option::is_none")]
    pub attributes: Option<VideoAttributes>,
    /// Resource id
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "relationships", skip_serializing_if = "Option::is_none")]
    pub relationships: Option<VideoRelationships>,
    /// Resource type
    #[serde(rename = "type")]
    pub r#type: String,
}

impl Video {
    pub fn new(id: String, r#type: String) -> Video {
        Video {
            attributes: None,
            id,
            relationships: None,
            r#type,
        }
    }
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct VideoAttributes {
    /// Available usage for this video
    #[serde(rename = "availability", skip_serializing_if = "Option::is_none")]
    pub availability: Option<Vec<VideoAvailability>>,
    #[serde(rename = "copyright", skip_serializing_if = "Option::is_none")]
    pub copyright: Option<copyright::Copyright>,
    /// Duration (ISO 8601)
    #[serde(rename = "duration")]
    pub duration: String,
    /// Explicit content
    #[serde(rename = "explicit")]
    pub explicit: bool,
    /// Video links external to TIDAL API
    #[serde(rename = "externalLinks", skip_serializing_if = "Option::is_none")]
    pub external_links: Option<Vec<ExternalLink>>,
    /// International Standard Recording Code (ISRC)
    #[serde(rename = "isrc")]
    pub isrc: String,
    /// Popularity (0.0 - 1.0)
    #[serde(rename = "popularity")]
    pub popularity: f64,
    /// Release date (ISO-8601)
    #[serde(rename = "releaseDate", skip_serializing_if = "Option::is_none")]
    pub release_date: Option<String>,
    /// Video title
    #[serde(rename = "title")]
    pub title: String,
    /// Video version, complements title
    #[serde(rename = "version", skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

impl VideoAttributes {
    pub fn new(
        duration: String,
        explicit: bool,
        isrc: String,
        popularity: f64,
        title: String,
    ) -> VideoAttributes {
        VideoAttributes {
            availability: None,
            copyright: None,
            duration,
            explicit,
            external_links: None,
            isrc,
            popularity,
            release_date: None,
            title,
            version: None,
        }
    }
}

/// Available usage for this video
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum VideoAvailability {
    #[serde(rename = "STREAM")]
    Stream,
    #[serde(rename = "DJ")]
    Dj,
    #[serde(rename = "STEM")]
    Stem,
}

impl Default for VideoAvailability {
    fn default() -> VideoAvailability {
        Self::Stream
    }
}


#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VideoRelationships {
    #[serde(rename = "albums")]
    pub albums: MultiRelationship<ResourceIdentifier>,
    #[serde(rename = "artists")]
    pub artists: MultiRelationship<ResourceIdentifier>,
    #[serde(rename = "providers")]
    pub providers: MultiRelationship<ResourceIdentifier>,
    #[serde(rename = "thumbnailArt")]
    pub thumbnail_art: MultiRelationship<ResourceIdentifier>,
}

impl VideoRelationships {
    pub fn new(
        albums: MultiRelationship<ResourceIdentifier>,
        artists: MultiRelationship<ResourceIdentifier>,
        providers: MultiRelationship<ResourceIdentifier>,
        thumbnail_art: MultiRelationship<ResourceIdentifier>,
    ) -> VideoRelationships {
        VideoRelationships {
            albums,
            artists,
            providers,
            thumbnail_art,
        }
    }
}
