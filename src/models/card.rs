use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Card {
    #[serde(rename = "first6")]
    pub first6: String,

    #[serde(rename = "last4")]
    pub last4: String,

    #[serde(rename = "expiry_month")]
    pub expiry_month: String,

    #[serde(rename = "expiry_year")]
    pub expiry_year: String,

    #[serde(rename = "card_type")]
    pub card_type: String,
}
