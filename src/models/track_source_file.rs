use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct TrackSourceFileAttributes {
    /// MD5 hash of file to be uploaded
    #[serde(rename = "md5Hash")]
    pub md5_hash: String,
    /// File size of the track in bytes
    #[serde(rename = "size")]
    pub size: i64,
    #[serde(rename = "status")]
    pub status: models::FileStatus,
    #[serde(rename = "uploadLink")]
    pub upload_link: models::FileUploadLink,
}

impl TrackSourceFileAttributes {
    pub fn new(
        md5_hash: String,
        size: i64,
        status: models::FileStatus,
        upload_link: models::FileUploadLink,
    ) -> TrackSourceFileAttributes {
        TrackSourceFileAttributes {
            md5_hash,
            size,
            status,
            upload_link,
        }
    }
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct TrackSourceFile {
    #[serde(rename = "attributes", skip_serializing_if = "Option::is_none")]
    pub attributes: Option<TrackSourceFileAttributes>,
    /// Resource id
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "relationships", skip_serializing_if = "Option::is_none")]
    pub relationships: Option<models::TrackSourceFilesRelationships>,
    /// Resource type
    #[serde(rename = "type")]
    pub r#type: String,
}

impl TrackSourceFile {
    pub fn new(id: String, r#type: String) -> TrackSourceFile {
        TrackSourceFile {
            attributes: None,
            id,
            relationships: None,
            r#type,
        }
    }
}
