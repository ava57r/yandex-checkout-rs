use crate::common::PaymentId;
use crate::models::airline::Airline;
use crate::models::amount::Amount;
use crate::models::authorization_details::AuthorizationDetails;
use crate::models::cancellation_details::CancellationDetails;
use crate::models::confirmation::{Confirmation, NewConfirmation};
use crate::models::metadata::Metadata;
use crate::models::payment_method::{PaymentMethod, PaymentMethodData};
use crate::models::receipt::Receipt;
use crate::models::receipt::ReceiptRegistrationStatus;
use crate::models::recipient::Recipient;

use chrono::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Payment {
    pub id: PaymentId,
    pub status: PaymentStatus,
    pub amount: Amount,
    pub description: Option<String>,
    pub recipient: Option<Recipient>,
    pub payment_method: PaymentMethod,
    pub captured_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub expires_at: Option<DateTime<Utc>>,
    pub confirmation: Option<Confirmation>,
    pub test: bool,
    pub refunded_amount: Option<Amount>,
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
    pub payment_method_data: Option<PaymentMethodData>,
    pub confirmation: Option<NewConfirmation>,
    pub save_payment_method: Option<bool>,
    pub capture: Option<bool>,
    pub client_ip: Option<String>,
    pub metadata: Option<Metadata>,
    pub airline: Option<Airline>,
}

impl NewPayment {
    pub fn new(amount: Amount) -> Self {
        NewPayment {
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
        }
    }

    pub fn description(mut self, value: String) -> Self {
        self.description = Some(value);

        self
    }

    pub fn receipt(mut self, value: Receipt) -> Self {
        self.receipt = Some(value);

        self
    }

    pub fn recipient(mut self, value: Recipient) -> Self {
        self.recipient = Some(value);

        self
    }

    pub fn payment_token(mut self, value: String) -> Self {
        self.payment_token = Some(value);

        self
    }

    pub fn payment_method_id(mut self, value: String) -> Self {
        self.payment_method_id = Some(value);

        self
    }

    pub fn payment_method_data(mut self, value: PaymentMethodData) -> Self {
        self.payment_method_data = Some(value);

        self
    }

    pub fn confirmation(mut self, value: NewConfirmation) -> Self {
        self.confirmation = Some(value);

        self
    }

    pub fn save_payment_method(mut self, value: bool) -> Self {
        self.save_payment_method = Some(value);

        self
    }

    pub fn capture(mut self, value: bool) -> Self {
        self.capture = Some(value);

        self
    }

    pub fn client_ip(mut self, value: String) -> Self {
        self.client_ip = Some(value);

        self
    }

    pub fn metadata(mut self, value: Metadata) -> Self {
        self.metadata = Some(value);

        self
    }

    pub fn airline(mut self, value: Airline) -> Self {
        self.airline = Some(value);

        self
    }
}

#[derive(Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct CapturePayment {
    pub amount: Option<Amount>,
    pub receipt: Option<Receipt>,
    pub airline: Option<Airline>,
}
