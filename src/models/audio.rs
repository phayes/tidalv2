use serde::{Deserialize, Serialize};

/// Audio Formats
#[derive(
    Clone, Copy, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize,
)]
pub enum AudioFormat {
    #[serde(rename = "HEAACV1")]
    #[default]
    Heaacv1,
    #[serde(rename = "AACLC")]
    Aaclc,
    #[serde(rename = "FLAC")]
    Flac,
    #[serde(rename = "FLAC_HIRES")]
    FlacHires,
}

impl AudioFormat {
    pub fn is_lossless(&self) -> bool {
        matches!(self, AudioFormat::Flac | AudioFormat::FlacHires)
    }
}

/// AudioNormalizationData : Track normalization data
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct AudioNormalization {
    #[serde(rename = "peakAmplitude", skip_serializing_if = "Option::is_none")]
    pub peak_amplitude: Option<f32>,
    #[serde(rename = "replayGain", skip_serializing_if = "Option::is_none")]
    pub replay_gain: Option<f32>,
}

impl AudioNormalization {
    /// Track normalization data
    pub fn new() -> AudioNormalization {
        AudioNormalization {
            peak_amplitude: None,
            replay_gain: None,
        }
    }
}
