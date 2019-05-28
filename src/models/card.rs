use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Card {
    pub first6: String,
    pub last4: String,
    pub expiry_month: String,
    pub expiry_year: String,
    pub card_type: CardType,
}

#[derive(Serialize, Deserialize)]
pub enum CardType {
    #[serde(rename = "MasterCard")]
    MasterCard,

    #[serde(rename = "Visa")]
    Visa,

    #[serde(rename = "Mir")]
    Mir,

    #[serde(rename = "UnionPay")]
    UnionPay,

    #[serde(rename = "JCB")]
    JCB,

    #[serde(rename = "AmericanExpress")]
    AmericanExpress,

    #[serde(rename = "DinersClub")]
    DinersClub,

    #[serde(rename = "Unknown")]
    Unknown,
}
