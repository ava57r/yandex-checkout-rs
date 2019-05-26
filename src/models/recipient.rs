use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Recipient {
    #[serde(rename = "gateway_id")]
    pub gateway_id: String,
}
