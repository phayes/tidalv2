use crate::models::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct TrackFileAttributes {
    #[serde(
        rename = "albumAudioNormalizationData",
        skip_serializing_if = "Option::is_none"
    )]
    pub album_audio_normalization_data: Option<audio::AudioNormalization>,
    /// File's audio format
    #[serde(rename = "format", skip_serializing_if = "Option::is_none")]
    pub format: Option<audio::AudioFormat>,
    #[serde(
        rename = "trackAudioNormalizationData",
        skip_serializing_if = "Option::is_none"
    )]
    pub track_audio_normalization_data: Option<audio::AudioNormalization>,
    /// Track presentation
    #[serde(rename = "trackPresentation", skip_serializing_if = "Option::is_none")]
    pub track_presentation: Option<track_manifest::TrackPresentation>,
    /// File URL
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

impl TrackFileAttributes {
    pub fn new() -> TrackFileAttributes {
        TrackFileAttributes {
            album_audio_normalization_data: None,
            format: None,
            track_audio_normalization_data: None,
            track_presentation: None,
            url: None,
        }
    }
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct TrackFile {
    #[serde(rename = "attributes", default)]
    pub attributes: TrackFileAttributes,
    /// Resource id
    #[serde(rename = "id")]
    pub id: String,
    /// Resource type
    #[serde(rename = "type")]
    pub r#type: String,
}

impl TrackFile {
    pub fn new(id: String, r#type: String) -> TrackFile {
        TrackFile {
            attributes: TrackFileAttributes::default(),
            id,
            r#type,
        }
    }
}
