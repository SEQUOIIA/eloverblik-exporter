pub mod fs;

use eloverblik_client::model::response::{GetMeteringDataTimeSeriesResponseResult};
use crate::error::Result;
use crate::model::UsageTimeSeries;

pub trait Store {
    fn put(&self, doc : StoreType) -> Result<()>;
}

pub enum StoreType {
    String{ key: String, value : String },
    MeterDataTimeSeries(GetMeteringDataTimeSeriesResponseResult),
    UsageTimeSeries{ key: String, value : UsageTimeSeries}
}