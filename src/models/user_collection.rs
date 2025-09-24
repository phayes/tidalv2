use crate::models::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct UserCollection {
    #[serde(rename = "attributes", skip_serializing_if = "Option::is_none")]
    pub attributes: Option<serde_json::Value>,
    /// Resource id
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "relationships", skip_serializing_if = "Option::is_none")]
    pub relationships: Option<UserCollectionsRelationships>,
    /// Resource type
    #[serde(rename = "type")]
    pub r#type: String,
}

impl UserCollection {
    pub fn new(id: String, r#type: String) -> UserCollection {
        UserCollection {
            attributes: None,
            id,
            relationships: None,
            r#type,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UserCollectionsRelationships {
    #[serde(rename = "albums")]
    pub albums: MultiRelationship<ResourceIdentifier<UserCollectionsResourceMeta>>,
    #[serde(rename = "artists")]
    pub artists: MultiRelationship<ResourceIdentifier<UserCollectionsResourceMeta>>,
    #[serde(rename = "owners")]
    pub owners: MultiRelationship<ResourceIdentifier>,
    #[serde(rename = "playlists")]
    pub playlists: MultiRelationship<ResourceIdentifier<UserCollectionsResourceMeta>>,
    #[serde(rename = "tracks")]
    pub tracks: MultiRelationship<ResourceIdentifier<UserCollectionsResourceMeta>>,
    #[serde(rename = "videos")]
    pub videos: MultiRelationship<ResourceIdentifier<UserCollectionsResourceMeta>>,
}

impl UserCollectionsRelationships {
    pub fn new(
        albums: MultiRelationship<ResourceIdentifier<UserCollectionsResourceMeta>>,
        artists: MultiRelationship<ResourceIdentifier<UserCollectionsResourceMeta>>,
        owners: MultiRelationship<ResourceIdentifier>,
        playlists: MultiRelationship<ResourceIdentifier<UserCollectionsResourceMeta>>,
        tracks: MultiRelationship<ResourceIdentifier<UserCollectionsResourceMeta>>,
        videos: MultiRelationship<ResourceIdentifier<UserCollectionsResourceMeta>>,
    ) -> UserCollectionsRelationships {
        UserCollectionsRelationships {
            albums,
            artists,
            owners,
            playlists,
            tracks,
            videos,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UserCollectionsResourceMeta {
    #[serde(rename = "addedAt")]
    added_at: String,
}
