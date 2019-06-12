use std::collections::HashMap;
use std::io::Read;

use crate::error::{Error, RequestError};
use reqwest;
use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use reqwest::RequestBuilder;
use serde::de::DeserializeOwned;
use serde_json;

use crate::config::Configuration;

const FOUR_KB: usize = 4096;

pub type Response<T> = Result<T, Error>;

pub struct ApiClient {
    client: reqwest::Client,
    config: Configuration,
}

impl ApiClient {
    pub fn new(config: Configuration, client: reqwest::Client) -> Self {
        Self { client, config }
    }

    pub fn get<T: DeserializeOwned>(&self, path: &str) -> Response<T> {
        let url = self.url(path);
        let request = self.client.get(&url).headers(self.headers());

        handle_request(request)
    }

    pub fn post_form<T: DeserializeOwned, F: serde::Serialize>(
        &self,
        path: &str,
        form: F,
        advanced_headers: Option<HashMap<&'static str, String>>,
    ) -> Response<T> {
        let url = self.url(path);
        let headers = if let Some(advanced_headers) = advanced_headers {
            self.with_advanced_headers(advanced_headers)
        } else {
            self.headers()
        };

        let body: String = serde_json::to_string(&form)?;
        let request = self.client.post(&url).headers(headers).body(body);

        handle_request(request)
    }

    pub fn post<T: DeserializeOwned>(
        &self,
        path: &str,
        advanced_headers: Option<HashMap<&'static str, String>>,
    ) -> Response<T> {
        let url = self.url(path);
        let headers = if let Some(advanced_headers) = advanced_headers {
            self.with_advanced_headers(advanced_headers)
        } else {
            self.headers()
        };

        let request = self.client.post(&url).headers(headers);

        handle_request(request)
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
            if let (Some(account_id), Some(secret_key)) = (
                self.config.account_id.as_ref(),
                self.config.secret_key.as_ref(),
            ) {
                headers.insert(
                    reqwest::header::AUTHORIZATION,
                    HeaderValue::from_str(&format!("{}:{}", account_id, secret_key)).unwrap(),
                );
            } else {
                log::warn!("Not set `account_id` and `secret_key`");
            }
        }

        headers
    }

    fn with_advanced_headers(&self, advanced_headers: HashMap<&'static str, String>) -> HeaderMap {
        let mut headers = self.headers();

        for (header_name, header_value) in advanced_headers {
            headers.insert(
                HeaderName::from_static(header_name),
                HeaderValue::from_str(&header_value).unwrap(),
            );
        }

        headers
    }
}

fn handle_request<T: DeserializeOwned>(request: RequestBuilder) -> Response<T> {
    let mut response = request.send()?;
    let mut body = String::with_capacity(FOUR_KB);
    response.read_to_string(&mut body)?;

    let status = response.status();
    if !status.is_success() {
        let mut err: RequestError = serde_json::from_str(&body).unwrap_or_else(|err| {
            let mut req = RequestError::default();
            req.description = Some(format!("failed to deserialize error: {}", err));
            req
        });
        err.http_status = status.as_u16();
        return Err(Error::from(err));
    }

    serde_json::from_str(&body).map_err(Error::Serde)
}
