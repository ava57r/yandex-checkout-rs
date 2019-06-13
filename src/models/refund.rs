use crate::common::PaymentId;
use crate::common::RefundId;
use crate::models::amount::Amount;
use crate::models::receipt::Receipt;

use chrono::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Refund {
    pub id: RefundId,
    pub payment_id: PaymentId,
    pub status: RefundStatus,
    pub created_at: DateTime<Utc>,
    pub amount: Amount,
    pub receipt_registration: Option<ReceiptRegistrationStatus>,
    pub description: Option<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RefundStatus {
    Succeeded,
    Canceled,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReceiptRegistrationStatus {
    Pending,
    Succeeded,
    Canceled,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct NewRefund {
    pub payment_id: PaymentId,
    pub amount: Option<Amount>,
    pub description: Option<String>,
    pub receipt: Option<Receipt>,
}

impl NewRefund {
    pub fn new(payment_id: PaymentId) -> Self {
        NewRefund {
            payment_id,
            amount: None,
            description: None,
            receipt: None,
        }
    }

    pub fn amount(mut self, value: Amount) -> Self {
        self.amount = Some(value);

        self
    }

    pub fn description(mut self, value: String) -> Self {
        self.description = Some(value);

        self
    }

    pub fn receipt(mut self, value: Receipt) -> Self {
        self.receipt = Some(value);

        self
    }
}
