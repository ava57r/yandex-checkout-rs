use crate::models::payer_bank_details::PayerBankDetails;
use crate::models::vat_data::VatData;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum PaymentMethod {
    #[serde(rename = "alfabank")]
    Alfabank(AlfabankType),
    #[serde(rename = "apple_pay")]
    ApplePay(ApplePayType),

    #[serde(rename = "b2b_sberbank")]
    B2BSberbank(B2BSberbankType),

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

    #[serde(rename = "webmoney")]
    Webmoney,

    #[serde(rename = "mobile_balance")]
    MobileBalance,

    #[serde(rename = "installments")]
    Installments,

    #[serde(rename = "psb")]
    Psb,

    #[serde(rename = "google_pay")]
    GooglePay,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AlfabankType {
    pub id: String,
    pub saved: bool,
    pub title: Option<String>,
    pub login: Option<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ApplePayType {
    pub id: String,
    pub saved: bool,
    pub title: Option<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct B2BSberbankType {
    pub id: String,
    pub saved: bool,
    pub title: Option<String>,
    pub payment_purpose: String,
    pub vat_data: VatData,
    pub payer_bank_details: PayerBankDetails,
}
