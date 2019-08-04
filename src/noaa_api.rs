use serde::Deserialize;

pub mod transform;

#[derive(Debug, Deserialize)]
#[serde(rename = "data")]
struct TideData {
    time: String,
    pred: f32,
}

#[derive(Debug, Deserialize)]
#[serde(rename = "item")]
struct Item {
    date: String,
    data: Vec<TideData>,
}

#[derive(Debug, Deserialize)]
struct HighLowValues {
    #[serde(rename = "item")]
    values: Vec<Item>,
}

#[derive(Debug, Deserialize)]
pub struct HighLowAndMetadata {
    stationId: u32,
    stationName: String,
    latitude: f64,
    longitude: f64,
    timeZone: String,
    #[serde(rename = "unit")]
    unit_name: String,
    #[serde(rename = "HighLowValues")]
    values: HighLowValues,
}
