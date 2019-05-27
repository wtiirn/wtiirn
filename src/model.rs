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
    write!(f, "{}", headline(n, p) + " " + &detail(n, p))
}

impl TidePredictionPair {
    pub fn headline(&self) -> String {
        match (self.next, self.prev) {
            (None, _) | (_, None) => format!("Incomplete pair! {:?}", self),
            (Some(n), Some(p)) => headline(n, p),
        }
    }

    pub fn detail(&self) -> String {
        match (self.next, self.prev) {
            (None, _) | (_, None) => format!("Incomplete pair! {:?}", self),
            (Some(n), Some(p)) => detail(n, p),
        }
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
