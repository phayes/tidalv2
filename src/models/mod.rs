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
pub mod search_result;
pub mod search_suggestions;
pub mod track;
pub mod track_file;
pub mod track_manifest;
pub mod track_statistics;
pub mod user;
pub mod user_collection;
pub mod user_recommendation;
pub mod user_share;
pub mod video;

// Shared modules that are brought into shared scope
mod resource;
pub use resource::*;
mod links;
pub use self::links::*;
mod external_link;
pub use self::external_link::*;
