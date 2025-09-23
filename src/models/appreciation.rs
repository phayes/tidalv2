use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Appreciation {
    #[serde(rename = "attributes", skip_serializing_if = "Option::is_none")]
    pub attributes: Option<AppreciationAttributes>,
    /// Resource id
    #[serde(rename = "id")]
    pub id: String,
    /// Resource type
    #[serde(rename = "type")]
    pub r#type: String,
}

impl Appreciation {
    pub fn new(id: String, r#type: String) -> Appreciation {
        Appreciation {
            attributes: None,
            id,
            r#type,
        }
    }
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct AppreciationAttributes {
    /// Time when the appreciation was created
    #[serde(rename = "createdAt")]
    pub created_at: String,
}

impl AppreciationAttributes {
    pub fn new(created_at: String) -> AppreciationAttributes {
        AppreciationAttributes { created_at }
    }
}
