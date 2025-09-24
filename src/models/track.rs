use crate::models::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Track {
    #[serde(rename = "attributes", skip_serializing_if = "Option::is_none")]
    pub attributes: Option<TrackAttributes>,
    /// Resource id
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "relationships", skip_serializing_if = "Option::is_none")]
    pub relationships: Option<TracksRelationships>,
    /// Resource type
    #[serde(rename = "type")]
    pub r#type: String,
}

impl Track {
    pub fn new(id: String, r#type: String) -> Track {
        Track {
            attributes: None,
            id,
            relationships: None,
            r#type,
        }
    }
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct TrackAttributes {
    /// Access type
    #[serde(rename = "accessType", skip_serializing_if = "Option::is_none")]
    pub access_type: Option<TrackAccess>,
    /// Available usage for this track
    #[serde(rename = "availability", skip_serializing_if = "Option::is_none")]
    pub availability: Option<Vec<TrackAvailability>>,
    /// Beats per minute
    #[serde(rename = "bpm", skip_serializing_if = "Option::is_none")]
    pub bpm: Option<f32>,
    #[serde(rename = "copyright", skip_serializing_if = "Option::is_none")]
    pub copyright: Option<copyright::Copyright>,
    /// Datetime of track creation (ISO 8601)
    #[serde(rename = "createdAt", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    /// Duration (ISO 8601)
    #[serde(rename = "duration")]
    pub duration: String,
    /// Explicit content
    #[serde(rename = "explicit")]
    pub explicit: bool,
    /// Track links external to TIDAL API
    #[serde(rename = "externalLinks", skip_serializing_if = "Option::is_none")]
    pub external_links: Option<Vec<ExternalLink>>,
    /// International Standard Recording Code (ISRC)
    #[serde(rename = "isrc")]
    pub isrc: String,
    /// Key
    #[serde(rename = "key")]
    pub key: TrackKey,
    /// The scale of the key
    #[serde(rename = "keyScale")]
    pub key_scale: TrackKeyScale,
    #[serde(rename = "mediaTags")]
    pub media_tags: Vec<String>,
    /// Popularity (0.0 - 1.0)
    #[serde(rename = "popularity")]
    pub popularity: f64,
    /// Is the track spotlighted?
    #[serde(rename = "spotlighted", skip_serializing_if = "Option::is_none")]
    pub spotlighted: Option<bool>,
    /// Track title
    #[serde(rename = "title")]
    pub title: String,
    #[serde(rename = "toneTags", skip_serializing_if = "Option::is_none")]
    pub tone_tags: Option<Vec<String>>,
    /// Track version, complements title
    #[serde(rename = "version", skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

impl TrackAttributes {
    pub fn new(
        duration: String,
        explicit: bool,
        isrc: String,
        key: TrackKey,
        key_scale: TrackKeyScale,
        media_tags: Vec<String>,
        popularity: f64,
        title: String,
    ) -> TrackAttributes {
        TrackAttributes {
            access_type: None,
            availability: None,
            bpm: None,
            copyright: None,
            created_at: None,
            duration,
            explicit,
            external_links: None,
            isrc,
            key,
            key_scale,
            media_tags,
            popularity,
            spotlighted: None,
            title,
            tone_tags: None,
            version: None,
        }
    }
}

/// Access type
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum TrackAccess {
    #[serde(rename = "PUBLIC")]
    Public,
    #[serde(rename = "UNLISTED")]
    Unlisted,
    #[serde(rename = "PRIVATE")]
    Private,
}

impl Default for TrackAccess {
    fn default() -> TrackAccess {
        Self::Public
    }
}

/// Available usage for this track
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum TrackAvailability {
    #[serde(rename = "STREAM")]
    Stream,
    #[serde(rename = "DJ")]
    Dj,
    #[serde(rename = "STEM")]
    Stem,
}

impl Default for TrackAvailability {
    fn default() -> TrackAvailability {
        Self::Stream
    }
}

/// Key
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum TrackKey {
    #[serde(rename = "UNKNOWN")]
    Unknown,
    #[serde(rename = "C")]
    C,
    #[serde(rename = "CSharp")]
    CSharp,
    #[serde(rename = "D")]
    D,
    #[serde(rename = "Eb")]
    Eb,
    #[serde(rename = "E")]
    E,
    #[serde(rename = "F")]
    F,
    #[serde(rename = "FSharp")]
    FSharp,
    #[serde(rename = "G")]
    G,
    #[serde(rename = "Ab")]
    Ab,
    #[serde(rename = "A")]
    A,
    #[serde(rename = "Bb")]
    Bb,
    #[serde(rename = "B")]
    B,
}

impl Default for TrackKey {
    fn default() -> TrackKey {
        Self::Unknown
    }
}

/// The scale of the key
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum TrackKeyScale {
    #[serde(rename = "UNKNOWN")]
    Unknown,
    #[serde(rename = "MAJOR")]
    Major,
    #[serde(rename = "MINOR")]
    Minor,
    #[serde(rename = "AEOLIAN")]
    Aeolian,
    #[serde(rename = "BLUES")]
    Blues,
    #[serde(rename = "DORIAN")]
    Dorian,
    #[serde(rename = "HARMONIC_MINOR")]
    HarmonicMinor,
    #[serde(rename = "LOCRIAN")]
    Locrian,
    #[serde(rename = "LYDIAN")]
    Lydian,
    #[serde(rename = "MIXOLYDIAN")]
    Mixolydian,
    #[serde(rename = "PENTATONIC_MAJOR")]
    PentatonicMajor,
    #[serde(rename = "PHRYGIAN")]
    Phrygian,
    #[serde(rename = "MELODIC_MINOR")]
    MelodicMinor,
    #[serde(rename = "PENTATONIC_MINOR")]
    PentatonicMinor,
}

impl Default for TrackKeyScale {
    fn default() -> TrackKeyScale {
        Self::Unknown
    }
}


#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TracksRelationships {
    #[serde(rename = "albums")]
    pub albums: MultiRelationship<ResourceIdentifier>,
    #[serde(rename = "artists")]
    pub artists: MultiRelationship<ResourceIdentifier>,
    #[serde(rename = "genres")]
    pub genres: MultiRelationship<ResourceIdentifier>,
    #[serde(rename = "lyrics")]
    pub lyrics: MultiRelationship<ResourceIdentifier>,
    #[serde(rename = "owners")]
    pub owners: MultiRelationship<ResourceIdentifier>,
    #[serde(rename = "providers")]
    pub providers: MultiRelationship<ResourceIdentifier>,
    #[serde(rename = "radio")]
    pub radio: MultiRelationship<ResourceIdentifier>,
    #[serde(rename = "similarTracks")]
    pub similar_tracks: MultiRelationship<ResourceIdentifier>,
    #[serde(rename = "sourceFile")]
    pub source_file: Relationship,
    #[serde(rename = "trackStatistics")]
    pub track_statistics: Relationship,
}

impl TracksRelationships {
    pub fn new(
        albums: MultiRelationship<ResourceIdentifier>,
        artists: MultiRelationship<ResourceIdentifier>,
        genres: MultiRelationship<ResourceIdentifier>,
        lyrics: MultiRelationship<ResourceIdentifier>,
        owners: MultiRelationship<ResourceIdentifier>,
        providers: MultiRelationship<ResourceIdentifier>,
        radio: MultiRelationship<ResourceIdentifier>,
        similar_tracks: MultiRelationship<ResourceIdentifier>,
        source_file: Relationship,
        track_statistics: Relationship,
    ) -> TracksRelationships {
        TracksRelationships {
            albums,
            artists,
            genres,
            lyrics,
            owners,
            providers,
            radio,
            similar_tracks,
            source_file,
            track_statistics,
        }
    }
}
