use eloverblik_client::cache::DiskCache;
use eloverblik_client::model::request::{GetMeteringDataTimeSeriesRequest, MeteringPoints};

mod config;
mod error;

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

    let metering_points = client.get_metering_points().await.unwrap();
    let first_meter_point = metering_points.result.last().unwrap();

    let timeseries = client.get_metering_data_timeseries(GetMeteringDataTimeSeriesRequest {
        metering_points: MeteringPoints {
            metering_point: vec![first_meter_point.metering_point_id.clone()]
        }
    }, "2023-08-01", "2023-08-20", "Hour").await.unwrap();

    println!("{:?}", timeseries);
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
