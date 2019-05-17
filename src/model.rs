use chrono::prelude::*;
use serde::Deserialize;
use std::fmt;

pub static TIME_FORMAT: &str = "%I:%M%P on %a %b %e, %Y";

#[derive(Debug, PartialEq, Clone, Copy, Deserialize)]
pub struct TidePrediction {
    pub tide: f32,
    pub time: DateTime<FixedOffset>,
}

impl TidePrediction {
    pub fn delta_from(&self, time: DateTime<FixedOffset>) -> i64 {
        self.time.timestamp() - time.timestamp()
    }

    pub fn is_before(&self, time: DateTime<FixedOffset>) -> bool {
        self.time < time
    }
}

impl fmt::Display for TidePrediction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{} meters above the datum at {}",
            self.tide,
            self.time.format(TIME_FORMAT)
        )
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct TidePredictionPair {
    pub next: Option<TidePrediction>,
    pub prev: Option<TidePrediction>,
}

impl fmt::Display for TidePredictionPair {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match (self.next, self.prev) {
            (None, _) | (_, None) => write!(f, "Incomplete pair! {:?}", self),
            (Some(n), Some(p)) => print_pair(f, n, p),
        }
    }
}

fn print_pair(f: &mut fmt::Formatter, n: TidePrediction, p: TidePrediction) -> fmt::Result {
    if n.tide > p.tide {
        write!(
            f,
            "The tide is coming in! Low tide was {}, High tide will be {}",
            p, n
        )
    } else {
        write!(
            f,
            "The tide is going out! High tide was {}, Low tide will be {}",
            p, n
        )
    }
}
