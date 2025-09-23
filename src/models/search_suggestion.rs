use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct SearchSuggestion {
    #[serde(rename = "attributes", skip_serializing_if = "Option::is_none")]
    pub attributes: Option<SearchSuggestionAttributes>,
    /// Resource id
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "relationships", skip_serializing_if = "Option::is_none")]
    pub relationships: Option<models::SearchSuggestionsRelationships>,
    /// Resource type
    #[serde(rename = "type")]
    pub r#type: String,
}

impl SearchSuggestion {
    pub fn new(id: String, r#type: String) -> SearchSuggestion {
        SearchSuggestion {
            attributes: None,
            id,
            relationships: None,
            r#type,
        }
    }
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct SearchSuggestionAttributes {
    /// Suggestions from search history
    #[serde(rename = "history", skip_serializing_if = "Option::is_none")]
    pub history: Option<Vec<models::SearchSuggestionsHistory>>,
    /// Suggested search queries
    #[serde(rename = "suggestions", skip_serializing_if = "Option::is_none")]
    pub suggestions: Option<Vec<models::SearchSuggestionsSuggestions>>,
    /// Unique tracking id
    #[serde(rename = "trackingId")]
    pub tracking_id: String,
}

impl SearchSuggestionAttributes {
    pub fn new(tracking_id: String) -> SearchSuggestionAttributes {
        SearchSuggestionAttributes {
            history: None,
            suggestions: None,
            tracking_id,
        }
    }
}
