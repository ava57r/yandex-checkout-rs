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

#[derive(Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum NewConfirmation {
    External(NewExternal),
    Redirect(NewRedirect),
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct NewExternal {
    pub locale: Option<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct NewRedirect {
    pub locale: Option<String>,
    pub enforce: Option<bool>,
    pub return_url: String,
}
