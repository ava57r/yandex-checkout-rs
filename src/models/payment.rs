use crate::models::airline::Airline;
use crate::models::amount::Amount;
use crate::models::authorization_details::AuthorizationDetails;
use crate::models::cancellation_details::CancellationDetails;
use crate::models::confirmation::Confirmation;
use crate::models::metadata::Metadata;
use crate::models::payment_method::PaymentMethod;
use crate::models::receipt::Receipt;
use crate::models::receipt::ReceiptRegistrationStatus;
use crate::models::recipient::Recipient;
use chrono::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Payment {
    pub id: String,
    pub status: PaymentStatus,
    pub amount: Amount,
    pub description: Option<String>,
    pub recipient: Option<Recipient>,
    pub payment_method: PaymentMethod,
    pub captured_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub expires_at: Option<DateTime<Utc>>,
    pub confirmation: Confirmation,
    pub test: bool,
    pub refunded_amount: Amount,
    pub paid: bool,
    pub receipt_registration: Option<ReceiptRegistrationStatus>,
    pub cancellation_details: Option<CancellationDetails>,
    pub authorization_details: Option<AuthorizationDetails>,
    pub metadata: Metadata,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PaymentStatus {
    Pending,
    WaitingForCapture,
    Succeeded,
    Canceled,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct NewPayment {
    pub amount: Amount,
    pub description: Option<String>,
    pub receipt: Option<Receipt>,
    pub recipient: Option<Recipient>,
    pub payment_token: Option<String>,
    pub payment_method_id: Option<String>,
    pub payment_method_data: Option<PaymentMethod>,
    pub confirmation: Option<Confirmation>,
    pub save_payment_method: Option<bool>,
    pub capture: Option<bool>,
    pub client_ip: Option<String>,
    pub metadata: Option<Metadata>,
    pub airline: Option<Airline>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CapturePayment {
    pub amount: Option<Amount>,
    pub receipt: Option<Receipt>,
    pub airline: Option<Airline>,
}
