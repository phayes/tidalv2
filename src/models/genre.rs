use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Genre {
    #[serde(rename = "attributes", default)]
    pub attributes: GenreAttributes,
    /// Resource id
    #[serde(rename = "id")]
    pub id: String,
    /// Resource type
    #[serde(rename = "type")]
    pub r#type: String,
}

impl Genre {
    pub fn new(id: String, r#type: String) -> Genre {
        Genre {
            attributes: GenreAttributes::default(),
            id,
            r#type,
        }
    }
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GenreAttributes {
    /// Genre name
    #[serde(rename = "genreName")]
    pub genre_name: String,
}

impl GenreAttributes {
    pub fn new(genre_name: String) -> GenreAttributes {
        GenreAttributes { genre_name }
    }
}
