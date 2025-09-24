use crate::models;
use models::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Artist {
    #[serde(rename = "attributes", skip_serializing_if = "Option::is_none")]
    pub attributes: Option<ArtistAttributes>,
    /// Resource id
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "relationships", skip_serializing_if = "Option::is_none")]
    pub relationships: Option<ArtistsRelationships>,
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

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ArtistsRelationships {
    #[serde(rename = "albums")]
    pub albums: MultiRelationship<ResourceIdentifier>,
    #[serde(rename = "biography")]
    pub biography: Relationship,
    #[serde(rename = "owners")]
    pub owners: MultiRelationship<ResourceIdentifier>,
    #[serde(rename = "profileArt")]
    pub profile_art: MultiRelationship<ResourceIdentifier>,
    #[serde(rename = "radio")]
    pub radio: MultiRelationship<ResourceIdentifier>,
    #[serde(rename = "roles")]
    pub roles: MultiRelationship<ResourceIdentifier>,
    #[serde(rename = "similarArtists")]
    pub similar_artists: MultiRelationship<ResourceIdentifier>,
    #[serde(rename = "trackProviders")]
    pub track_providers: MultiRelationship<
        ResourceIdentifier<crate::apis::artists_api::ArtistsTrackProvidersResourceMeta>,
    >,
    #[serde(rename = "tracks")]
    pub tracks: MultiRelationship<ResourceIdentifier>,
    #[serde(rename = "videos")]
    pub videos: MultiRelationship<ResourceIdentifier>,
}
