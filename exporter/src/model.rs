use std::collections::{BTreeMap, HashMap};
use serde::{Deserialize, Serialize};
use eloverblik_client::model::response::GetMeteringDataTimeSeriesResponseResult;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UsageTimeSeries {
    pub data : BTreeMap<String, f64>,
    pub granularity : Granularity
}

impl UsageTimeSeries {
    pub fn new(gran : Granularity) -> Self {
        UsageTimeSeries {
            granularity:  gran,
            data: BTreeMap::new()
        }
    }

    pub fn new_hourly(source : GetMeteringDataTimeSeriesResponseResult) -> Self {
        let mut payload = Self::new(Granularity::Hourly);

        let ts = source.my_energy_data_market_document.time_series.last().unwrap();

        for period in &ts.period {
            let parsed_date = period.time_interval.end.clone().parse::<chrono::DateTime<chrono::Utc>>().unwrap();
            let base_key = parsed_date.format("%m/%d/%Y").to_string();
            for point in &period.point {
                let parsed = point.position.parse::<i32>().unwrap() - 1;
                let pos = {
                    if parsed >= 10 {
                        format!("{}:00", parsed)
                    } else {
                        format!("0{}:00", parsed)
                    }
                };
                let key = format!("{} {}", base_key, pos);

                payload.data.insert(key, point.out_quantity_quantity.parse().unwrap());
            }
        }

        return payload
    }

    pub fn new_daily(source : GetMeteringDataTimeSeriesResponseResult) -> Self {
        let mut payload = Self::new(Granularity::Daily);

        let ts = source.my_energy_data_market_document.time_series.last().unwrap();

        for period in &ts.period {
            let parsed_date = period.time_interval.end.clone().parse::<chrono::DateTime<chrono::Utc>>().unwrap();
            let key = parsed_date.format("%m/%d/%Y").to_string();
            let mut total : f64 = 0.0;
            for point in &period.point {
                total = total + point.out_quantity_quantity.parse::<f64>().unwrap();
            }
            payload.data.insert(key, f64::trunc(total * 100.0) / 100.0);
        }

        return payload
    }

}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Granularity {
    Hourly,
    Daily,
    Monthly
}