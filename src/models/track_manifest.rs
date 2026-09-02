use crate::models::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct TrackManifestAttributes {
    #[serde(
        rename = "albumAudioNormalizationData",
        skip_serializing_if = "Option::is_none"
    )]
    pub album_audio_normalization_data: Option<audio::AudioNormalization>,
    #[serde(rename = "drmData", skip_serializing_if = "Option::is_none")]
    pub drm_data: Option<drm_data::DrmData>,
    /// Formats present in manifest
    #[serde(rename = "formats", skip_serializing_if = "Option::is_none")]
    pub formats: Option<Vec<audio::AudioFormat>>,
    /// Unique manifest hash
    #[serde(rename = "hash", skip_serializing_if = "Option::is_none")]
    pub hash: Option<String>,
    #[serde(
        rename = "trackAudioNormalizationData",
        skip_serializing_if = "Option::is_none"
    )]
    pub track_audio_normalization_data: Option<audio::AudioNormalization>,
    /// Track presentation
    #[serde(rename = "trackPresentation", skip_serializing_if = "Option::is_none")]
    pub track_presentation: Option<TrackPresentation>,
    /// Manifest URI
    #[serde(rename = "uri", skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
}

impl TrackManifestAttributes {
    pub fn new() -> TrackManifestAttributes {
        TrackManifestAttributes {
            album_audio_normalization_data: None,
            drm_data: None,
            formats: None,
            hash: None,
            track_audio_normalization_data: None,
            track_presentation: None,
            uri: None,
        }
    }
}

/// Track presentation
#[derive(
    Clone, Copy, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize,
)]
pub enum TrackPresentation {
    #[serde(rename = "FULL")]
    #[default]
    Full,
    #[serde(rename = "PREVIEW")]
    Preview,
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct TrackManifest {
    #[serde(rename = "attributes", default)]
    pub attributes: TrackManifestAttributes,
    /// Resource id
    #[serde(rename = "id")]
    pub id: String,
    /// Resource type
    #[serde(rename = "type")]
    pub r#type: String,
}

impl TrackManifest {
    pub fn new(id: String, r#type: String) -> TrackManifest {
        TrackManifest {
            attributes: TrackManifestAttributes::default(),
            id,
            r#type,
        }
    }
}
