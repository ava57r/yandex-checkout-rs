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
    pub payment_subject: Option<PaymentSubject>,
    pub payment_mode: Option<PaymentMode>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PaymentSubject {
    Commodity,
    Excise,
    Job,
    Service,
    GamblingBet,
    GamblingPrize,
    Lottery,
    LotteryPrize,
    IntellectualActivity,
    Payment,
    AgentCommission,
    PropertyRight,
    NonOperatingGain,
    InsurancePremium,
    SalesTax,
    ResortFee,
    Composite,
    Another,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PaymentMode {
    FullPrepayment,
    PartialPrepayment,
    Advance,
    FullPayment,
    PartialPayment,
    Credit,
    CreditPayment,
}
