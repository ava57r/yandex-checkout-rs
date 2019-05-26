use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Confirmation {
    #[serde(rename = "type")]
    pub confirmation_type: ConfirmationType,

    #[serde(rename = "enforce")]
    pub enforce: Option<bool>,

    #[serde(rename = "return_url")]
    pub return_url: Option<String>,

    #[serde(rename = "confirmation_url")]
    pub confirmation_url: Option<String>, // required when type = redirect
}

#[derive(Serialize, Deserialize)]
pub enum ConfirmationType {
    #[serde(rename = "redirect")]
    Redirect,
    #[serde(rename = "external")]
    External,
}
