use crate::models::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct SearchSuggestion {
    #[serde(rename = "attributes", default)]
    pub attributes: SearchSuggestionAttributes,
    /// Resource id
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "relationships", skip_serializing_if = "Option::is_none")]
    pub relationships: Option<SearchSuggestionsRelationships>,
    /// Resource type
    #[serde(rename = "type")]
    pub r#type: String,
}

impl SearchSuggestion {
    pub fn new(id: String, r#type: String) -> SearchSuggestion {
        SearchSuggestion {
            attributes: SearchSuggestionAttributes::default(),
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
    pub history: Option<Vec<SearchSuggestionsHistory>>,
    /// Suggested search queries
    #[serde(rename = "suggestions", skip_serializing_if = "Option::is_none")]
    pub suggestions: Option<Vec<SearchSuggestionsSuggestions>>,
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

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SearchSuggestionsRelationships {
    #[serde(rename = "directHits")]
    pub direct_hits: MultiRelationship<ResourceIdentifier>,
}

impl SearchSuggestionsRelationships {
    pub fn new(
        direct_hits: MultiRelationship<ResourceIdentifier>,
    ) -> SearchSuggestionsRelationships {
        SearchSuggestionsRelationships { direct_hits }
    }
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct SearchSuggestionsHighlights {
    #[serde(rename = "length")]
    pub length: i32,
    #[serde(rename = "start")]
    pub start: i32,
}

impl SearchSuggestionsHighlights {
    pub fn new(length: i32, start: i32) -> SearchSuggestionsHighlights {
        SearchSuggestionsHighlights { length, start }
    }
}

/// SearchSuggestionsHistory : Suggestions from search history
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct SearchSuggestionsHistory {
    #[serde(rename = "highlights", skip_serializing_if = "Option::is_none")]
    pub highlights: Option<Vec<SearchSuggestionsHighlights>>,
    #[serde(rename = "query")]
    pub query: String,
}

impl SearchSuggestionsHistory {
    /// Suggestions from search history
    pub fn new(query: String) -> SearchSuggestionsHistory {
        SearchSuggestionsHistory {
            highlights: None,
            query,
        }
    }
}

/// SearchSuggestionsSuggestions : Suggested search queries
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct SearchSuggestionsSuggestions {
    #[serde(rename = "highlights", skip_serializing_if = "Option::is_none")]
    pub highlights: Option<Vec<SearchSuggestionsHighlights>>,
    #[serde(rename = "query")]
    pub query: String,
}

impl SearchSuggestionsSuggestions {
    /// Suggested search queries
    pub fn new(query: String) -> SearchSuggestionsSuggestions {
        SearchSuggestionsSuggestions {
            highlights: None,
            query,
        }
    }
}
