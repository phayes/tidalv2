use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct SearchResult {
    #[serde(rename = "attributes", skip_serializing_if = "Option::is_none")]
    pub attributes: Option<SearchResultAttributes>,
    /// Resource id
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "relationships", skip_serializing_if = "Option::is_none")]
    pub relationships: Option<models::SearchResultsRelationships>,
    /// Resource type
    #[serde(rename = "type")]
    pub r#type: String,
}

impl SearchResult {
    pub fn new(id: String, r#type: String) -> SearchResult {
        SearchResult {
            attributes: None,
            id,
            relationships: None,
            r#type,
        }
    }
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct SearchResultAttributes {
    /// 'did you mean' prompt
    #[serde(rename = "didYouMean", skip_serializing_if = "Option::is_none")]
    pub did_you_mean: Option<String>,
    /// search request unique tracking number
    #[serde(rename = "trackingId")]
    pub tracking_id: String,
}

impl SearchResultAttributes {
    pub fn new(tracking_id: String) -> SearchResultAttributes {
        SearchResultAttributes {
            did_you_mean: None,
            tracking_id,
        }
    }
}
