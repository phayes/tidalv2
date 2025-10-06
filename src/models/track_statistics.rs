use crate::models::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct TrackStatistics {
    #[serde(rename = "attributes", default)]
    pub attributes: TrackStatisticsAttributes,
    /// Resource id
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "relationships", skip_serializing_if = "Option::is_none")]
    pub relationships: Option<TrackStatisticsRelationships>,
    /// Resource type
    #[serde(rename = "type")]
    pub r#type: String,
}

impl TrackStatistics {
    pub fn new(id: String, r#type: String) -> TrackStatistics {
        TrackStatistics {
            attributes: TrackStatisticsAttributes::default(),
            id,
            relationships: None,
            r#type,
        }
    }
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct TrackStatisticsAttributes {
    /// Total playbacks
    #[serde(rename = "totalPlaybacks")]
    pub total_playbacks: i32,
    /// Unique listeners
    #[serde(rename = "uniqueListeners")]
    pub unique_listeners: i32,
}

impl TrackStatisticsAttributes {
    pub fn new(total_playbacks: i32, unique_listeners: i32) -> TrackStatisticsAttributes {
        TrackStatisticsAttributes {
            total_playbacks,
            unique_listeners,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TrackStatisticsRelationships {
    #[serde(rename = "owners")]
    pub owners: MultiRelationship<ResourceIdentifier>,
}

impl TrackStatisticsRelationships {
    pub fn new(owners: MultiRelationship<ResourceIdentifier>) -> TrackStatisticsRelationships {
        TrackStatisticsRelationships { owners }
    }
}
