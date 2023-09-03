use std::collections::HashMap;
use std::str::FromStr;
use chrono::{DateTime, ParseError, TimeZone, Utc};
use serde::{Deserialize, Serialize};

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

impl ElSpotPricesResponse {
    pub fn into_records_as_map(self) -> HashMap<String, Record>{
        let mut map = HashMap::new();

        for record in self.records {
            let key = record.hour_utc_to_datetime().unwrap().format("%m/%d/%Y %H:%M").to_string();
            map.insert(key, record);
        }

        map
    }
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

impl Record {
    pub fn as_kwh_price_dkk(&self) -> f64 {
        self.spot_price_dkk / 1000.0
    }

    pub fn as_kwh_price_eur(&self) -> f64 {
        self.spot_price_eur / 1000.0
    }

    pub fn hour_utc_to_datetime(&self) -> Result<DateTime<Utc>, ParseError> {
        Utc.datetime_from_str(self.hour_utc.as_str(), "%Y-%m-%dT%H:%M:%S")
    }
}