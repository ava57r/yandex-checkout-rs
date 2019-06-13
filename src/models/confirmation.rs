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

impl Redirect {
    pub fn new(confirmation_url: String) -> Self {
        Redirect {
            confirmation_url,
            return_url: None,
            enforce: None,
        }
    }

    pub fn enforce(mut self, value: bool) -> Self {
        self.enforce = Some(value);

        self
    }

    pub fn return_url(mut self, value: String) -> Self {
        self.return_url = Some(value);

        self
    }
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

impl NewRedirect {
    pub fn new(return_url: String) -> Self {
        NewRedirect {
            return_url,
            enforce: None,
            locale: None,
        }
    }

    pub fn enforce(mut self, value: bool) -> Self {
        self.enforce = Some(value);

        self
    }

    pub fn locale(mut self, value: String) -> Self {
        self.locale = Some(value);

        self
    }
}
