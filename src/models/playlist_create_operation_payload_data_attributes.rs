use crate::models;
use serde::{Deserialize, Serialize};

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
/// Access type
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum PlaylistAccess {
    #[serde(rename = "PUBLIC")]
    Public,
    #[serde(rename = "UNLISTED")]
    Unlisted,
}

impl Default for PlaylistAccess {
    fn default() -> PlaylistAccess {
        Self::Public
    }
}
