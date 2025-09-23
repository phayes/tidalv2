use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Provider {
    #[serde(rename = "attributes", skip_serializing_if = "Option::is_none")]
    pub attributes: Option<ProviderAttributes>,
    /// Resource id
    #[serde(rename = "id")]
    pub id: String,
    /// Resource type
    #[serde(rename = "type")]
    pub r#type: String,
}

impl Provider {
    pub fn new(id: String, r#type: String) -> Provider {
        Provider {
            attributes: None,
            id,
            r#type,
        }
    }
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProviderAttributes {
    /// Provider name
    #[serde(rename = "name")]
    pub name: String,
}

impl ProviderAttributes {
    pub fn new(name: String) -> ProviderAttributes {
        ProviderAttributes { name }
    }
}
