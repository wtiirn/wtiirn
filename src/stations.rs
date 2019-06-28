use crate::model::{Coordinates, TidePrediction};
use std::collections::HashMap;

/// The generic information about a tide station, divorced
/// from meta-data like "how are the tides predicted" and
/// "who's responsible for this station".
#[derive(Debug, PartialEq, Clone)]
pub struct Station {
    pub name: String,
    pub coordinates: Coordinates,
    id: u64,
}

static ATKINSON_PREDICTIONS_SRC: &'static str = include_str!("atkinson_predictions.json");
static LAVACA_PREDICTIONS_SRC: &'static str = include_str!("lavaca_predictions.json");

fn parse_predictions(src: &str) -> Vec<TidePrediction> {
    serde_json::from_str(src).expect("Failure to parse included predictions.json")
}

/// Queryable repository of stations.
pub struct StationCatalogue {
    stations: Vec<Station>,
    station_predictions: HashMap<u64, Vec<TidePrediction>>,
}

impl StationCatalogue {
    /// Initialize a catalogue from a suitable data source.
    /// Panics if there isn't at least one tide station in
    /// the initialized catalogue.
    pub fn load() -> Self {
        let point_atkinson = Station {
            name: "Point Atkinson".to_owned(),
            coordinates: Coordinates {
                lat: 49.336,
                lon: -123.262,
            },
            id: 1,
        };

        let port_lavaca = Station {
            name: "Port Lavaca".to_string(),
            coordinates: Coordinates {
                lat: 28.6406,
                lon: -96.6098,
            },
            id: 2,
        };

        let point_atkinson_predictions = parse_predictions(ATKINSON_PREDICTIONS_SRC);
        let port_lavaca_predictions = parse_predictions(LAVACA_PREDICTIONS_SRC);

        let station_predictions = [
            (point_atkinson.id, point_atkinson_predictions),
            (port_lavaca.id, port_lavaca_predictions),
        ]
        .iter()
        .cloned()
        .collect();

        StationCatalogue {
            stations: vec![point_atkinson, port_lavaca],
            station_predictions,
        }
    }

    /// Find the station nearest to the given coordinates.
    pub fn find_near(&self, coordinates: &Coordinates) -> &Station {
        use crate::compute::gcd::great_circle_distance;
        let cmp_distance = |s1: &&Station, s2: &&Station| {
            let d1 = great_circle_distance(&s1.coordinates, coordinates);
            let d2 = great_circle_distance(&s2.coordinates, coordinates);
            d1.partial_cmp(&d2).expect("Distances shouldn't be NaN")
        };
        self.stations
            .iter()
            .min_by(cmp_distance)
            .expect("StationCatalogue has at least one station, so there must be a minimum")
    }

    /// Add a station's data to this catalogue, assigning it an appropriate unique id.
    fn add(&mut self, name: &str, coordinates: &Coordinates, predictions: &[TidePrediction]) {
        let id = self.stations.len() as u64;
        let station = Station {
            name: name.to_owned(),
            coordinates: *coordinates,
            id,
        };
        self.stations.push(station);
        self.station_predictions.insert(id, predictions.to_vec());
    }

    pub fn predictions_for_station(&self, station: &Station) -> Option<&[TidePrediction]> {
        self.station_predictions
            .get(&station.id)
            .map(|x| x.as_slice())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use chrono::prelude::*;
    use uom::si::f64::*;
    use uom::si::length::meter;

    #[test]
    fn test_adding_and_finding_stations() {
        let mut catalogue = StationCatalogue {
            stations: vec![],
            station_predictions: HashMap::new(),
        };
        catalogue.add(
            "Point Atkinson",
            &Coordinates {
                lat: 49.336,
                lon: -123.262,
            },
            &vec![],
        );
        catalogue.add(
            "Port Lavaca",
            &Coordinates {
                lat: 28.6406,
                lon: -96.6098,
            },
            &vec![],
        );
        let aus = Coordinates {
            lat: 30.194444,
            lon: -97.67,
        };
        let yvr = Coordinates {
            lat: 49.194722,
            lon: -123.183889,
        };
        assert_eq!(catalogue.find_near(&aus).name, "Port Lavaca");
        assert_eq!(catalogue.find_near(&yvr).name, "Point Atkinson");
    }

    #[test]
    fn test_finding_predictions_for_station() {
        let mut catalogue = StationCatalogue {
            stations: vec![],
            station_predictions: HashMap::new(),
        };
        catalogue.add(
            "Point Atkinson",
            &Coordinates {
                lat: 49.336,
                lon: -123.262,
            },
            &vec![TidePrediction {
                tide: Length::new::<meter>(2.0),
                time: FixedOffset::west(8 * 3600)
                    .ymd(2019, 05, 14)
                    .and_hms(0, 0, 0),
            }],
        );

        catalogue.add(
            "Port Lavaca",
            &Coordinates {
                lat: 28.6406,
                lon: -96.6098,
            },
            &vec![],
        );

        let yvr = Coordinates {
            lat: 49.194722,
            lon: -123.183889,
        };

        assert_eq!(
            catalogue
                .predictions_for_station(catalogue.find_near(&yvr))
                .expect("No predictions found")
                .len(),
            1
        );
    }

    #[test]
    fn test_parsing_predictions_file() {
        parse_predictions(PREDICTIONS_SRC);
    }

}
