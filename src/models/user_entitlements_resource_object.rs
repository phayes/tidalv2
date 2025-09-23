use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct UserEntitlementAttributes {
    /// entitlements for user
    #[serde(rename = "entitlements")]
    pub entitlements: Vec<EntitlementsFalse>,
}

impl UserEntitlementAttributes {
    pub fn new(entitlements: Vec<EntitlementsFalse>) -> UserEntitlementAttributes {
        UserEntitlementAttributes { entitlements }
    }
}

/// entitlements for user
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum EntitlementsFalse {
    #[serde(rename = "MUSIC")]
    Music,
    #[serde(rename = "DJ")]
    Dj,
}

impl Default for EntitlementsFalse {
    fn default() -> EntitlementsFalse {
        Self::Music
    }
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct UserEntitlement {
    #[serde(rename = "attributes", skip_serializing_if = "Option::is_none")]
    pub attributes: Option<UserEntitlementAttributes>,
    /// Resource id
    #[serde(rename = "id")]
    pub id: String,
    /// Resource type
    #[serde(rename = "type")]
    pub r#type: String,
}

impl UserEntitlement {
    pub fn new(id: String, r#type: String) -> UserEntitlement {
        UserEntitlement {
            attributes: None,
            id,
            r#type,
        }
    }
}
