use serde_json;

use yandex_checkout::models::{NewPayment, PaymentMethodData};

static SOURCE: &str = include_str!("json/new_payment.json");

#[test]
fn mapping_new_payment() {
    let data: NewPayment = serde_json::from_str(SOURCE).expect("Cannot der. NewPayment");

    let result = match data.payment_method_data {
        Some(PaymentMethodData::BankCard(_)) => true,
        _ => false,
    };

    assert!(result, "Cannot get payment_method_data");
}
