use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct CancellationDetails {
    #[serde(rename = "party")]
    pub party: PartyType,

    #[serde(rename = "reason")]
    pub reason: ReasonType,
}

#[derive(Serialize, Deserialize)]
pub enum PartyType {
    #[serde(rename = "yandex_checkout")]
    YandexCheckout,

    #[serde(rename = "payment_network")]
    PaymentNetwork,

    #[serde(rename = "merchant")]
    Merchant,
}

#[derive(Serialize, Deserialize)]
pub enum ReasonType {
    #[serde(rename = "3d_secure_failed")]
    ThreeDSecureFailed,

    #[serde(rename = "call_issuer")]
    CallIssuer,

    #[serde(rename = "card_expired")]
    CardExpired,

    #[serde(rename = "country_forbidden")]
    CountryForbidden,

    #[serde(rename = "fraud_suspected")]
    FraudSuspected,

    #[serde(rename = "general_decline")]
    GeneralDecline,

    #[serde(rename = "identification_required")]
    IdentificationRequired,

    #[serde(rename = "insufficient_funds")]
    InsufficientFunds,

    #[serde(rename = "invalid_card_number")]
    InvalidCardNumber,

    #[serde(rename = "invalid_csc")]
    InvalidCsc,

    #[serde(rename = "issuer_unavailable")]
    IssuerUnavailable,

    #[serde(rename = "payment_method_limit_exceeded")]
    PaymentMethodLimitExceeded,

    #[serde(rename = "payment_method_restricted")]
    PaymentMethodRestricted,
}
