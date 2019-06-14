extern crate futures;
extern crate reqwest;
extern crate tokio;
extern crate yandex_checkout;

use std::sync::Arc;

use futures::Future;

use yandex_checkout::config::Configuration;
use yandex_checkout::error::Error;
use yandex_checkout::models::{Amount, NewPayment, Payment};
use yandex_checkout::r#async::payment_service::{PaymentService, PaymentServiceImpl};
use yandex_checkout::r#async::ApiClient;

fn main() {
    env_logger::init();

    tokio::run(fetch());
}

fn fetch() -> impl Future<Item = (), Error = ()> {
    let config = Configuration {
        account_id: Some("yandex_test1".to_string()),
        secret_key:Some("d1da36613493".to_string()),
        ..Configuration::default()
    };
    let client = ApiClient::new(config, reqwest::r#async::Client::new());
    let service = Arc::new(PaymentServiceImpl::new(client));

    create_payment(service.clone())
        .and_then(move |payment| service.find_one(payment.id))
        .map_err(|e| {
            println!("err: {:?}", e);
            ()
        })
        .map(|_| ())
}

fn create_payment<S: PaymentService>(
    service: Arc<S>,
) -> impl Future<Item = Payment, Error = Error> {
    let amount = Amount {
        value: "0".to_string(),
        currency: "RUB".to_string(),
    };
    let new_payment = NewPayment::new(amount);
    service.create(new_payment, None)
}
