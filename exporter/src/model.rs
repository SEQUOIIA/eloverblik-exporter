use std::collections::{BTreeMap, HashMap};
use serde::{Deserialize, Serialize};
use eloverblik_client::model::response::{GetMeteringDataTimeSeriesResponseResult, GetMeteringPointChargesResponseResult};
use energidataservice_client::model::response::Record;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UsageTimeSeries {
    pub data : BTreeMap<String, Data>,
    pub granularity : Granularity
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Data {
    pub wh : f64,
    pub cost : f64
}

impl UsageTimeSeries {
    pub fn new(gran : Granularity) -> Self {
        UsageTimeSeries {
            granularity:  gran,
            data: BTreeMap::new()
        }
    }

    pub fn new_hourly(source : GetMeteringDataTimeSeriesResponseResult, prices : &HashMap<String, Record>, meter_point_charges : &GetMeteringPointChargesResponseResult) -> Self {
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
                let price = match prices.get(&key) {
                    None => 0.0,
                    Some(val) => {
                        if point.out_quantity_quantity.eq("0.0") {
                            0.0
                        } else {
                            meter_point_charges.result.get_full_price(val.as_kwh_price_eur(), point.position.clone(), point.out_quantity_quantity.parse().unwrap())
                        }
                    }
                };

                payload.data.insert(key, Data {
                    wh: point.out_quantity_quantity.parse().unwrap(),
                    cost: price
                });
            }
        }

        return payload
    }

    pub fn new_daily(source : GetMeteringDataTimeSeriesResponseResult, prices : &HashMap<String, Record>, meter_point_charges : &GetMeteringPointChargesResponseResult) -> Self {
        let mut payload = Self::new(Granularity::Daily);

        let ts = source.my_energy_data_market_document.time_series.last().unwrap();

        for period in &ts.period {
            let parsed_date = period.time_interval.end.clone().parse::<chrono::DateTime<chrono::Utc>>().unwrap();
            let key = parsed_date.format("%m/%d/%Y").to_string();
            let mut total : f64 = 0.0;
            let mut total_price : f64 = 0.0;
            for point in &period.point {
                let parsed = point.position.parse::<i32>().unwrap() - 1;
                let pos = {
                    if parsed >= 10 {
                        format!("{}:00", parsed)
                    } else {
                        format!("0{}:00", parsed)
                    }
                };

                let price_key = format!("{} {}", key, pos);
                let price = match prices.get(&price_key) {
                    None => 0.0,
                    Some(val) => {
                        if point.out_quantity_quantity.eq("0.0") {
                            0.0
                        } else {
                            meter_point_charges.result.get_full_price(val.as_kwh_price_eur(), point.position.clone(), point.out_quantity_quantity.parse().unwrap())
                        }
                    }
                };

                total = total + point.out_quantity_quantity.parse::<f64>().unwrap();
                total_price = total_price + price;
            }
            payload.data.insert(key, Data {
                wh: f64::trunc(total * 100.0) / 100.0,
                cost: total_price
            });
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