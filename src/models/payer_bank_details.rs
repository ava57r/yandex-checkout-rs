use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct PayerBankDetails {
    pub full_name: String,
    pub short_name: String,
    pub address: String,
    pub inn: String,
    pub kpp: String,
    pub bank_name: String,
    pub bank_branch: String,
    pub bank_bik: String,
    pub account: String,
}
