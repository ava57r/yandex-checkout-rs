use std::collections::HashMap;

use crate::client::ApiClient;
use crate::common::{advanced_headers, PaymentId};
use crate::error::{ApiResult, Error};
use crate::models::payment::{NewPayment, Payment};

pub struct PaymentServiceImpl {
    client: ApiClient,
}

impl PaymentServiceImpl {
    const BASE_PATH: &'static str = "/payments";

    pub fn new(client: ApiClient) -> Self {
        Self { client }
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
            advanced_headers(idempotency_key).into();

        self.client
            .post_form(Self::BASE_PATH, params, advanced_headers)
    }

    fn capture(
        &self,
        payment_id: PaymentId,
        idempotency_key: Option<String>,
    ) -> ApiResult<Payment> {
        let advanced_headers: Option<HashMap<&'static str, String>> =
            advanced_headers(idempotency_key).into();
        let url = format!("{}/{}/capture", Self::BASE_PATH, payment_id);
        self.client.post(&url, advanced_headers)
    }

    fn cancel(&self, payment_id: PaymentId, idempotency_key: Option<String>) -> ApiResult<Payment> {
        let advanced_headers: Option<HashMap<&'static str, String>> =
            advanced_headers(idempotency_key).into();
        let url = format!("{}/{}/cancel", Self::BASE_PATH, payment_id);
        self.client.post(&url, advanced_headers)
    }

    fn list(&self) -> ApiResult<()> {
        Err(Error::Unsupported("list payments not supported"))
    }
}
