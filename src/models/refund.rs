use crate::models::amount::Amount;
use crate::models::receipt::Receipt;
use chrono::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Refund {
    pub id: String,
    pub payment_id: String,
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
    pub payment_id: String,
    pub amount: Option<Amount>,
    pub description: Option<String>,
    pub receipt: Option<Receipt>,
}
