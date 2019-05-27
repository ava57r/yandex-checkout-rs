use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Confirmation {
    External,
    Redirect(Redirect),
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Redirect {
    pub enforce: Option<bool>,
    pub return_url: Option<String>,
    pub confirmation_url: String,
}
