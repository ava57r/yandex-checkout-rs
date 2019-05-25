use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Amount {
    #[serde(rename = "value")]
    pub value: String,

    #[serde(rename = "currency")]
    pub currency: String,
}
