use tidalrs::Client;
use crate::apis::configuration::Configuration;

const TIDALV2_BASE_URL: &str = "https://openapi.tidal.com/v2";
const USER_AGENT: &str = "Mozilla/5.0 (Linux; Android 12; wv) AppleWebKit/537.36 (KHTML, like Gecko) Version/4.0 Chrome/91.0.4472.114 Safari/537.36";

impl Client {
    pub fn tidalv2_config(&self) -> Configuration {
        let country_code = self.get_country_code();
        let authz = self.get_authz();
        Configuration {
            base_path: TIDALV2_BASE_URL.to_string(),
            user_agent: Some(USER_AGENT.to_string()),
            client: self.client.clone(),
            bearer_access_token: Some(authz.access_token),
            country_code: country_code,
        }
    }
}