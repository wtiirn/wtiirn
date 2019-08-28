use crate::model::{Coordinates, TidePrediction};
use serde::Deserialize;
use uuid::Uuid;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

/// The generic information about a tide station, divorced
/// from meta-data like "how are the tides predicted" and
/// "who's responsible for this station".
#[derive(Debug, PartialEq, Clone)]
pub struct Station {
    pub name: String,
    pub coordinates: Coordinates,
    pub id: Uuid,
}

impl Station {
    pub fn generate_id(name: &str, source_id: u32) -> Uuid {
        let name_bytes = format!("{}{}", name, source_id).into_bytes();
        Uuid::new_v5(&Uuid::NAMESPACE_OID, &name_bytes)
    }
}

#[derive(Debug, PartialEq, Clone, Deserialize)]
pub struct PredictionWithId {
    pub station_id: Uuid,
    pub prediction: TidePrediction,
}

static ATKINSON_PREDICTIONS_SRC: &'static str = include_str!("../public/atkinson_predictions.json");
static LAVACA_PREDICTIONS_SRC: &'static str = include_str!("../public/lavaca_predictions.json");

fn parse_predictions(src: &str) -> Vec<PredictionWithId> {
    serde_json::from_str(src).expect("Failure to parse included predictions.json")
}

/// Queryable repository of stations.
pub struct StationCatalogue {
    stations: Vec<Station>,
    predictions: Vec<PredictionWithId>,
}

impl StationCatalogue {
    pub fn empty() -> Self {
        StationCatalogue {
            stations: vec![],
            predictions: vec![],
        }
    }
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
            id: Uuid::parse_str("0dd4be22-22f2-4d3c-9950-54c8a3d52b12").expect("uuid fail"),
        };

        let port_lavaca = Station {
            name: "Port Lavaca".to_string(),
            coordinates: Coordinates {
                lat: 28.6406,
                lon: -96.6098,
            },
            id: Uuid::parse_str("946cc0d2-c976-423c-bb1e-89a400fbf8c1").expect("uuid fail"),
        };

        let point_atkinson_predictions = load_predictions_from_json_at_path("public/atkinson_predictions.json")
            .expect("failed to load atkinson predictions from file");
        let port_lavaca_predictions = load_predictions_from_json_at_path("public/lavaca_predictions.json")
            .expect("failed to load lavaca predictions from file");

        let predictions = point_atkinson_predictions.into_iter().chain(port_lavaca_predictions.into_iter())
            .collect();

        StationCatalogue {
            stations: vec![point_atkinson, port_lavaca],
            predictions,
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
        let id = Uuid::new_v4();
        let station = Station {
            name: name.to_owned(),
            coordinates: *coordinates,
            id,
        };
        self.stations.push(station);
        self.predictions
            .append(&mut predictions_with_id(id, predictions.to_vec()));
    }

    pub fn predictions_for_station(&self, station: &Station) -> Option<Vec<TidePrediction>> {
        Some(
            self.predictions
                .iter()
                .filter(|x| x.station_id == station.id)
                .map(|x| x.prediction)
                .collect(),
        )
    }
}

fn load_predictions_from_json_at_path(path_str: &str) -> Result<Vec<PredictionWithId>, Box<dyn Error>> {
    let path = Path::new(path_str);
    let mut string = String::new();
    let mut file = File::open(&path)?;
    file.read_to_string(&mut string)?;
    Ok(parse_predictions(&string))
}

fn predictions_with_id(
    station_id: Uuid,
    predictions: Vec<TidePrediction>,
) -> Vec<PredictionWithId> {
    predictions
        .into_iter()
        .map(|prediction| PredictionWithId {
            station_id,
            prediction,
        })
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;
    use chrono::prelude::*;
    use uom::si::f64::*;
    use uom::si::length::meter;

    #[test]
    fn test_adding_and_finding_stations() {
        let mut catalogue = StationCatalogue::empty();
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
        let mut catalogue = StationCatalogue::empty();
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
        parse_predictions(ATKINSON_PREDICTIONS_SRC);
        parse_predictions(LAVACA_PREDICTIONS_SRC);
    }

    mod id_generation {
        use super::*;
        #[test]
        fn it_should_generate_the_same_id_for_the_same_input() {
            let name = "A Tide Station";
            let source_id = 123456;

            let id1 = Station::generate_id(name, source_id);
            let id2 = Station::generate_id(name, source_id);
            assert_eq!(id1, id2);
        }

        #[test]
        fn it_should_generate_different_ids_for_different_inputs() {
            let name = "A Tide Station";
            let source_id = 123456;

            let name2 = "A different station";

            let id1 = Station::generate_id(name, source_id);
            let id2 = Station::generate_id(name2, source_id);
            assert_ne!(id1, id2);

            let source_id2 = 654321;
            let id3 = Station::generate_id(name, source_id2);
            assert_ne!(id1, id3);
            assert_ne!(id2, id3);
        }
    }
}
