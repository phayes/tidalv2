use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct UserCollection {
    #[serde(rename = "attributes", skip_serializing_if = "Option::is_none")]
    pub attributes: Option<serde_json::Value>,
    /// Resource id
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "relationships", skip_serializing_if = "Option::is_none")]
    pub relationships: Option<models::UserCollectionsRelationships>,
    /// Resource type
    #[serde(rename = "type")]
    pub r#type: String,
}

impl UserCollection {
    pub fn new(id: String, r#type: String) -> UserCollection {
        UserCollection {
            attributes: None,
            id,
            relationships: None,
            r#type,
        }
    }
}
