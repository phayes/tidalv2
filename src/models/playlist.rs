use crate::models::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Playlist {
    #[serde(rename = "attributes", default)]
    pub attributes: PlaylistAttributes,
    /// Resource id
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "relationships", skip_serializing_if = "Option::is_none")]
    pub relationships: Option<PlaylistRelationships>,
    /// Resource type
    #[serde(rename = "type")]
    pub r#type: String,
}

impl Playlist {
    pub fn new(id: String, r#type: String) -> Playlist {
        Playlist {
            attributes: PlaylistAttributes::default(),
            id,
            relationships: None,
            r#type,
        }
    }
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct PlaylistAttributes {
    /// Access type
    #[serde(rename = "accessType")]
    pub access_type: PlaylistAccess,
    /// Indicates if the playlist has a duration and set number of tracks
    #[serde(rename = "bounded")]
    pub bounded: bool,
    /// Datetime of playlist creation (ISO 8601)
    #[serde(rename = "createdAt")]
    pub created_at: String,
    /// Playlist description
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Duration of playlist (ISO 8601)
    #[serde(rename = "duration", skip_serializing_if = "Option::is_none")]
    pub duration: Option<String>,
    #[serde(rename = "externalLinks")]
    pub external_links: Vec<ExternalLink>,
    /// Datetime of last modification of the playlist (ISO 8601)
    #[serde(rename = "lastModifiedAt")]
    pub last_modified_at: String,
    /// Playlist name
    #[serde(rename = "name")]
    pub name: String,
    /// Number of items in the playlist
    #[serde(rename = "numberOfItems", skip_serializing_if = "Option::is_none")]
    pub number_of_items: Option<i32>,
    /// The type of the playlist
    #[serde(rename = "playlistType")]
    pub playlist_type: PlaylistType,
}

impl PlaylistAttributes {
    pub fn new(
        access_type: PlaylistAccess,
        bounded: bool,
        created_at: String,
        external_links: Vec<ExternalLink>,
        last_modified_at: String,
        name: String,
        playlist_type: PlaylistType,
    ) -> PlaylistAttributes {
        PlaylistAttributes {
            access_type,
            bounded,
            created_at,
            description: None,
            duration: None,
            external_links,
            last_modified_at,
            name,
            number_of_items: None,
            playlist_type,
        }
    }
}

/// Access type
#[derive(
    Clone, Copy, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize,
)]
pub enum PlaylistAccess {
    #[serde(rename = "UNLISTED")]
    #[default]
    Unlisted,
    #[serde(rename = "PUBLIC")]
    Public,
}

/// The type of the playlist
#[derive(
    Clone, Copy, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize,
)]
pub enum PlaylistType {
    #[serde(rename = "USER")]
    #[default]
    User,
    #[serde(rename = "EDITORIAL")]
    Editorial,
    #[serde(rename = "MIX")]
    Mix,
    #[serde(rename = "ARTIST")]
    Artist,
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct PlaylistItemMeta {
    #[serde(rename = "itemId")]
    pub item_id: String,
}

impl PlaylistItemMeta {
    pub fn new(item_id: String) -> PlaylistItemMeta {
        PlaylistItemMeta { item_id }
    }
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct PlaylistItemPosition {
    #[serde(rename = "positionBefore")]
    pub position_before: String,
}

impl PlaylistItemPosition {
    pub fn new(position_before: String) -> PlaylistItemPosition {
        PlaylistItemPosition { position_before }
    }
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct PlaylistsItemsIdentifierMeta {
    #[serde(rename = "addedAt", skip_serializing_if = "Option::is_none")]
    pub added_at: Option<String>,
    #[serde(rename = "itemId", skip_serializing_if = "Option::is_none")]
    pub item_id: Option<String>,
}

impl PlaylistsItemsIdentifierMeta {
    pub fn new() -> PlaylistsItemsIdentifierMeta {
        PlaylistsItemsIdentifierMeta {
            added_at: None,
            item_id: None,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PlaylistRelationships {
    #[serde(rename = "coverArt", skip_serializing_if = "Option::is_none")]
    pub cover_art: Option<MultiRelationship<ResourceIdentifier>>,
    #[serde(rename = "items")]
    pub items: MultiRelationship<ResourceIdentifier<PlaylistsItemsIdentifierMeta>>,
    #[serde(rename = "owners", skip_serializing_if = "Option::is_none")]
    pub owners: Option<MultiRelationship<ResourceIdentifier>>,
}

impl PlaylistRelationships {
    pub fn new(
        cover_art: Option<MultiRelationship<ResourceIdentifier>>,
        items: MultiRelationship<ResourceIdentifier<PlaylistsItemsIdentifierMeta>>,
        owners: Option<MultiRelationship<ResourceIdentifier>>,
    ) -> PlaylistRelationships {
        PlaylistRelationships {
            cover_art,
            items,
            owners,
        }
    }
}

/*------------------------------------------------*/
/* Playlist Modification types                 */
/*------------------------------------------------*/

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreatePlaylist {
    #[serde(rename = "attributes")]
    pub attributes: CreatePlaylistAttributes,
    /// Resource type - Must be [`ResourceType::Playlists`]
    #[serde(rename = "type")]
    pub r#type: ResourceType,
}

impl CreatePlaylist {
    pub fn new(attributes: CreatePlaylistAttributes) -> CreatePlaylist {
        CreatePlaylist {
            attributes,
            r#type: ResourceType::Playlists,
        }
    }
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreatePlaylistAttributes {
    /// Access type
    #[serde(rename = "accessType", skip_serializing_if = "Option::is_none")]
    pub access_type: Option<PlaylistAccess>,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "name")]
    pub name: String,
}

impl CreatePlaylistAttributes {
    pub fn new(name: String) -> CreatePlaylistAttributes {
        CreatePlaylistAttributes {
            access_type: None,
            description: None,
            name,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UpdatePlaylist {
    #[serde(rename = "attributes")]
    pub attributes: UpdatePlaylistAttributes,
    #[serde(rename = "id")]
    pub id: String,
    /// Resource type - Must be [`ResourceType::Playlists`]
    #[serde(rename = "type")]
    pub r#type: ResourceType,
}

impl UpdatePlaylist {
    pub fn new(attributes: UpdatePlaylistAttributes, id: String) -> UpdatePlaylist {
        UpdatePlaylist {
            attributes,
            id,
            r#type: ResourceType::Playlists,
        }
    }
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct UpdatePlaylistAttributes {
    /// Access type
    #[serde(rename = "accessType", skip_serializing_if = "Option::is_none")]
    pub access_type: Option<PlaylistAccess>,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl UpdatePlaylistAttributes {
    pub fn new() -> UpdatePlaylistAttributes {
        UpdatePlaylistAttributes {
            access_type: None,
            description: None,
            name: None,
        }
    }
}
