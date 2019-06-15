use chrono::prelude::*;
use serde::Deserialize;
use std::convert::TryFrom;
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

impl TryFrom<Option<&str>> for Coordinates {
    type Error = ();
    fn try_from(st: Option<&str>) -> Result<Coordinates, ()> {
        match st {
            Some(s) => Coordinates::try_from(s),
            _ => Err(()),
        }
    }
}

impl TryFrom<&str> for Coordinates {
    type Error = ();
    fn try_from(s: &str) -> Result<Coordinates, ()> {
        let mut maybe_lat = None;
        let mut maybe_lon = None;
        let tuples = s
            .split("&")
            .map(|x| (x.split("=").next(), x.split("=").last()));

        for (name, value) in tuples {
            match (name, value) {
                (Some(n), Some(v)) if n == "lat" => maybe_lat = v.parse::<f64>().ok(),
                (Some(n), Some(v)) if n == "lon" => maybe_lon = v.parse::<f64>().ok(),
                _ => (),
            }
        }

        match (maybe_lat, maybe_lon) {
            (Some(lat), Some(lon)) => Ok(Coordinates { lat, lon }),
            _ => Err(()),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse_from_a_string() {
        assert_eq!(
            Coordinates::try_from(Some("lat=123.456&lon=54.321")),
            Ok(Coordinates {
                lat: 123.456,
                lon: 54.321
            })
        );
    }
}
