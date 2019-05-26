use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum ReceiptRegistrationStatus {
    #[serde(rename = "pending")]
    Pending,
    #[serde(rename = "succeeded")]
    Succeeded,
    #[serde(rename = "canceled")]
    Canceled,
}
