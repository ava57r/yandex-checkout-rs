use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct AuthorizationDetails {
    #[serde(rename = "rrn")]
    pub rrn: Option<String>,

    #[serde(rename = "auth_code")]
    pub auth_code: Option<String>,
}
