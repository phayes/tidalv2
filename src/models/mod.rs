pub mod albums_items_resource_identifier;
pub use self::albums_items_resource_identifier::AlbumsItemsResourceIdentifier;
pub mod albums_items_resource_identifier_meta;
pub use self::albums_items_resource_identifier_meta::AlbumsItemsResourceIdentifierMeta;
pub mod albums_relationships;
pub use self::albums_relationships::AlbumsRelationships;
pub mod album;
pub use self::album::{Album, AlbumAttributes};
pub mod appreciations_create_operation_meta;
pub use self::appreciations_create_operation_meta::AppreciationsCreateOperationMeta;
pub mod appreciation;
pub use self::appreciation::{Appreciation, AppreciationAttributes};
pub mod artist_biographies_relationships;
pub use self::artist_biographies_relationships::ArtistBiographiesRelationships;
pub mod artist_biography;
pub use self::artist_biography::{ArtistBiography, ArtistBiographyAttributes};
pub mod artist_create_operation_meta;
pub use self::artist_create_operation_meta::ArtistCreateOperationMeta;
pub mod artist_role;
pub use self::artist_role::{ArtistRole, ArtistRoleAttributes};
pub mod artists_followers_resource_meta_viewer_context;
pub use self::artists_followers_resource_meta_viewer_context::ArtistsFollowersResourceMetaViewerContext;
pub mod artists_relationships;
pub use self::artists_relationships::ArtistsRelationships;
pub mod artist;
pub use self::artist::{Artist, ArtistAttributes};
pub mod artists_track_providers_resource_identifier;
pub use self::artists_track_providers_resource_identifier::ArtistsTrackProvidersResourceIdentifier;
pub mod artists_track_providers_resource_identifier_meta;
pub use self::artists_track_providers_resource_identifier_meta::ArtistsTrackProvidersResourceIdentifierMeta;
pub mod artwork_file;
pub use self::artwork_file::ArtworkFile;
pub mod artwork_file_meta;
pub use self::artwork_file_meta::ArtworkFileMeta;
pub mod artwork_source_file;
pub use self::artwork_source_file::ArtworkSourceFile;
pub mod artworks_relationships;
pub use self::artworks_relationships::ArtworksRelationships;
pub mod artwork;
pub use self::artwork::{Artwork, ArtworkAttributes};
pub mod audio_normalization_data;
pub use self::audio_normalization_data::AudioNormalizationData;
pub mod barcode_id;
pub use self::barcode_id::BarcodeId;
pub mod copyright;
pub use self::copyright::Copyright;
pub mod drm_data;
pub use self::drm_data::DrmData;
pub mod error_object;
pub use self::error_object::ErrorObject;
pub mod error_object_source;
pub use self::error_object_source::ErrorObjectSource;
pub mod errors_document;
pub use self::errors_document::ErrorsDocument;
pub mod external_link;
pub use self::external_link::ExternalLink;
pub mod external_link_meta;
pub use self::external_link_meta::ExternalLinkMeta;
pub mod external_link_payload;
pub use self::external_link_payload::ExternalLinkPayload;
pub mod file_status;
pub use self::file_status::FileStatus;
pub mod file_upload_link;
pub use self::file_upload_link::FileUploadLink;
pub mod file_upload_link_meta;
pub use self::file_upload_link_meta::FileUploadLinkMeta;
pub mod genre;
pub use self::genre::{Genre, GenreAttributes};
pub mod links;
pub use self::links::Links;
pub mod links_meta;
pub use self::links_meta::LinksMeta;
pub mod lyrics_relationships;
pub use self::lyrics_relationships::LyricsRelationships;
pub mod lyrics;
pub use self::lyrics::{Lyrics, LyricsAttributes};
pub mod playlist_create_operation_payload;
pub use self::playlist_create_operation_payload::PlaylistCreateOperationPayload;
pub mod playlist_create_operation_payload_data;
pub use self::playlist_create_operation_payload_data::PlaylistCreateOperationPayloadData;
pub mod playlist_create_operation_payload_data_attributes;
pub use self::playlist_create_operation_payload_data_attributes::PlaylistCreateOperationPayloadDataAttributes;
pub mod playlist_items_relationship_add_operation_payload;
pub use self::playlist_items_relationship_add_operation_payload::PlaylistItemsRelationshipAddOperationPayload;
pub mod playlist_items_relationship_add_operation_payload_data;
pub use self::playlist_items_relationship_add_operation_payload_data::PlaylistItemsRelationshipAddOperationPayloadData;
pub mod playlist_items_relationship_add_operation_payload_meta;
pub use self::playlist_items_relationship_add_operation_payload_meta::PlaylistItemsRelationshipAddOperationPayloadMeta;
pub mod playlist_items_relationship_remove_operation_payload;
pub use self::playlist_items_relationship_remove_operation_payload::PlaylistItemsRelationshipRemoveOperationPayload;
pub mod playlist_items_relationship_remove_operation_payload_data;
pub use self::playlist_items_relationship_remove_operation_payload_data::PlaylistItemsRelationshipRemoveOperationPayloadData;
pub mod playlist_items_relationship_remove_operation_payload_data_meta;
pub use self::playlist_items_relationship_remove_operation_payload_data_meta::PlaylistItemsRelationshipRemoveOperationPayloadDataMeta;
pub mod playlist_items_relationship_reorder_operation_payload;
pub use self::playlist_items_relationship_reorder_operation_payload::PlaylistItemsRelationshipReorderOperationPayload;
pub mod playlist_items_relationship_reorder_operation_payload_data;
pub use self::playlist_items_relationship_reorder_operation_payload_data::PlaylistItemsRelationshipReorderOperationPayloadData;
pub mod playlist_items_relationship_reorder_operation_payload_data_meta;
pub use self::playlist_items_relationship_reorder_operation_payload_data_meta::PlaylistItemsRelationshipReorderOperationPayloadDataMeta;
pub mod playlist_items_relationship_reorder_operation_payload_meta;
pub use self::playlist_items_relationship_reorder_operation_payload_meta::PlaylistItemsRelationshipReorderOperationPayloadMeta;
pub mod playlist_update_operation_payload;
pub use self::playlist_update_operation_payload::PlaylistUpdateOperationPayload;
pub mod playlist_update_operation_payload_data;
pub use self::playlist_update_operation_payload_data::PlaylistUpdateOperationPayloadData;
pub mod playlist_update_operation_payload_data_attributes;
pub use self::playlist_update_operation_payload_data_attributes::PlaylistUpdateOperationPayloadDataAttributes;
pub mod playlists_items_resource_identifier;
pub use self::playlists_items_resource_identifier::PlaylistsItemsResourceIdentifier;
pub mod playlists_items_resource_identifier_meta;
pub use self::playlists_items_resource_identifier_meta::PlaylistsItemsResourceIdentifierMeta;
pub mod playlists_relationships;
pub use self::playlists_relationships::PlaylistsRelationships;
pub mod playlist;
pub use self::playlist::{Playlist, PlaylistAttributes};
pub mod provider;
pub use self::provider::{Provider, ProviderAttributes};
pub mod resource_identifier;
pub use self::resource_identifier::{ResourceIdentifier};
pub mod resource_object_object_object;
pub use self::resource_object_object_object::ResourceObjectObjectObject;
pub mod search_results_relationships;
pub use self::search_results_relationships::SearchResultsRelationships;
pub mod search_result;
pub use self::search_result::{SearchResult, SearchResultAttributes};
pub mod search_suggestions_highlights;
pub use self::search_suggestions_highlights::SearchSuggestionsHighlights;
pub mod search_suggestions_history;
pub use self::search_suggestions_history::SearchSuggestionsHistory;
pub mod search_suggestions_relationships;
pub use self::search_suggestions_relationships::SearchSuggestionsRelationships;
pub mod search_suggestion;
pub use self::search_suggestion::{SearchSuggestion, SearchSuggestionAttributes};
pub mod search_suggestions_suggestions;
pub use self::search_suggestions_suggestions::SearchSuggestionsSuggestions;
pub mod track_file;
pub use self::track_file::{TrackFile, TrackFileAttributes};
pub mod track_manifest;
pub use self::track_manifest::{TrackManifest, TrackManifestAttributes};
pub mod track_source_files_relationships;
pub use self::track_source_files_relationships::TrackSourceFilesRelationships;
pub mod track_source_file;
pub use self::track_source_file::{TrackSourceFile, TrackSourceFileAttributes};
pub mod track_statistics_relationships;
pub use self::track_statistics_relationships::TrackStatisticsRelationships;
pub mod track_statistics;
pub use self::track_statistics::{TrackStatistics, TrackStatisticsAttributes};
pub mod tracks_relationships;
pub use self::tracks_relationships::TracksRelationships;
pub mod track;
pub use self::track::{Track, TrackAttributes};
pub mod user_collections_albums_resource_identifier;
pub use self::user_collections_albums_resource_identifier::UserCollectionsAlbumsResourceIdentifier;
pub mod user_collections_albums_resource_identifier_meta;
pub use self::user_collections_albums_resource_identifier_meta::UserCollectionsAlbumsResourceIdentifierMeta;
pub mod user_collections_artists_resource_identifier;
pub use self::user_collections_artists_resource_identifier::UserCollectionsArtistsResourceIdentifier;
pub mod user_collections_artists_resource_identifier_meta;
pub use self::user_collections_artists_resource_identifier_meta::UserCollectionsArtistsResourceIdentifierMeta;
pub mod user_collections_playlists_resource_identifier;
pub use self::user_collections_playlists_resource_identifier::UserCollectionsPlaylistsResourceIdentifier;
pub mod user_collections_playlists_resource_identifier_meta;
pub use self::user_collections_playlists_resource_identifier_meta::UserCollectionsPlaylistsResourceIdentifierMeta;
pub mod user_collections_relationships;
pub use self::user_collections_relationships::UserCollectionsRelationships;
pub mod user_collection;
pub use self::user_collection::UserCollection;
pub mod user_collections_tracks_resource_identifier;
pub use self::user_collections_tracks_resource_identifier::UserCollectionsTracksResourceIdentifier;
pub mod user_collections_tracks_resource_identifier_meta;
pub use self::user_collections_tracks_resource_identifier_meta::UserCollectionsTracksResourceIdentifierMeta;
pub mod user_collections_videos_resource_identifier;
pub use self::user_collections_videos_resource_identifier::UserCollectionsVideosResourceIdentifier;
pub mod user_collections_videos_resource_identifier_meta;
pub use self::user_collections_videos_resource_identifier_meta::UserCollectionsVideosResourceIdentifierMeta;
pub mod user_entitlement;
pub use self::user_entitlement::{UserEntitlement, UserEntitlementAttributes};
pub mod user_recommendations_relationships;
pub use self::user_recommendations_relationships::UserRecommendationsRelationships;
pub mod user_recommendation;
pub use self::user_recommendation::UserRecommendation;
pub mod user_shares_relationships;
pub use self::user_shares_relationships::UserSharesRelationships;
pub mod user_share;
pub use self::user_share::{UserShare, UserShareAttributes};
pub mod user;
pub use self::user::{User, UserAttributes};
pub mod videos_relationships;
pub use self::videos_relationships::VideosRelationships;
pub mod video;
pub use self::video::{Video, VideoAttributes};

// Generic Resource struct for all single resource data documents
use serde::{de::DeserializeOwned, Deserialize, Deserializer, Serialize, Serializer};
use strum::{Display, AsRefStr, IntoStaticStr};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize, Display, AsRefStr, IntoStaticStr)]
#[serde(rename_all = "camelCase")]
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

// AnyResource enum - a fundamental type that can represent any resource in the API
#[derive(Clone, Debug, PartialEq)]
pub enum AnyResource {
    Albums(Album),
    Appreciations(Appreciation),
    ArtistBiographies(ArtistBiography),
    ArtistRoles(ArtistRole),
    Artists(Artist),
    Artworks(Artwork),
    Genres(Genre),
    Lyrics(Lyrics),
    Playlists(Playlist),
    Providers(Provider),
    SearchResults(SearchResult),
    SearchSuggestions(SearchSuggestion),
    TrackFiles(TrackFile),
    TrackManifests(TrackManifest),
    TrackSourceFiles(TrackSourceFile),
    TrackStatistics(TrackStatistics),
    Tracks(Track),
    UserCollections(UserCollection),
    UserEntitlements(UserEntitlement),
    UserRecommendations(UserRecommendation),
    UserShares(UserShare),
    Users(User),
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

impl Into<ResourceType> for AnyResource {
    fn into(self) -> ResourceType {
        self.resource_type()
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
pub struct DataWrap<T> {
    pub data: T,
}

impl<T> DataWrap<T> {
    pub fn new(data: T) -> DataWrap<T> {
        DataWrap { data }
    }
}