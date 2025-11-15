use crate::models::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct User {
    #[serde(rename = "attributes", default)]
    pub attributes: UserAttributes,
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
            attributes: UserAttributes::default(),
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

/// Represents a tidal user as returned by the authroization endpoint.
/// This is used to identify the authenticated user.
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AuthzUser {
    /// Whether the user has accepted the End User License Agreement
    #[serde(rename = "acceptedEULA")]
    pub accepted_eula: bool,
    /// Whether an account link has been created
    pub account_link_created: bool,
    /// User's address (if provided)
    pub address: Option<String>,
    /// Apple ID associated with the account (if any)
    pub apple_uid: Option<String>,
    /// Channel ID associated with the user
    pub channel_id: u64,
    /// User's city (if provided)
    pub city: Option<String>,
    /// User's country code (e.g., "US", "GB")
    pub country_code: String,
    /// Unix timestamp when the account was created
    pub created: u64,
    /// User's email address
    pub email: String,
    /// Whether the email address has been verified
    pub email_verified: bool,
    /// Facebook UID associated with the account (if any)
    pub facebook_uid: Option<u64>,
    /// User's first name (if provided)
    pub first_name: Option<String>,
    /// User's full name (if provided)
    pub full_name: Option<String>,
    /// Google UID associated with the account
    pub google_uid: String,
    /// User's last name (if provided)
    pub last_name: Option<String>,
    /// Whether this is a new user account
    pub new_user: bool,
    /// User's nickname (if provided)
    pub nickname: Option<String>,
    /// Parent ID associated with the user
    pub parent_id: u64,
    /// User's phone number (if provided)
    pub phone_number: Option<String>,
    /// User's postal code (if provided)
    pub postalcode: Option<String>,
    /// Unix timestamp when the account was last updated
    pub updated: u64,
    /// User's US state (if provided and in US)
    pub us_state: Option<String>,
    /// Unique user ID
    pub user_id: u64,
    /// User's username
    pub username: String,
}
