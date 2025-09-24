use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreatePlaylist {
    #[serde(rename = "attributes")]
    pub attributes: models::CreatePlaylistAttributes,
    /// Resource type - Must be [`models::ResourceType::Playlists`]
    #[serde(rename = "type")]
    pub r#type: models::ResourceType,
}

impl CreatePlaylist {
    pub fn new(attributes: models::CreatePlaylistAttributes) -> CreatePlaylist {
        CreatePlaylist {
            attributes,
            r#type: models::ResourceType::Playlists,
        }
    }
}
