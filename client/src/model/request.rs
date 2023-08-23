use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetMeteringPointChargesRequest {
    pub metering_points: MeteringPoints
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetMeteringDataTimeSeriesRequest {
    pub metering_points: MeteringPoints
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MeteringPoints {
    pub metering_point: Vec<String>
}

