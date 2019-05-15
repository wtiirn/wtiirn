use chrono::prelude::*;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct TidePrediction {
    pub tide: f32,
    pub time: DateTime<FixedOffset>,
}
