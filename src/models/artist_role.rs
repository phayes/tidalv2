use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ArtistRole {
    #[serde(rename = "attributes", skip_serializing_if = "Option::is_none")]
    pub attributes: Option<ArtistRoleAttributes>,
    /// Resource id
    #[serde(rename = "id")]
    pub id: String,
    /// Resource type
    #[serde(rename = "type")]
    pub r#type: String,
}

impl ArtistRole {
    pub fn new(id: String, r#type: String) -> ArtistRole {
        ArtistRole {
            attributes: None,
            id,
            r#type,
        }
    }
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ArtistRoleAttributes {
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl ArtistRoleAttributes {
    pub fn new() -> ArtistRoleAttributes {
        ArtistRoleAttributes { name: None }
    }
}
