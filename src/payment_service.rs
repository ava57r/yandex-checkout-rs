use crate::client::ApiClient;
use crate::error::ApiResult;
use crate::models::payment::Payment;

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
    fn find_one(&self) -> ApiResult<Payment>;

    fn create(&self, idempotency_key: Option<String>) -> ApiResult<Payment>;

    fn capture(&self, payment_id: String, idempotency_key: Option<String>) -> ApiResult<Payment>;

    fn cancel(&self, payment_id: String, idempotency_key: Option<String>) -> ApiResult<Payment>;

    fn list(&self);
}

impl PaymentService for PaymentServiceImpl {
    fn find_one(&self) -> ApiResult<Payment> {
        unimplemented!()
    }

    fn create(&self, idempotency_key: Option<String>) -> ApiResult<Payment> {
        unimplemented!()
    }

    fn capture(&self, payment_id: String, idempotency_key: Option<String>) -> ApiResult<Payment> {
        unimplemented!()
    }

    fn cancel(&self, payment_id: String, idempotency_key: Option<String>) -> ApiResult<Payment> {
        unimplemented!()
    }

    fn list(&self) {
        unimplemented!()
    }
}
