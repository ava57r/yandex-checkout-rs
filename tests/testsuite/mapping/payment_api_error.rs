use serde_json;

use yandex_checkout::error::RequestError;

static SOURCE: &str = include_str!("json/payments_api_error.json");

#[test]
fn mapping() {
    let data: RequestError = serde_json::from_str(SOURCE).expect("Cannot der. RequestError");

    assert_eq!(data.code, Some("invalid_request".to_string()));
}
