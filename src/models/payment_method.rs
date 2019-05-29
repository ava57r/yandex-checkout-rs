use crate::models::card::Card;
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

    #[serde(rename = "bank_card")]
    BankCard(BankCardType),

    #[serde(rename = "cash")]
    Cash(CashType),

    #[serde(rename = "google_pay")]
    GooglePay(GooglePayType),

    #[serde(rename = "installments")]
    Installments(InstallmentsType),

    #[serde(rename = "mobile_balance")]
    MobileBalance(MobileBalanceType),

    #[serde(rename = "qiwi")]
    Qiwi(QiwiType),

    #[serde(rename = "sberbank")]
    Sberbank(SberbankType),

    #[serde(rename = "sberbank")]
    TinkoffBank(TinkoffBankType),

    #[serde(rename = "webmoney")]
    Webmoney(WebmoneyType),

    #[serde(rename = "yandex_money")]
    YandexMoney(YandexMoneyType),

    #[serde(other, skip_serializing)]
    Other,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PaymentMethodFields {
    pub id: String,
    pub saved: bool,
    pub title: Option<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AlfabankType {
    #[serde(flatten)]
    pub shared_fields: PaymentMethodFields,
    pub login: Option<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ApplePayType {
    #[serde(flatten)]
    pub shared_fields: PaymentMethodFields,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct B2BSberbankType {
    #[serde(flatten)]
    pub shared_fields: PaymentMethodFields,
    pub payment_purpose: String,
    pub vat_data: VatData,
    pub payer_bank_details: PayerBankDetails,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct BankCardType {
    #[serde(flatten)]
    pub shared_fields: PaymentMethodFields,
    pub card: Option<Card>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CashType {
    #[serde(flatten)]
    pub shared_fields: PaymentMethodFields,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GooglePayType {
    #[serde(flatten)]
    pub shared_fields: PaymentMethodFields,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct InstallmentsType {
    #[serde(flatten)]
    pub shared_fields: PaymentMethodFields,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct MobileBalanceType {
    #[serde(flatten)]
    pub shared_fields: PaymentMethodFields,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct QiwiType {
    #[serde(flatten)]
    pub shared_fields: PaymentMethodFields,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SberbankType {
    #[serde(flatten)]
    pub shared_fields: PaymentMethodFields,
    pub phone: Option<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct TinkoffBankType {
    #[serde(flatten)]
    pub shared_fields: PaymentMethodFields,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct WebmoneyType {
    #[serde(flatten)]
    pub shared_fields: PaymentMethodFields,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct YandexMoneyType {
    #[serde(flatten)]
    pub shared_fields: PaymentMethodFields,
    pub account_number: Option<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum PaymentMethodData {
    #[serde(rename = "alfabank")]
    Alfabank(NewAlfabankType),
    #[serde(rename = "apple_pay")]
    ApplePay(NewApplePayType),

    #[serde(rename = "b2b_sberbank")]
    B2BSberbank(NewB2BSberbankType),

    #[serde(rename = "bank_card")]
    BankCard(NewBankCardType),

    #[serde(rename = "cash")]
    Cash(NewCashType),

    #[serde(rename = "google_pay")]
    GooglePay(NewGooglePayType),

    #[serde(rename = "installments")]
    Installments,

    #[serde(rename = "mobile_balance")]
    MobileBalance(NewMobileBalanceType),

    #[serde(rename = "qiwi")]
    Qiwi(NewQiwiType),

    #[serde(rename = "sberbank")]
    Sberbank(NewSberbankType),

    #[serde(rename = "sberbank")]
    TinkoffBank,

    #[serde(rename = "webmoney")]
    Webmoney,

    #[serde(rename = "yandex_money")]
    YandexMoney,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct NewAlfabankType {
    #[serde(flatten)]
    pub login: Option<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct NewApplePayType {
    pub payment_data: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct NewB2BSberbankType {
    pub payment_purpose: String,
    pub vat_data: VatData,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct NewBankCardType {
    pub card: Option<Card>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct NewCashType {
    pub phone: Option<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct NewGooglePayType {
    pub payment_method_token: String,
    pub google_transaction_id: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct NewMobileBalanceType {
    pub phone: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct NewQiwiType {
    pub phone: Option<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct NewSberbankType {
    pub phone: Option<String>,
}
