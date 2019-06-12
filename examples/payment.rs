extern crate yandex_checkout;

use yandex_checkout::client::ApiClient;
use yandex_checkout::common::PaymentId;
use yandex_checkout::config::Configuration;
use yandex_checkout::models::amount::Amount;
use yandex_checkout::models::payment::{NewPayment, Payment};
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
    let new_payment = NewPayment {
        amount,
        description: None,
        receipt: None,
        recipient: None,
        payment_token: None,
        payment_method_id: None,
        payment_method_data: None,
        confirmation: None,
        save_payment_method: None,
        capture: None,
        client_ip: None,
        metadata: None,
        airline: None,
    };
    service.create(new_payment, None).expect("Cannot run create")
}
