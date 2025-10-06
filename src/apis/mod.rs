mod error;
pub use error::*;

// Common API Error Types - these replace the duplicated error enums across all API files
use crate::models;
use serde::{Deserialize, Serialize};

pub(crate) fn urlencode<T: AsRef<str>>(s: T) -> String {
    ::url::form_urlencoded::byte_serialize(s.as_ref().as_bytes()).collect()
}

pub mod albums_api;
pub mod artist_roles_api;
pub mod artists_api;
pub mod artworks_api;
pub mod playlists_api;
pub mod providers_api;
pub mod search_results_api;
pub mod search_suggestions_api;
pub mod track_files_api;
pub mod track_manifests_api;
pub mod tracks_api;
pub mod user_collections_api;
pub mod user_recommendations_api;
pub mod users_api;
pub mod videos_api;
pub mod configuration;
