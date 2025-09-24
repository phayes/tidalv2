use crate::models::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct SearchResult {
    #[serde(rename = "attributes", skip_serializing_if = "Option::is_none")]
    pub attributes: Option<SearchResultAttributes>,
    /// Resource id
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "relationships", skip_serializing_if = "Option::is_none")]
    pub relationships: Option<SearchResultsRelationships>,
    /// Resource type
    #[serde(rename = "type")]
    pub r#type: String,
}

impl SearchResult {
    pub fn new(id: String, r#type: String) -> SearchResult {
        SearchResult {
            attributes: None,
            id,
            relationships: None,
            r#type,
        }
    }
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct SearchResultAttributes {
    /// 'did you mean' prompt
    #[serde(rename = "didYouMean", skip_serializing_if = "Option::is_none")]
    pub did_you_mean: Option<String>,
    /// search request unique tracking number
    #[serde(rename = "trackingId")]
    pub tracking_id: String,
}

impl SearchResultAttributes {
    pub fn new(tracking_id: String) -> SearchResultAttributes {
        SearchResultAttributes {
            did_you_mean: None,
            tracking_id,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SearchResultsRelationships {
    #[serde(rename = "albums")]
    pub albums: MultiRelationship<ResourceIdentifier>,
    #[serde(rename = "artists")]
    pub artists: MultiRelationship<ResourceIdentifier>,
    #[serde(rename = "playlists")]
    pub playlists: MultiRelationship<ResourceIdentifier>,
    #[serde(rename = "topHits")]
    pub top_hits: MultiRelationship<ResourceIdentifier>,
    #[serde(rename = "tracks")]
    pub tracks: MultiRelationship<ResourceIdentifier>,
    #[serde(rename = "videos")]
    pub videos: MultiRelationship<ResourceIdentifier>,
}

impl SearchResultsRelationships {
    pub fn new(
        albums: MultiRelationship<ResourceIdentifier>,
        artists: MultiRelationship<ResourceIdentifier>,
        playlists: MultiRelationship<ResourceIdentifier>,
        top_hits: MultiRelationship<ResourceIdentifier>,
        tracks: MultiRelationship<ResourceIdentifier>,
        videos: MultiRelationship<ResourceIdentifier>,
    ) -> SearchResultsRelationships {
        SearchResultsRelationships {
            albums,
            artists,
            playlists,
            top_hits,
            tracks,
            videos,
        }
    }
}
