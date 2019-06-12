use std::collections::HashMap;

use futures::future;

use crate::common::{advanced_headers, PaymentId};
use crate::error::Error;
use crate::models::payment::{NewPayment, Payment};
use crate::r#async::{ApiClient, ResponseFuture};

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
    fn find_one(&self, payment_id: PaymentId) -> ResponseFuture<Payment>;

    fn create(
        &self,
        params: NewPayment,
        idempotency_key: Option<String>,
    ) -> ResponseFuture<Payment>;

    fn capture(
        &self,
        payment_id: PaymentId,
        idempotency_key: Option<String>,
    ) -> ResponseFuture<Payment>;

    fn cancel(
        &self,
        payment_id: PaymentId,
        idempotency_key: Option<String>,
    ) -> ResponseFuture<Payment>;

    fn list(&self) -> ResponseFuture<()>;
}

impl PaymentService for PaymentServiceImpl {
    fn find_one(&self, payment_id: PaymentId) -> ResponseFuture<Payment> {
        let request_path = format!("{}/{}", Self::BASE_PATH, payment_id);
        self.client.get(request_path.as_str())
    }

    fn create(
        &self,
        params: NewPayment,
        idempotency_key: Option<String>,
    ) -> ResponseFuture<Payment> {
        let advanced_headers: Option<HashMap<&'static str, String>> =
            advanced_headers(idempotency_key).into();

        self.client
            .post_form(Self::BASE_PATH, params, advanced_headers)
    }

    fn capture(
        &self,
        payment_id: PaymentId,
        idempotency_key: Option<String>,
    ) -> ResponseFuture<Payment> {
        let advanced_headers: Option<HashMap<&'static str, String>> =
            advanced_headers(idempotency_key).into();
        let url = format!("{}/{}/capture", Self::BASE_PATH, payment_id);
        self.client.post(&url, advanced_headers)
    }

    fn cancel(
        &self,
        payment_id: PaymentId,
        idempotency_key: Option<String>,
    ) -> ResponseFuture<Payment> {
        let advanced_headers: Option<HashMap<&'static str, String>> =
            advanced_headers(idempotency_key).into();
        let url = format!("{}/{}/cancel", Self::BASE_PATH, payment_id);
        self.client.post(&url, advanced_headers)
    }

    fn list(&self) -> ResponseFuture<()> {
        Box::new(future::err(Error::Unsupported(
            "list payments not supported",
        )))
    }
}
