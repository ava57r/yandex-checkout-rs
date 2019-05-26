use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Airline {
    pub ticket_number: Option<String>,
    pub booking_reference: Option<String>,
    pub passengers: Vec<Passenger>,
    pub legs: Vec<Leg>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Passenger {
    pub first_name: String,
    pub last_name: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Leg {
    pub departure_airport: String,
    pub destination_airport: String,
    pub departure_date: String,
    pub carrier_code: Option<String>,
}
