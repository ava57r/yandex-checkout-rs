use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Confirmation {
    #[serde(rename = "type")]
    pub confirmation_type: String,
}
