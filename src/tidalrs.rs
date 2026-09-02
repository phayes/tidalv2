//! Interoperability with the `tidalrs` v1 API client.
//!
//! Enable the `tidalrs` Cargo feature to convert an authenticated
//! [`tidalrs::TidalClient`] into a [`crate::TidalClient`]. The resulting client
//! copies the country code and current authorization credentials. The HTTP
//! clients are not shared: `tidalrs` still uses reqwest 0.12 while this crate
//! uses reqwest 0.13.

use crate::client::Authz;

/// Creates a v2 client from a `tidalrs` client.
///
/// The v2 client receives its own copy of the current authorization state.
/// Subsequent credential changes are not synchronized between the two clients.
/// Use the same TIDAL client ID that was used to construct the `tidalrs` client
/// so that the v2 client can refresh the copied credentials.
///
/// # Example
///
/// ```
/// use tidalv2::tidalrs::TidalV2ClientExt;
///
/// let v1_client = tidalrs::TidalClient::new("client_id".to_string());
/// let v2_client = v1_client.tidalv2_client("client_id");
///
/// assert_eq!(v2_client.get_country_code(), "US");
/// ```
pub trait TidalV2ClientExt {
    /// Builds a v2 client using this client's country and authorization state.
    fn tidalv2_client(&self, client_id: impl Into<String>) -> crate::TidalClient;
}

impl TidalV2ClientExt for ::tidalrs::TidalClient {
    fn tidalv2_client(&self, client_id: impl Into<String>) -> crate::TidalClient {
        // TODO: Share the HTTP client once `tidalrs` uses the same reqwest version.
        let mut client = crate::TidalClient::new(client_id.into())
            .with_country_code(self.get_country_code());

        if let Some(authz) = self.get_authz() {
            client = client.with_authz(Authz {
                access_token: authz.access_token.clone(),
                refresh_token: Some(authz.refresh_token.clone()),
                user_id: Some(authz.user_id),
                country_code: authz.country_code.clone(),
                expires_timestamp: None,
            });
        }

        client
    }
}

#[cfg(test)]
mod tests {
    use super::TidalV2ClientExt;

    #[test]
    fn copies_authorization_into_v2_client() {
        let authz = ::tidalrs::Authz::new(
            "access-token".to_string(),
            "refresh-token".to_string(),
            42,
            Some("GB".to_string()),
        );
        let v1_client = ::tidalrs::TidalClient::new("client-id".to_string()).with_authz(authz);

        let v2_client = v1_client.tidalv2_client("client-id");
        let v2_authz = v2_client
            .get_authz()
            .expect("authorization should be copied");

        assert_eq!(v2_client.get_country_code(), "GB");
        assert_eq!(v2_authz.access_token, "access-token");
        assert_eq!(v2_authz.refresh_token.as_deref(), Some("refresh-token"));
        assert_eq!(v2_authz.user_id, Some(42));
        assert_eq!(v2_authz.country_code.as_deref(), Some("GB"));
        assert_eq!(v2_authz.expires_timestamp, None);
    }
}
