use std::collections::HashMap;
use std::fmt;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentId(String);

impl PaymentId {
    pub fn new(inner: String) -> Self {
        PaymentId(inner)
    }
}
impl AsRef<str> for PaymentId {
    fn as_ref(&self) -> &str {
        self.0.as_str()
    }
}

impl From<PaymentId> for String {
    fn from(other: PaymentId) -> Self {
        other.0
    }
}

impl fmt::Display for PaymentId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}

pub fn advanced_headers(idempotency_key: Option<String>) -> HashMap<&'static str, String> {
    let idempotency_key = match idempotency_key {
        Some(key) => key,
        None => Uuid::new_v4().to_hyphenated().to_string(),
    };
    let mut advanced_headers = HashMap::new();
    advanced_headers.insert("idempotence-key", idempotency_key);

    advanced_headers
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RefundId(String);

impl RefundId {
    pub fn new(inner: String) -> Self {
        RefundId(inner)
    }
}
impl AsRef<str> for RefundId {
    fn as_ref(&self) -> &str {
        self.0.as_str()
    }
}

impl From<RefundId> for String {
    fn from(other: RefundId) -> Self {
        other.0
    }
}

impl fmt::Display for RefundId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}
