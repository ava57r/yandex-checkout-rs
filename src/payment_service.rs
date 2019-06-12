use std::collections::HashMap;
use std::fmt;

use uuid::Uuid;

use crate::client::ApiClient;
use crate::error::{ApiResult, Error};
use crate::models::payment::{NewPayment, Payment};

#[derive(Debug)]
pub struct PaymentId(String);

impl PaymentId {
    pub fn new(inner: String) -> Self {
        PaymentId(inner)
    }
}
impl AsRef<str> for PaymentId {
    fn as_ref(&self) -> &str {
        self.0.as_str()
    }
}

impl From<PaymentId> for String {
    fn from(other: PaymentId) -> Self {
        other.0
    }
}

impl fmt::Display for PaymentId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}

pub struct PaymentServiceImpl {
    client: ApiClient,
}

impl PaymentServiceImpl {
    const BASE_PATH: &'static str = "/payments";

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

pub trait PaymentService {
    fn find_one(&self, payment_id: PaymentId) -> ApiResult<Payment>;

    fn create(&self, params: NewPayment, idempotency_key: Option<String>) -> ApiResult<Payment>;

    fn capture(&self, payment_id: PaymentId, idempotency_key: Option<String>)
        -> ApiResult<Payment>;

    fn cancel(&self, payment_id: PaymentId, idempotency_key: Option<String>) -> ApiResult<Payment>;

    fn list(&self) -> ApiResult<()>;
}

impl PaymentService for PaymentServiceImpl {
    fn find_one(&self, payment_id: PaymentId) -> ApiResult<Payment> {
        let request_path = format!("{}/{}", Self::BASE_PATH, payment_id);
        self.client.get(request_path.as_str())
    }

    fn create(&self, params: NewPayment, idempotency_key: Option<String>) -> ApiResult<Payment> {
        let advanced_headers: Option<HashMap<&'static str, String>> =
            self.advanced_headers(idempotency_key).into();

        self.client
            .post_form(Self::BASE_PATH, params, advanced_headers)
    }

    fn capture(
        &self,
        payment_id: PaymentId,
        idempotency_key: Option<String>,
    ) -> ApiResult<Payment> {
        let advanced_headers: Option<HashMap<&'static str, String>> =
            self.advanced_headers(idempotency_key).into();
        let url = format!("{}/{}/capture", Self::BASE_PATH, payment_id);
        self.client.post(&url, advanced_headers)
    }

    fn cancel(&self, payment_id: PaymentId, idempotency_key: Option<String>) -> ApiResult<Payment> {
        let advanced_headers: Option<HashMap<&'static str, String>> =
            self.advanced_headers(idempotency_key).into();
        let url = format!("{}/{}/cancel", Self::BASE_PATH, payment_id);
        self.client.post(&url, advanced_headers)
    }

    fn list(&self) -> ApiResult<()> {
        Err(Error::Unsupported("list payments not supported"))
    }
}
