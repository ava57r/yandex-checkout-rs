use crate::error;
use reqwest;
use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use serde::de::DeserializeOwned;

use crate::config::Configuration;

pub type Response<T> = Result<T, error::Error>;

pub struct ApiClient {
    client: reqwest::Client,
    config: Configuration,
}

impl ApiClient {
    pub fn new(config: Configuration, client: reqwest::Client) -> Self {
        Self { client, config }
    }

    pub fn get<T: DeserializeOwned>(&self, path: &str) -> Response<T> {
        unimplemented!()
    }

    pub fn url(&self, path: &str) -> String {
        format!("{}/{}", self.config.endpoint_api, &path[1..])
    }

    fn headers(&self) -> HeaderMap {
        let mut headers = HeaderMap::new();
        headers.insert(
            reqwest::header::CONTENT_TYPE,
            HeaderValue::from_str("application/json").unwrap(),
        );

        if let Some(auth_token) = self.config.auth_token.as_ref() {
            headers.insert(
                reqwest::header::AUTHORIZATION,
                HeaderValue::from_str(&format!("Bearer {}", auth_token)).unwrap(),
            );
        } else {
            let account_id = self.config.account_id.as_ref().expect("Not set account_id");
            let secret_key = self.config.secret_key.as_ref().expect("Not set secret_key");

            headers.insert(
                reqwest::header::AUTHORIZATION,
                HeaderValue::from_str(&format!("{}:{}", account_id, secret_key)).unwrap(),
            );
        }

        headers
    }
}
