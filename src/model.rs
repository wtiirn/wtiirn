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
    pub next: TidePrediction,
    pub prev: TidePrediction,
}

impl fmt::Display for TidePredictionPair {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        print_pair(f, self.next, self.prev)
    }
}

fn print_pair(f: &mut fmt::Formatter, n: TidePrediction, p: TidePrediction) -> fmt::Result {
    write!(f, "{}", headline(n, p) + " " + &detail(n, p))
}

impl TidePredictionPair {
    pub fn headline(&self) -> String {
        headline(self.next, self.prev)
    }

    pub fn detail(&self) -> String {
        detail(self.next, self.prev)
    }
}

fn headline(n: TidePrediction, p: TidePrediction) -> String {
    if n.tide > p.tide {
        "The tide is coming in!".into()
    } else {
        "The tide is going out!".into()
    }
}

fn detail(n: TidePrediction, p: TidePrediction) -> String {
    if n.tide > p.tide {
        format!("Low tide was {}, High tide will be {}", p, n)
    } else {
        format!("High tide was {}, Low tide will be {}", p, n)
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Coordinates {
    pub lat: f64,
    pub lon: f64,
}

impl Coordinates {
    /// Lat and Lon in radians.
    pub fn to_radians(&self) -> (f64, f64) {
        (
            self.lat * std::f64::consts::PI / 180.0,
            self.lon * std::f64::consts::PI / 180.0,
        )
    }
}

