use crate::models::{
    album::Album, artist::Artist, artist_biography::ArtistBiography, artist_role::ArtistRole,
    artwork::Artwork, genre::Genre, Links, lyrics::Lyrics, playlist::Playlist, provider::Provider, SearchResult, SearchSuggestion, track::Track,
    track_file::TrackFile, track_manifest::TrackManifest, track_statistics::TrackStatistics, user::User, user_collection::UserCollection,
    user_recommendation::UserRecommendation, user_share::UserShare, video::Video,
};

// Generic Resource struct for all single resource data documents
use serde::{de::DeserializeOwned, Deserialize, Deserializer, Serialize, Serializer};
use serde_json::Value;
use strum::{AsRefStr, Display, IntoStaticStr};

#[derive(
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    Ord,
    PartialOrd,
    Hash,
    Serialize,
    Deserialize,
    Display,
    AsRefStr,
    IntoStaticStr,
)]
#[serde(rename_all = "camelCase")]
#[strum(serialize_all = "camelCase")]
pub enum ResourceType {
    Albums,
    Appreciations,
    ArtistBiographies,
    ArtistRoles,
    Artists,
    Artworks,
    Genres,
    Lyrics,
    Playlists,
    Providers,
    SearchResults,
    SearchSuggestions,
    TrackFiles,
    TrackManifests,
    TrackSourceFiles,
    TrackStatistics,
    Tracks,
    UserCollections,
    UserEntitlements,
    UserRecommendations,
    UserShares,
    Users,
    Videos,
}

// AnyResource - all possible resource types
#[derive(Clone, Debug, PartialEq)]
pub enum AnyResource {

    /// Album
    Albums(Album),

    /// Not supported
    Appreciations(Value),
    
    /// Artist Biography
    ArtistBiographies(ArtistBiography),

    /// Artist Role
    ArtistRoles(ArtistRole),

    /// Artist
    Artists(Artist),

    /// Artwork
    Artworks(Artwork),

    /// Genre
    Genres(Genre),

    /// Lyrics
    Lyrics(Lyrics),

    /// Playlist
    Playlists(Playlist),

    /// Provider
    Providers(Provider),

    /// Search Result
    SearchResults(SearchResult),

    /// Search Suggestion
    SearchSuggestions(SearchSuggestion),

    /// Track File
    TrackFiles(TrackFile),

    /// Track Manifest
    TrackManifests(TrackManifest),

    /// Not supported
    TrackSourceFiles(Value),
    
    /// Track Statistics
    TrackStatistics(TrackStatistics),

    /// Track
    Tracks(Track),

    /// User Collection (favoutites)
    UserCollections(UserCollection),

    /// Not supported
    UserEntitlements(Value),  

    /// User Recommendation
    UserRecommendations(UserRecommendation),

    /// User Share
    UserShares(UserShare),

    /// User
    Users(User),

    /// Video
    Videos(Video),
}

impl AnyResource {
    pub fn resource_type(&self) -> ResourceType {
        match self {
            AnyResource::Albums(_) => ResourceType::Albums,
            AnyResource::Appreciations(_) => ResourceType::Appreciations,
            AnyResource::ArtistBiographies(_) => ResourceType::ArtistBiographies,
            AnyResource::ArtistRoles(_) => ResourceType::ArtistRoles,
            AnyResource::Artists(_) => ResourceType::Artists,
            AnyResource::Artworks(_) => ResourceType::Artworks,
            AnyResource::Genres(_) => ResourceType::Genres,
            AnyResource::Lyrics(_) => ResourceType::Lyrics,
            AnyResource::Playlists(_) => ResourceType::Playlists,
            AnyResource::Providers(_) => ResourceType::Providers,
            AnyResource::SearchResults(_) => ResourceType::SearchResults,
            AnyResource::SearchSuggestions(_) => ResourceType::SearchSuggestions,
            AnyResource::TrackFiles(_) => ResourceType::TrackFiles,
            AnyResource::TrackManifests(_) => ResourceType::TrackManifests,
            AnyResource::TrackSourceFiles(_) => ResourceType::TrackSourceFiles,
            AnyResource::TrackStatistics(_) => ResourceType::TrackStatistics,
            AnyResource::Tracks(_) => ResourceType::Tracks,
            AnyResource::UserCollections(_) => ResourceType::UserCollections,
            AnyResource::UserEntitlements(_) => ResourceType::UserEntitlements,
            AnyResource::UserRecommendations(_) => ResourceType::UserRecommendations,
            AnyResource::UserShares(_) => ResourceType::UserShares,
            AnyResource::Users(_) => ResourceType::Users,
            AnyResource::Videos(_) => ResourceType::Videos,
        }
    }
}

impl From<AnyResource> for ResourceType {
    fn from(val: AnyResource) -> Self {
        val.resource_type()
    }
}

impl Default for AnyResource {
    fn default() -> Self {
        Self::Albums(Default::default())
    }
}

impl<'de> Deserialize<'de> for AnyResource {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        // First grab the whole object as a serde_json::Value
        let v = serde_json::Value::deserialize(deserializer)?;

        // We expect an object with a "type" field alongside other fields (id, attributes, etc.)
        let type_str = v
            .get("type")
            .and_then(|t| t.as_str())
            .ok_or_else(|| serde::de::Error::custom("missing or non-string `type` field"))?;

        // Helper to map the value into the typed inner struct
        fn from_value<T: for<'a> Deserialize<'a>, E: serde::de::Error>(
            v: serde_json::Value,
        ) -> Result<T, E> {
            serde_json::from_value(v).map_err(E::custom)
        }

        match type_str {
            "albums" => Ok(AnyResource::Albums(from_value(v)?)),
            "appreciations" => Ok(AnyResource::Appreciations(from_value(v)?)),
            "artistBiographies" => Ok(AnyResource::ArtistBiographies(from_value(v)?)),
            "artistRoles" => Ok(AnyResource::ArtistRoles(from_value(v)?)),
            "artists" => Ok(AnyResource::Artists(from_value(v)?)),
            "artworks" => Ok(AnyResource::Artworks(from_value(v)?)),
            "genres" => Ok(AnyResource::Genres(from_value(v)?)),
            "lyrics" => Ok(AnyResource::Lyrics(from_value(v)?)),
            "playlists" => Ok(AnyResource::Playlists(from_value(v)?)),
            "providers" => Ok(AnyResource::Providers(from_value(v)?)),
            "searchResults" => Ok(AnyResource::SearchResults(from_value(v)?)),
            "searchSuggestions" => Ok(AnyResource::SearchSuggestions(from_value(v)?)),
            "trackFiles" => Ok(AnyResource::TrackFiles(from_value(v)?)),
            "trackManifests" => Ok(AnyResource::TrackManifests(from_value(v)?)),
            "trackSourceFiles" => Ok(AnyResource::TrackSourceFiles(from_value(v)?)),
            "trackStatistics" => Ok(AnyResource::TrackStatistics(from_value(v)?)),
            "tracks" => Ok(AnyResource::Tracks(from_value(v)?)),
            "userCollections" => Ok(AnyResource::UserCollections(from_value(v)?)),
            "userEntitlements" => Ok(AnyResource::UserEntitlements(from_value(v)?)),
            "userRecommendations" => Ok(AnyResource::UserRecommendations(from_value(v)?)),
            "userShares" => Ok(AnyResource::UserShares(from_value(v)?)),
            "users" => Ok(AnyResource::Users(from_value(v)?)),
            "videos" => Ok(AnyResource::Videos(from_value(v)?)),
            other => Err(serde::de::Error::custom(format!(
                "unknown `type` discriminator: {other}"
            ))),
        }
    }
}

impl Serialize for AnyResource {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            AnyResource::Albums(v) => v.serialize(serializer),
            AnyResource::Appreciations(v) => v.serialize(serializer),
            AnyResource::ArtistBiographies(v) => v.serialize(serializer),
            AnyResource::ArtistRoles(v) => v.serialize(serializer),
            AnyResource::Artists(v) => v.serialize(serializer),
            AnyResource::Artworks(v) => v.serialize(serializer),
            AnyResource::Genres(v) => v.serialize(serializer),
            AnyResource::Lyrics(v) => v.serialize(serializer),
            AnyResource::Playlists(v) => v.serialize(serializer),
            AnyResource::Providers(v) => v.serialize(serializer),
            AnyResource::SearchResults(v) => v.serialize(serializer),
            AnyResource::SearchSuggestions(v) => v.serialize(serializer),
            AnyResource::TrackFiles(v) => v.serialize(serializer),
            AnyResource::TrackManifests(v) => v.serialize(serializer),
            AnyResource::TrackSourceFiles(v) => v.serialize(serializer),
            AnyResource::TrackStatistics(v) => v.serialize(serializer),
            AnyResource::Tracks(v) => v.serialize(serializer),
            AnyResource::UserCollections(v) => v.serialize(serializer),
            AnyResource::UserEntitlements(v) => v.serialize(serializer),
            AnyResource::UserRecommendations(v) => v.serialize(serializer),
            AnyResource::UserShares(v) => v.serialize(serializer),
            AnyResource::Users(v) => v.serialize(serializer),
            AnyResource::Videos(v) => v.serialize(serializer),
        }
    }
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Resource<T> {
    #[serde(rename = "data")]
    pub data: T,
    #[serde(rename = "included", skip_serializing_if = "Option::is_none")]
    pub included: Option<Vec<AnyResource>>,
    #[serde(rename = "links")]
    pub links: Links,
}

impl<T> Resource<T> {
    pub fn new(data: T, links: Links) -> Resource<T> {
        Resource {
            data,
            included: None,
            links,
        }
    }
}

// Generic MultiResource struct for all multi resource data documents
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct MultiResource<T> {
    #[serde(rename = "data")]
    pub data: Vec<T>,
    #[serde(rename = "included", skip_serializing_if = "Option::is_none")]
    pub included: Option<Vec<AnyResource>>,
    #[serde(rename = "links")]
    pub links: Links,
}

impl<T> MultiResource<T> {
    pub fn new(data: Vec<T>, links: Links) -> MultiResource<T> {
        MultiResource {
            data,
            included: None,
            links,
        }
    }
}

// Consolidated Relationship struct for all single relationship data documents
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Relationship {
    #[serde(rename = "data", skip_serializing_if = "Option::is_none")]
    pub data: Option<ResourceIdentifier>,
    #[serde(rename = "included", skip_serializing_if = "Option::is_none")]
    pub included: Option<Vec<AnyResource>>,
    #[serde(rename = "links")]
    pub links: Links,
}

impl Relationship {
    pub fn new(links: Links) -> Relationship {
        Relationship {
            data: None,
            included: None,
            links,
        }
    }
}

// Generic MultiRelationship struct for all multi relationship data documents
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct MultiRelationship<T> {
    #[serde(rename = "data", skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<T>>,
    #[serde(rename = "included", skip_serializing_if = "Option::is_none")]
    pub included: Option<Vec<AnyResource>>,
    #[serde(rename = "links")]
    pub links: Links,
}

impl<T> MultiRelationship<T> {
    pub fn new(links: Links) -> MultiRelationship<T> {
        MultiRelationship {
            data: None,
            included: None,
            links,
        }
    }
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct DataWrap<T, M = ()> {
    pub data: T,
    // Optional metadata; omit if None
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub meta: Option<M>,
}

impl<T> DataWrap<T, ()> {
    pub fn new(data: T) -> Self {
        Self { data, meta: None }
    }
}

impl<T, M> DataWrap<T, M> {
    pub fn new_with_meta(data: T, meta: M) -> Self {
        Self {
            data,
            meta: Some(meta),
        }
    }
}

impl<T> From<T> for DataWrap<T, ()> {
    fn from(data: T) -> Self {
        DataWrap::new(data)
    }
}

impl<T, M> From<(T, M)> for DataWrap<T, M> {
    fn from((data, meta): (T, M)) -> Self {
        DataWrap::new_with_meta(data, meta)
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResourceIdentifier<M = ()> {
    /// Resource id
    #[serde(rename = "id")]
    pub id: String,
    /// Resource type
    #[serde(rename = "type")]
    pub r#type: ResourceType,
    #[serde(rename = "meta", skip_serializing_if = "Option::is_none")]
    pub meta: Option<M>,
}

impl ResourceIdentifier<()> {
    pub fn new(id: String, r#type: ResourceType) -> ResourceIdentifier {
        ResourceIdentifier {
            id,
            r#type,
            meta: None,
        }
    }
}

impl<M> ResourceIdentifier<M> {
    pub fn new_with_meta(id: String, r#type: ResourceType, meta: M) -> ResourceIdentifier<M> {
        ResourceIdentifier {
            id,
            r#type,
            meta: Some(meta),
        }
    }
}
