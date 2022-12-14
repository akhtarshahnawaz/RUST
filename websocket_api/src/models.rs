use serde::de;
use serde::{Deserialize, Deserializer};

// For Multiple Depth Streams
// `derive` macro implements Debug (For easy printing) and Deserialize (For serde Deserialization) Traits for the struct
#[derive(Debug, Deserialize)]
pub struct DepthStreamWrapper {
    pub stream: String,
    pub data: DepthStreamData,
}

// Individual Offer
// `derive` implements Debug (For easy printing) and Deserialize (For serde Deserialization) Traits for the struct
#[derive(Debug, Deserialize)]
pub struct OfferData {
    // deserialization happens with the funtion `de_float_from_str` which is defined below
    #[serde(deserialize_with = "de_float_from_str")]
    pub price: f32,
    #[serde(deserialize_with = "de_float_from_str")]
    pub size: f32,
}

// Maintaining Depth Order Book
// `derive` implements Debug (For easy printing) and Deserialize (For serde Deserialization) Traits for the struct
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DepthStreamData {
    pub last_update_id: usize,
    pub bids: Vec<OfferData>,
    pub asks: Vec<OfferData>,
}


pub fn de_float_from_str<'a, D>(deserializer: D) -> Result<f32, D::Error> where D: Deserializer<'a>,
{
    let str_val = String::deserialize(deserializer)?;
    str_val.parse::<f32>().map_err(de::Error::custom)
}