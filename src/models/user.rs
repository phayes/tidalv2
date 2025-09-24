use crate::models::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct User {
    #[serde(rename = "attributes", skip_serializing_if = "Option::is_none")]
    pub attributes: Option<UserAttributes>,
    /// Resource id
    #[serde(rename = "id")]
    pub id: String,
    /// Resource type
    #[serde(rename = "type")]
    pub r#type: String,
}

impl User {
    pub fn new(id: String, r#type: String) -> User {
        User {
            attributes: None,
            id,
            r#type,
        }
    }
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct UserAttributes {
    /// ISO 3166-1 alpha-2 country code
    #[serde(rename = "country")]
    pub country: String,
    /// email address
    #[serde(rename = "email", skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// Is the email verified
    #[serde(rename = "emailVerified", skip_serializing_if = "Option::is_none")]
    pub email_verified: Option<bool>,
    /// Users first name
    #[serde(rename = "firstName", skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    /// Users last name
    #[serde(rename = "lastName", skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    /// Users nostr public key
    #[serde(rename = "nostrPublicKey", skip_serializing_if = "Option::is_none")]
    pub nostr_public_key: Option<String>,
    /// user name
    #[serde(rename = "username")]
    pub username: String,
}

impl UserAttributes {
    pub fn new(country: String, username: String) -> UserAttributes {
        UserAttributes {
            country,
            email: None,
            email_verified: None,
            first_name: None,
            last_name: None,
            nostr_public_key: None,
            username,
        }
    }
}
