use crate::models::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Album {
    #[serde(rename = "attributes", default)]
    pub attributes: AlbumAttributes,
    /// Resource id
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "relationships", skip_serializing_if = "Option::is_none")]
    pub relationships: Option<album::AlbumsRelationships>,
    /// Resource type
    #[serde(rename = "type")]
    pub r#type: String,
}

impl Album {
    pub fn new(id: String, r#type: String) -> Album {
        Album {
            attributes: AlbumAttributes::default(),
            id,
            relationships: None,
            r#type,
        }
    }
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct AlbumAttributes {
    /// Available usage for this album
    #[serde(rename = "availability", skip_serializing_if = "Option::is_none")]
    pub availability: Option<Vec<AlbumAvailability>>,
    /// Barcode id (EAN-13 or UPC-A)
    #[serde(rename = "barcodeId")]
    pub barcode_id: String,
    #[serde(rename = "copyright", skip_serializing_if = "Option::is_none")]
    pub copyright: Option<copyright::Copyright>,
    /// Duration (ISO 8601)
    #[serde(rename = "duration")]
    pub duration: String,
    /// Explicit content
    #[serde(rename = "explicit")]
    pub explicit: bool,
    /// Album links external to TIDAL API
    #[serde(rename = "externalLinks", skip_serializing_if = "Option::is_none")]
    pub external_links: Option<Vec<ExternalLink>>,
    #[serde(rename = "mediaTags")]
    pub media_tags: Vec<String>,
    /// Number of items in album
    #[serde(rename = "numberOfItems")]
    pub number_of_items: i32,
    /// Number of volumes
    #[serde(rename = "numberOfVolumes")]
    pub number_of_volumes: i32,
    /// Popularity (0.0 - 1.0)
    #[serde(rename = "popularity")]
    pub popularity: f64,
    /// Release date (ISO-8601)
    #[serde(rename = "releaseDate", skip_serializing_if = "Option::is_none")]
    pub release_date: Option<String>,
    /// Album title
    #[serde(rename = "title")]
    pub title: String,
    /// Album type
    #[serde(rename = "type")]
    pub r#type: AlbumType,
    /// Album version
    #[serde(rename = "version", skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

impl AlbumAttributes {
    pub fn new(
        barcode_id: String,
        duration: String,
        explicit: bool,
        media_tags: Vec<String>,
        number_of_items: i32,
        number_of_volumes: i32,
        popularity: f64,
        title: String,
        r#type: AlbumType,
    ) -> AlbumAttributes {
        AlbumAttributes {
            availability: None,
            barcode_id,
            copyright: None,
            duration,
            explicit,
            external_links: None,
            media_tags,
            number_of_items,
            number_of_volumes,
            popularity,
            release_date: None,
            title,
            r#type,
            version: None,
        }
    }
}

/// Available usage for this album
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum AlbumAvailability {
    #[serde(rename = "STREAM")]
    Stream,
    #[serde(rename = "DJ")]
    Dj,
    #[serde(rename = "STEM")]
    Stem,
}

impl Default for AlbumAvailability {
    fn default() -> AlbumAvailability {
        Self::Stream
    }
}

/// Album type
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum AlbumType {
    #[serde(rename = "ALBUM")]
    Album,
    #[serde(rename = "EP")]
    Ep,
    #[serde(rename = "SINGLE")]
    Single,
}

impl Default for AlbumType {
    fn default() -> AlbumType {
        Self::Album
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AlbumsRelationships {
    #[serde(rename = "artists")]
    pub artists: MultiRelationship<ResourceIdentifier>,
    #[serde(rename = "coverArt")]
    pub cover_art: MultiRelationship<ResourceIdentifier>,
    #[serde(rename = "genres")]
    pub genres: MultiRelationship<ResourceIdentifier>,
    #[serde(rename = "items")]
    pub items: MultiRelationship<ResourceIdentifier<AlbumsItemsResourceMeta>>,
    #[serde(rename = "owners")]
    pub owners: MultiRelationship<ResourceIdentifier>,
    #[serde(rename = "providers")]
    pub providers: MultiRelationship<ResourceIdentifier>,
    #[serde(rename = "similarAlbums")]
    pub similar_albums: MultiRelationship<ResourceIdentifier>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AlbumsItemsResourceMeta {
    /// track number
    #[serde(rename = "trackNumber")]
    pub track_number: i32,
    /// volume number
    #[serde(rename = "volumeNumber")]
    pub volume_number: i32,
}
