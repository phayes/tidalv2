use crate::models::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ArtistBiography {
    #[serde(rename = "attributes", skip_serializing_if = "Option::is_none")]
    pub attributes: Option<ArtistBiographyAttributes>,
    /// Resource id
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "relationships", skip_serializing_if = "Option::is_none")]
    pub relationships: Option<ArtistBiographyRelationships>,
    /// Resource type
    #[serde(rename = "type")]
    pub r#type: String,
}

impl ArtistBiography {
    pub fn new(id: String, r#type: String) -> ArtistBiography {
        ArtistBiography {
            attributes: None,
            id,
            relationships: None,
            r#type,
        }
    }
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ArtistBiographyAttributes {
    /// Boolean to indicate if the biography is editable (source = tidal or artist)
    #[serde(rename = "editable")]
    pub editable: bool,
    /// Artist biography
    #[serde(rename = "text")]
    pub text: String,
}

impl ArtistBiographyAttributes {
    pub fn new(editable: bool, text: String) -> ArtistBiographyAttributes {
        ArtistBiographyAttributes { editable, text }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ArtistBiographyRelationships {
    #[serde(rename = "owners")]
    pub owners: MultiRelationship<ResourceIdentifier>,
}
