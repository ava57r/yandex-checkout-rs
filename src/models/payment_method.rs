use crate::models::card::Card;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct PaymentMethod {
    #[serde(rename = "type")]
    pub payment_method_type: String,

    #[serde(rename = "id")]
    pub id: String,

    #[serde(rename = "saved")]
    pub saved: bool,

    #[serde(rename = "card")]
    pub card: Card,

    #[serde(rename = "title")]
    pub title: String,
}
