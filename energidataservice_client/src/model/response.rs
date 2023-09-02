use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ElSpotPricesResponse {
    pub total: i64,
    pub filters: String,
    pub sort: String,
    pub limit: i64,
    pub dataset: String,
    pub records: Vec<Record>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Record {
    #[serde(rename = "HourUTC")]
    pub hour_utc: String,
    #[serde(rename = "HourDK")]
    pub hour_dk: String,
    #[serde(rename = "PriceArea")]
    pub price_area: String,
    #[serde(rename = "SpotPriceDKK")]
    pub spot_price_dkk: f64,
    #[serde(rename = "SpotPriceEUR")]
    pub spot_price_eur: f64,
}
