use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Configuration {
    pub endpoint_api: String,
    pub account_id: Option<String>,
    pub secret_key: Option<String>,
    pub auth_token: Option<String>,
}

impl Configuration {
    pub const ENDPOINT_API: &'static str = "https://payment.yandex.net/api/v3";
}

impl Default for Configuration {
    fn default() -> Self {
        Self {
            endpoint_api: Self::ENDPOINT_API.to_string(),
            account_id: None,
            secret_key: None,
            auth_token: None,
        }
    }
}
