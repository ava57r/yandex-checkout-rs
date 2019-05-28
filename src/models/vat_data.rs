use crate::models::amount::Amount;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum VatData {
    Calculated(CalculatedType),
    Mixed(MixedType),
    Untaxed,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CalculatedType {
    pub rate: String,
    pub amount: Amount,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct MixedType {
    pub amount: Amount,
}
