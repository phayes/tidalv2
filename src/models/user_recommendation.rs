use crate::models::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct UserRecommendation {
    #[serde(rename = "attributes", default)]
    pub attributes: serde_json::Value,
    /// Resource id
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "relationships", skip_serializing_if = "Option::is_none")]
    pub relationships: Option<UserRecommendationsRelationships>,
    /// Resource type
    #[serde(rename = "type")]
    pub r#type: String,
}

impl UserRecommendation {
    pub fn new(id: String, r#type: String) -> UserRecommendation {
        UserRecommendation {
            attributes: serde_json::Value::default(),
            id,
            relationships: None,
            r#type,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UserRecommendationsRelationships {
    #[serde(rename = "discoveryMixes")]
    pub discovery_mixes: MultiRelationship<ResourceIdentifier>,
    #[serde(rename = "myMixes")]
    pub my_mixes: MultiRelationship<ResourceIdentifier>,
    #[serde(rename = "newArrivalMixes")]
    pub new_arrival_mixes: MultiRelationship<ResourceIdentifier>,
}

impl UserRecommendationsRelationships {
    pub fn new(
        discovery_mixes: MultiRelationship<ResourceIdentifier>,
        my_mixes: MultiRelationship<ResourceIdentifier>,
        new_arrival_mixes: MultiRelationship<ResourceIdentifier>,
    ) -> UserRecommendationsRelationships {
        UserRecommendationsRelationships {
            discovery_mixes,
            my_mixes,
            new_arrival_mixes,
        }
    }
}
