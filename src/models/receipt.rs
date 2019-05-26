use crate::models::amount::Amount;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReceiptRegistrationStatus {
    Pending,
    Succeeded,
    Canceled,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Receipt {
    pub items: Vec<ReceiptItem>,
    pub tax_system_code: Option<u32>,
    pub phone: Option<String>,
    pub email: Option<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ReceiptItem {
    pub description: String,
    pub quantity: String,
    pub amount: Amount,
    pub vat_code: u32,
    pub payment_subject: Option<String>,
    pub payment_mode: Option<String>,
}
