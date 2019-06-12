use std::collections::HashMap;

use crate::common::{advanced_headers, RefundId};
use crate::models::refund::{NewRefund, Refund};
use crate::r#async::{ApiClient, ResponseFuture};

pub struct RefundServiceImpl {
    client: ApiClient,
}

impl RefundServiceImpl {
    const BASE_PATH: &'static str = "/refunds";

    pub fn new(client: ApiClient) -> Self {
        Self { client }
    }
}

pub trait RefundService {
    fn find_one(&self, refund_id: RefundId) -> ResponseFuture<Refund>;

    fn create(&self, params: NewRefund, idempotency_key: Option<String>) -> ResponseFuture<Refund>;
}

impl RefundService for RefundServiceImpl {
    fn find_one(&self, refund_id: RefundId) -> ResponseFuture<Refund> {
        let request_path = format!("{}/{}", Self::BASE_PATH, refund_id);
        self.client.get(request_path.as_str())
    }

    fn create(&self, params: NewRefund, idempotency_key: Option<String>) -> ResponseFuture<Refund> {
        let advanced_headers: Option<HashMap<&'static str, String>> =
            advanced_headers(idempotency_key).into();

        self.client
            .post_form(Self::BASE_PATH, params, advanced_headers)
    }
}
