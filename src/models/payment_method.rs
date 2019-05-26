use crate::models::card::Card;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct PaymentMethod {
    #[serde(rename = "type")]
    pub payment_method_type: PaymentMethodType,

    #[serde(rename = "id")]
    pub id: String,

    #[serde(rename = "saved")]
    pub saved: bool,

    #[serde(rename = "card")]
    pub card: Option<Card>,

    #[serde(rename = "title")]
    pub title: String,

    #[serde(rename = "title")]
    pub login: String,
}

#[derive(Serialize, Deserialize)]
pub enum PaymentMethodType {
    #[serde(rename = "sberbank")]
    Sberbank,
    #[serde(rename = "bank_card")]
    BankCard,
    #[serde(rename = "cash")]
    Cash,
    #[serde(rename = "yandex_money")]
    YandexMoney,
    #[serde(rename = "qiwi")]
    Qiwi,
    #[serde(rename = "alfabank")]
    Alfabank,
    #[serde(rename = "webmoney")]
    Webmoney,
    #[serde(rename = "apple_pay")]
    ApplePay,
    #[serde(rename = "mobile_balance")]
    MobileBalance,
    #[serde(rename = "installments")]
    Installments,
    #[serde(rename = "psb")]
    Psb,
    #[serde(rename = "google_pay")]
    GooglePay,
    #[serde(rename = "b2b_sberbank")]
    B2BSberbank,
}
