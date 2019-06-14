extern crate yandex_checkout;

use yandex_checkout::client::ApiClient;
use yandex_checkout::common::PaymentId;
use yandex_checkout::config::Configuration;
use yandex_checkout::models::{Amount, NewPayment, Payment};
use yandex_checkout::payment_service::{PaymentService, PaymentServiceImpl};

fn main() {
    let config = Configuration {
        account_id: Some("account_id".to_string()),
        ..Configuration::default()
    };
    let client = ApiClient::new(config, reqwest::Client::new());
    let service = PaymentServiceImpl::new(client);

    let payment = create_payment(&service);
    let _ = get_payment(&service, payment.id);
}

fn get_payment<S: PaymentService>(service: &S, id: PaymentId) -> Payment {
    service.find_one(id).expect("Cannot run find_one")
}

fn create_payment<S: PaymentService>(service: &S) -> Payment {
    let amount = Amount {
        value: "0".to_string(),
        currency: "RUB".to_string(),
    };
    let new_payment = NewPayment::new(amount);
    service
        .create(new_payment, None)
        .expect("Cannot run create")
}
