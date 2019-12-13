use chrono::prelude::*;
use serde::Deserialize;
use std::fmt;
use uom::si::f64::*;
use uom::si::length::meter;

pub static TIME_FORMAT: &str = "%_I:%M %p on %a %b %e, %Y";

#[derive(Debug, PartialEq, Clone, Copy, Deserialize)]
pub struct TidePrediction {
    pub tide: Length,
    pub time: DateTime<FixedOffset>,
}

impl TidePrediction {
    pub fn is_before(&self, time: DateTime<FixedOffset>) -> bool {
        self.time < time
    }

    pub fn set_offset(&mut self, offset: FixedOffset) {
        self.time = self.time.with_timezone(&offset);
    }

    pub fn as_table_row(&self) -> String {
        format!(
            "<td>{}m</td><td>{}</td>",
            self.tide.get::<meter>(),
            self.time.format(TIME_FORMAT)
        )
    }
}

impl fmt::Display for TidePrediction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{} meters above the <a href='https://en.wikipedia.org/wiki/Chart_datum'>datum</a> at {}",
            self.tide.get::<meter>(),
            self.time.format(TIME_FORMAT)
        )
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct TidePredictionPair {
    pub next: TidePrediction,
    pub prev: TidePrediction,
}

impl TidePredictionPair {
    pub fn headline(&self) -> String {
        if self.tide_is_coming_in() {
            "The tide is coming in!".into()
        } else {
            "The tide is going out!".into()
        }
    }

    pub fn detail(&self) -> String {
        if self.tide_is_coming_in() {
            format!(
                "Low tide was {}, High tide will be {}",
                self.prev, self.next
            )
        } else {
            format!(
                "High tide was {}, Low tide will be {}",
                self.prev, self.next
            )
        }
    }

    fn prev_tide_type(&self) -> &str {
        if self.tide_is_coming_in() {
            "Low"
        } else {
            "High"
        }
    }

    fn next_tide_type(&self) -> &str {
        if !self.tide_is_coming_in() {
            "Low"
        } else {
            "High"
        }
    }

    pub fn as_table(&self) -> String {
        format!(
            "<table>
            <thead>
            <th></th><th>Tide</th><th><a target='_blank' href='https://en.wikipedia.org/wiki/Chart_datum'>Level</a></th><th>Time</th>
            </thead>
            <tr><td>Previous Tide</td><td>{}</td>{}</tr>
            <tr><td>Next Tide</td><td>{}</td>{}</tr>
            </table>",
            self.prev_tide_type(),
            self.prev.as_table_row(),
            self.next_tide_type(),
            self.next.as_table_row(),
            )
    }

    pub fn set_offset(&mut self, offset: FixedOffset) -> Self {
        self.next.set_offset(offset);
        self.prev.set_offset(offset);
        *self
    }

    pub fn tide_is_coming_in(&self) -> bool {
        self.next.tide > self.prev.tide
    }
}

#[derive(Debug, PartialEq, Copy, Clone, Deserialize)]
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
