use chrono::prelude::*;

#[derive(Debug)]
pub struct TidePrediction {
    pub tide: f32,
    pub time: DateTime<FixedOffset>,
}
