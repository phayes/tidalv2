use crate::models;
use models::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Artist {
    #[serde(rename = "attributes", default)]
    pub attributes: ArtistAttributes,
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
            attributes: ArtistAttributes::default(),
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
    #[serde(rename = "biography", skip_serializing_if = "Option::is_none")]
    pub biography: Option<Relationship>,
    #[serde(rename = "owners", skip_serializing_if = "Option::is_none")]
    pub owners: Option<MultiRelationship<ResourceIdentifier>>,
    #[serde(rename = "profileArt", skip_serializing_if = "Option::is_none")]
    pub profile_art: Option<MultiRelationship<ResourceIdentifier>>,
    #[serde(rename = "radio", skip_serializing_if = "Option::is_none")]
    pub radio: Option<MultiRelationship<ResourceIdentifier>>,
    #[serde(rename = "roles", skip_serializing_if = "Option::is_none")]
    pub roles: Option<MultiRelationship<ResourceIdentifier>>,
    #[serde(rename = "similarArtists", skip_serializing_if = "Option::is_none")]
    pub similar_artists: Option<MultiRelationship<ResourceIdentifier>>,
    #[serde(rename = "trackProviders", skip_serializing_if = "Option::is_none")]
    pub track_providers: Option<
        MultiRelationship<
            ResourceIdentifier<crate::apis::artists_api::ArtistsTrackProvidersResourceMeta>,
        >,
    >,
    #[serde(rename = "tracks")]
    pub tracks: MultiRelationship<ResourceIdentifier>,
    #[serde(rename = "videos", skip_serializing_if = "Option::is_none")]
    pub videos: Option<MultiRelationship<ResourceIdentifier>>,
}
