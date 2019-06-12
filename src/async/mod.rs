use std::collections::HashMap;

use crate::error::{Error, ErrorResponse, RequestError};
use futures::future::{self, Future};
use futures::Stream;
use reqwest;
use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use reqwest::r#async::Client;
use reqwest::r#async::RequestBuilder;
use serde::de::DeserializeOwned;
use serde_json;

pub mod payment_service;
pub mod refund_service;
pub mod settings_service;

use crate::config::Configuration;

const FOUR_KB: usize = 4096;

pub type ResponseFuture<T> = Box<dyn Future<Item = T, Error = Error> + Send>;

pub struct ApiClient {
    client: Client,
    config: Configuration,
}

impl ApiClient {
    pub fn new(config: Configuration, client: Client) -> Self {
        Self { client, config }
    }

    pub fn get<T>(&self, path: &str) -> ResponseFuture<T>
    where
        T: DeserializeOwned + Send + 'static,
    {
        let url = self.url(path);
        let request = self.client.get(&url).headers(self.headers());

        handle_request(request)
    }

    pub fn post_form<T, F: serde::Serialize>(
        &self,
        path: &str,
        form: F,
        advanced_headers: Option<HashMap<&'static str, String>>,
    ) -> ResponseFuture<T>
    where
        T: DeserializeOwned + Send + 'static,
    {
        let url = self.url(path);
        let headers = if let Some(advanced_headers) = advanced_headers {
            self.with_advanced_headers(advanced_headers)
        } else {
            self.headers()
        };

        let body: String = match serde_json::to_string(&form).map_err(Error::Serde) {
            Err(e) => return Box::new(future::err(e)),
            Ok(ok) => ok,
        };

        let request = self.client.post(&url).headers(headers).body(body);

        handle_request(request)
    }

    pub fn post<T>(
        &self,
        path: &str,
        advanced_headers: Option<HashMap<&'static str, String>>,
    ) -> ResponseFuture<T>
    where
        T: DeserializeOwned + Send + 'static,
    {
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

fn handle_request<T: DeserializeOwned + Send + 'static>(
    request: RequestBuilder,
) -> ResponseFuture<T> {
    let fut = request.send().map_err(Error::Http).and_then(|response| {
        let status = response.status();
        response
            .into_body()
            .concat2()
            .map_err(Error::Http)
            .and_then(move |body| {
                let mut body = std::io::Cursor::new(body);
                let mut bytes = Vec::with_capacity(FOUR_KB);
                std::io::copy(&mut body, &mut bytes)?;

                if !status.is_success() {
                    let mut err = serde_json::from_slice(&bytes).unwrap_or_else(|err| {
                        let mut req = ErrorResponse {
                            error: RequestError::default(),
                        };
                        req.error.description =
                            Some(format!("failed to deserialize error: {}", err));
                        req
                    });
                    err.error.http_status = status.as_u16();
                    return Err(Error::from(err.error));
                }

                serde_json::from_slice(&bytes).map_err(Error::Serde)
            })
    });

    Box::new(fut)
}
