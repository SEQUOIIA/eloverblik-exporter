use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TokenResponse {
    pub result : String
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetMeteringPointsResponse {
    pub result: Vec<GetMeteringPointsResponseResult>
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetMeteringPointsResponseResult {
    pub street_code: String,
    pub street_name: String,
    pub building_number: String,
    pub floor_id: String,
    pub room_id: String,
    pub city_sub_division_name: Option<serde_json::Value>,
    pub municipality_code: String,
    pub location_description: String,
    pub settlement_method: String,
    pub meter_reading_occurrence: String,
    pub first_consumer_party_name: String,
    pub second_consumer_party_name: Option<serde_json::Value>,
    pub meter_number: String,
    pub consumer_start_date: String,
    pub metering_point_id: String,
    #[serde(rename = "typeOfMP")]
    pub type_of_mp: String,
    pub balance_supplier_name: String,
    pub postcode: String,
    pub city_name: String,
    pub has_relation: bool,
    #[serde(rename = "consumerCVR")]
    pub consumer_cvr: Option<serde_json::Value>,
    #[serde(rename = "dataAccessCVR")]
    pub data_access_cvr: Option<serde_json::Value>,
    pub child_metering_points: Vec<Option<serde_json::Value>>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetMeteringPointChargesResponse {
    pub result: Vec<GetMeteringPointChargesResponseResult>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetMeteringPointChargesResponseResult {
    pub result: GetMeteringPointChargesResponseResultResult,
    pub success: bool,
    pub error_code: i64,
    pub error_text: String,
    pub id: String,
    pub stack_trace: Value,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetMeteringPointChargesResponseResultResult {
    pub fees: Vec<Value>,
    pub metering_point_id: String,
    pub subscriptions: Vec<GetMeteringPointChargesResponseResultResultSubscription>,
    pub tariffs: Vec<GetMeteringPointChargesResponseResultResultTariff>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetMeteringPointChargesResponseResultResultSubscription {
    pub price: f64,
    pub quantity: i64,
    pub name: String,
    pub description: String,
    pub owner: String,
    pub valid_from_date: String,
    pub valid_to_date: Value,
    pub period_type: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetMeteringPointChargesResponseResultResultTariff {
    pub prices: Vec<GetMeteringPointChargesResponseResultResultTariffPrice>,
    pub name: String,
    pub description: String,
    pub owner: String,
    pub valid_from_date: String,
    pub valid_to_date: Value,
    pub period_type: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetMeteringPointChargesResponseResultResultTariffPrice {
    pub position: String,
    pub price: f64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetMeteringDataTimeSeriesResponse {
    pub result: Vec<GetMeteringDataTimeSeriesResponseResult>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetMeteringDataTimeSeriesResponseResult {
    #[serde(rename = "MyEnergyData_MarketDocument")]
    pub my_energy_data_market_document: MyEnergyDataMarketDocument,
    pub success: bool,
    pub error_code: i64,
    pub error_text: String,
    pub id: String,
    pub stack_trace: Value,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MyEnergyDataMarketDocument {
    #[serde(rename = "mRID")]
    pub m_rid: String,
    pub created_date_time: String,
    #[serde(rename = "sender_MarketParticipant.name")]
    pub sender_market_participant_name: String,
    #[serde(rename = "sender_MarketParticipant.mRID")]
    pub sender_market_participant_m_rid: SenderMarketParticipantMRid,
    #[serde(rename = "period.timeInterval")]
    pub period_time_interval: PeriodTimeInterval,
    #[serde(rename = "TimeSeries")]
    pub time_series: Vec<TimeSeries>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SenderMarketParticipantMRid {
    pub coding_scheme: Value,
    pub name: Value,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PeriodTimeInterval {
    pub start: String,
    pub end: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TimeSeries {
    #[serde(rename = "mRID")]
    pub m_rid: String,
    pub business_type: String,
    pub curve_type: String,
    #[serde(rename = "measurement_Unit.name")]
    pub measurement_unit_name: String,
    #[serde(rename = "MarketEvaluationPoint")]
    pub market_evaluation_point: MarketEvaluationPoint,
    #[serde(rename = "Period")]
    pub period: Vec<Period>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MarketEvaluationPoint {
    #[serde(rename = "mRID")]
    pub m_rid: MRid,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MRid {
    pub coding_scheme: String,
    pub name: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Period {
    pub resolution: String,
    pub time_interval: TimeInterval,
    #[serde(rename = "Point")]
    pub point: Vec<Point>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TimeInterval {
    pub start: String,
    pub end: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Point {
    pub position: String,
    #[serde(rename = "out_Quantity.quantity")]
    pub out_quantity_quantity: String,
    #[serde(rename = "out_Quantity.quality")]
    pub out_quantity_quality: String,
}
