use crate::models::amount::Amount;
use crate::models::authorization_details::AuthorizationDetails;
use crate::models::cancellation_details::CancellationDetails;
use crate::models::confirmation::Confirmation;
use crate::models::metadata::Metadata;
use crate::models::payment_method::PaymentMethod;
use crate::models::receipt::ReceiptRegistrationStatus;
use crate::models::recipient::Recipient;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Payment {
    #[serde(rename = "id")]
    pub id: String,

    #[serde(rename = "status")]
    pub status: String,

    #[serde(rename = "amount")]
    pub amount: Amount,

    #[serde(rename = "description")]
    pub description: Option<String>,

    #[serde(rename = "recipient")]
    pub recipient: Option<Recipient>,

    #[serde(rename = "payment_method")]
    pub payment_method: PaymentMethod,

    #[serde(rename = "captured_at")]
    pub captured_at: Option<String>,

    #[serde(rename = "created_at")]
    pub created_at: String,

    #[serde(rename = "expires_at")]
    pub expires_at: Option<String>,

    #[serde(rename = "confirmation")]
    pub confirmation: Confirmation,

    #[serde(rename = "test")]
    pub test: bool,

    #[serde(rename = "refunded_amount")]
    pub refunded_amount: Amount,

    #[serde(rename = "paid")]
    pub paid: bool,

    #[serde(rename = "receipt_registration")]
    pub receipt_registration: Option<ReceiptRegistrationStatus>,

    #[serde(rename = "cancellation_details")]
    pub cancellation_details: Option<CancellationDetails>,

    #[serde(rename = "authorization_details")]
    pub authorization_details: Option<AuthorizationDetails>,

    #[serde(rename = "metadata")]
    pub metadata: Metadata,
}
