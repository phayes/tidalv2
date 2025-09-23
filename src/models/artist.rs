use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Artist {
    #[serde(rename = "attributes", skip_serializing_if = "Option::is_none")]
    pub attributes: Option<ArtistAttributes>,
    /// Resource id
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "relationships", skip_serializing_if = "Option::is_none")]
    pub relationships: Option<models::ArtistsRelationships>,
    /// Resource type
    #[serde(rename = "type")]
    pub r#type: String,
}

impl Artist {
    pub fn new(id: String, r#type: String) -> Artist {
        Artist {
            attributes: None,
            id,
            relationships: None,
            r#type,
        }
    }
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ArtistAttributes {
    /// Is the artist enabled for contributions?
    #[serde(
        rename = "contributionsEnabled",
        skip_serializing_if = "Option::is_none"
    )]
    pub contributions_enabled: Option<bool>,
    /// Contributions sales pitch
    #[serde(
        rename = "contributionsSalesPitch",
        skip_serializing_if = "Option::is_none"
    )]
    pub contributions_sales_pitch: Option<String>,
    /// Artist links external to TIDAL API
    #[serde(rename = "externalLinks", skip_serializing_if = "Option::is_none")]
    pub external_links: Option<Vec<models::ExternalLink>>,
    /// Artist handle
    #[serde(rename = "handle", skip_serializing_if = "Option::is_none")]
    pub handle: Option<String>,
    /// Artist name
    #[serde(rename = "name")]
    pub name: String,
    /// Artist popularity (0.0 - 1.0)
    #[serde(rename = "popularity")]
    pub popularity: f64,
    /// Is the artist spotlighted?
    #[serde(rename = "spotlighted", skip_serializing_if = "Option::is_none")]
    pub spotlighted: Option<bool>,
}

impl ArtistAttributes {
    pub fn new(name: String, popularity: f64) -> ArtistAttributes {
        ArtistAttributes {
            contributions_enabled: None,
            contributions_sales_pitch: None,
            external_links: None,
            handle: None,
            name,
            popularity,
            spotlighted: None,
        }
    }
}
