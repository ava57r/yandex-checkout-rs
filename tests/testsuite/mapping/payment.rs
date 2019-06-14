use serde_json;

use yandex_checkout::models::Payment;

static SOURCE_CREATED_PAYMENT: &str = include_str!("json/created_payment.json");

#[test]
fn mapping_created_payment() {
    let _data: Payment = serde_json::from_str(SOURCE_CREATED_PAYMENT).expect("Cannot der. Payment");
}

static SOURCE_NO_CAPTURED_PAYMENT: &str = include_str!("json/no_captured_payment.json");

#[test]
fn mapping_no_captured_payment() {
    let _data: Payment =
        serde_json::from_str(SOURCE_NO_CAPTURED_PAYMENT).expect("Cannot der. Payment");
}

static SOURCE_CAPTURED_PAYMENT: &str = include_str!("json/captured_payment.json");

#[test]
fn mapping_captured_payment() {
    let _data: Payment =
        serde_json::from_str(SOURCE_CAPTURED_PAYMENT).expect("Cannot der. Payment");
}

static SOURCE_CANCELED_PAYMENT: &str = include_str!("json/canceled_payment.json");

#[test]
fn mapping_canceled_payment() {
    let _data: Payment =
        serde_json::from_str(SOURCE_CANCELED_PAYMENT).expect("Cannot der. Payment");
}
