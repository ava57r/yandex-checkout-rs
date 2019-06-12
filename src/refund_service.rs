use std::collections::HashMap;
use std::fmt;

use uuid::Uuid;

use crate::client::ApiClient;
use crate::error::ApiResult;
use crate::models::refund::{NewRefund, Refund};

#[derive(Debug)]
pub struct RefundId(String);

impl RefundId {
    pub fn new(inner: String) -> Self {
        RefundId(inner)
    }
}
impl AsRef<str> for RefundId {
    fn as_ref(&self) -> &str {
        self.0.as_str()
    }
}

impl From<RefundId> for String {
    fn from(other: RefundId) -> Self {
        other.0
    }
}

impl fmt::Display for RefundId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}

pub struct RefundServiceImpl {
    client: ApiClient,
}

impl RefundServiceImpl {
    const BASE_PATH: &'static str = "/refunds";

    pub fn new(client: ApiClient) -> Self {
        Self { client }
    }

    fn advanced_headers(&self, idempotency_key: Option<String>) -> HashMap<&'static str, String> {
        let idempotency_key = match idempotency_key {
            Some(key) => key,
            None => Uuid::new_v4().to_hyphenated().to_string(),
        };
        let mut advanced_headers = HashMap::new();
        advanced_headers.insert("idempotence-key", idempotency_key);

        advanced_headers
    }
}

pub trait RefundService {
    fn find_one(&self, refund_id: RefundId) -> ApiResult<Refund>;

    fn create(&self, params: NewRefund, idempotency_key: Option<String>) -> ApiResult<Refund>;
}

impl RefundService for RefundServiceImpl {
    fn find_one(&self, refund_id: RefundId) -> ApiResult<Refund> {
        let request_path = format!("{}/{}", Self::BASE_PATH, refund_id);
        self.client.get(request_path.as_str())
    }

    fn create(&self, params: NewRefund, idempotency_key: Option<String>) -> ApiResult<Refund> {
        let advanced_headers: Option<HashMap<&'static str, String>> =
            self.advanced_headers(idempotency_key).into();

        self.client
            .post_form(Self::BASE_PATH, params, advanced_headers)
    }
}
