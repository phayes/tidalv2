use crate::models::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct UserShare {
    #[serde(rename = "attributes", skip_serializing_if = "Option::is_none")]
    pub attributes: Option<UserShareAttributes>,
    /// Resource id
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "relationships", skip_serializing_if = "Option::is_none")]
    pub relationships: Option<UserSharesRelationships>,
    /// Resource type
    #[serde(rename = "type")]
    pub r#type: String,
}

impl UserShare {
    pub fn new(id: String, r#type: String) -> UserShare {
        UserShare {
            attributes: None,
            id,
            relationships: None,
            r#type,
        }
    }
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct UserShareAttributes {
    /// Share code
    #[serde(rename = "code")]
    pub code: String,
    /// Datetime of userShare creation (ISO 8601)
    #[serde(rename = "createdAt")]
    pub created_at: String,
    /// Links external to TIDAL API
    #[serde(rename = "externalLinks", skip_serializing_if = "Option::is_none")]
    pub external_links: Option<Vec<ExternalLink>>,
}

impl UserShareAttributes {
    pub fn new(code: String, created_at: String) -> UserShareAttributes {
        UserShareAttributes {
            code,
            created_at,
            external_links: None,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UserSharesRelationships {
    #[serde(rename = "owners")]
    pub owners: MultiRelationship<ResourceIdentifier>,
    #[serde(rename = "sharedResources")]
    pub shared_resources: MultiRelationship<ResourceIdentifier>,
}

impl UserSharesRelationships {
    pub fn new(
        owners: MultiRelationship<ResourceIdentifier>,
        shared_resources: MultiRelationship<ResourceIdentifier>,
    ) -> UserSharesRelationships {
        UserSharesRelationships {
            owners,
            shared_resources,
        }
    }
}
