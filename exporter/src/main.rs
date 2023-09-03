use eloverblik_client::cache::DiskCache;
use eloverblik_client::model::request::{GetMeteringDataTimeSeriesRequest, GetMeteringPointChargesRequest, MeteringPoints};
use energidataservice_client::model::request::ElSpotPricesRequest;
use crate::model::UsageTimeSeries;
use crate::store::fs::FsStore;
use crate::store::{Store, StoreType};

mod config;
mod error;
mod store;
mod model;

#[tokio::main]
async fn main() {
    let conf = config::load_conf().unwrap();
    setup_tracing(get_trace_level(&conf.log_level));
    let client = eloverblik_client::new_builder()
        .add_config(eloverblik_client::Config {
            refresh_token: conf.eloverblik_refresh_token
        })
        .add_cache(Box::new(DiskCache {
            path: "eloverblik-cache".to_owned(),
            default_expiration_time_in_secs: 3600
        }))
        .build();

    let eds_client = energidataservice_client::new_builder()
        .build();

    let metering_points = client.get_metering_points().await.unwrap();
    let first_meter_point = metering_points.result.last().unwrap();

    let timeseries = client.get_metering_data_timeseries(GetMeteringDataTimeSeriesRequest {
        metering_points: MeteringPoints {
            metering_point: vec![first_meter_point.metering_point_id.clone()]
        }
    }, "2023-08-01", "2023-08-31", "Hour").await.unwrap();

    let metering_point_charges = client.get_metering_point_charges(GetMeteringPointChargesRequest {
        metering_points: MeteringPoints {
            metering_point: vec![first_meter_point.metering_point_id.clone()]
        }
    }).await.unwrap();
    let first_meter_point_charges = metering_point_charges.result.last().unwrap();

    let mut stores : Vec<Box<dyn Store>> = Vec::new();
    stores.push(Box::new(FsStore {
        path: "eloverblik-store".to_owned()
    }));

    let prices = eds_client.get_elspotprices(ElSpotPricesRequest {
        limit: Some(0),
        timezone: Some("UTC".to_owned()),
        start: Some("2023-08-01".to_owned()),
        end: Some("2023-08-31".to_owned()),
        filter: Some("{\"PriceArea\":[\"DK2\"]}".to_owned()),
        sort: Some("HourUTC".to_owned()),
    }).await.unwrap();

    let prices_map = prices.clone().into_records_as_map();


    let hourly = UsageTimeSeries::new_hourly(timeseries.result.last().unwrap().clone(), &prices_map, &first_meter_point_charges);
    let daily = UsageTimeSeries::new_daily(timeseries.result.last().unwrap().clone(), &prices_map, &first_meter_point_charges);



    for store in &stores {
        store.put(StoreType::MeterDataTimeSeries(timeseries.result.last().unwrap().clone())).unwrap();
        store.put(StoreType::UsageTimeSeries {key: "hourly".to_owned(), value: hourly.clone()}).unwrap();
        store.put(StoreType::UsageTimeSeries {key: "daily".to_owned(), value: daily.clone()}).unwrap();
        store.put(StoreType::String {key: "prices".to_owned(), value: serde_json::to_string(&prices).unwrap()}).unwrap();
        store.put(StoreType::String {key: "meteringpoint_charges.json".to_owned(), value: serde_json::to_string(&metering_point_charges).unwrap()}).unwrap();
    }

}


fn setup_tracing(log_level : Option<tracing::Level>) {
    let max_level = log_level.unwrap_or(tracing::Level::TRACE);
    let filter = tracing_subscriber::EnvFilter::from_default_env()
        .add_directive(format!("eloverblik_exporter={}", max_level.as_str().to_lowercase()).parse().unwrap())
        .add_directive(format!("eloverblik_client={}", max_level.as_str().to_lowercase()).parse().unwrap())
        .add_directive("reqwest=info".parse().unwrap())
        .add_directive("mio=info".parse().unwrap())
        .add_directive("want=info".parse().unwrap())
        .add_directive("actix_web=info".parse().unwrap())
        .add_directive("hyper=info".parse().unwrap());


    let subscriber = tracing_subscriber::fmt()
        .with_max_level(max_level)
        .with_env_filter(filter)
        .finish();
    tracing_log::LogTracer::init().unwrap();
    tracing::subscriber::set_global_default(subscriber).unwrap();
}

fn get_trace_level(input : &str) -> Option<tracing::Level> {
    return match input.to_uppercase().as_str() {
        "TRACE" => Some(tracing::Level::TRACE),
        "DEBUG" => Some(tracing::Level::DEBUG),
        "INFO" => Some(tracing::Level::INFO),
        "WARN" => Some(tracing::Level::WARN),
        "ERROR" => Some(tracing::Level::ERROR),
        _ => None
    }
}
