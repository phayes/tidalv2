// Resource modules
pub mod album;
pub mod artist;
pub mod artist_biography;
pub mod artist_role;
pub mod artwork;
pub mod artwork_file;
pub mod audio;
pub mod barcode_id;
pub mod copyright;
pub mod drm_data;
pub mod genre;
pub mod lyrics;
pub mod playlist;
pub mod provider;
pub mod track;
pub mod user_collection;

// Shared modules that are brought into shared scope
mod resource;
pub use resource::*;
mod links;
pub use self::links::*;
mod error_object;
pub use self::error_object::*;
mod error_object_source;
pub use self::error_object_source::*;
mod errors_document;
pub use self::errors_document::*;
mod external_link;
pub use self::external_link::*;

// TODO: Modules that need to be moved to the resource modules
mod user;
pub use self::user::{User, UserAttributes};
mod video;
pub use self::video::{Video, VideoAttributes, VideoAvailability, VideoRelationships};
mod user_share;
pub use self::user_share::{UserShare, UserShareAttributes};
mod user_recommendation;
pub use self::user_recommendation::UserRecommendation;
mod track_manifest;
pub use self::track_manifest::{TrackManifest, TrackManifestAttributes, TrackPresentation};
mod track_statistics;
pub use self::track_statistics::{TrackStatistics, TrackStatisticsAttributes};
mod track_file;
pub use self::track_file::{
    TrackFile, TrackFileAttributes,
};



/// Search Related (TODO)
mod search_results_relationships;
pub use self::search_results_relationships::SearchResultsRelationships;
mod search_result;
pub use self::search_result::{SearchResult, SearchResultAttributes};
mod search_suggestions_highlights;
pub use self::search_suggestions_highlights::SearchSuggestionsHighlights;
mod search_suggestions_history;
pub use self::search_suggestions_history::SearchSuggestionsHistory;
mod search_suggestions_relationships;
pub use self::search_suggestions_relationships::SearchSuggestionsRelationships;
mod search_suggestion;
pub use self::search_suggestion::{SearchSuggestion, SearchSuggestionAttributes};
mod search_suggestions_suggestions;
pub use self::search_suggestions_suggestions::SearchSuggestionsSuggestions;
