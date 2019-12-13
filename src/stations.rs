use crate::model::{Coordinates, TidePrediction};
use serde::Deserialize;
use std::error::Error;
use std::fs::{self, File};
use std::io::prelude::*;
use std::path::Path;
use uuid::Uuid;

/// The generic information about a tide station, divorced
/// from meta-data like "how are the tides predicted" and
/// "who's responsible for this station".
#[derive(Debug, PartialEq, Clone, Deserialize)]
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
pub struct PredictionsWithId {
    pub station_id: Uuid,
    pub predictions: Vec<TidePrediction>,
}

/// Queryable repository of stations.
pub struct StationCatalogue {
    stations: Vec<Station>,
    predictions: Vec<PredictionsWithId>,
}

impl StationCatalogue {
    pub fn empty() -> Self {
        StationCatalogue {
            stations: vec![],
            predictions: vec![],
        }
    }

    pub fn test() -> Self {
        StationCatalogue {
            stations: vec![Station {
                name: "Test Station".into(),
                coordinates: Coordinates { lat: 0.0, lon: 0.0 },
                id: Uuid::new_v4(),
            }],
            predictions: vec![],
        }
    }

    /// Initialize a catalogue from a suitable data source.
    /// Panics if there isn't at least one tide station in
    /// the initialized catalogue.
    pub fn load() -> Self {
        println!("Initializing Station Catalogue");
        let stations =
            load_stations_from_dir(Path::new("data/stations")).expect("failed to load stations");
        println!("Loaded {} total stations", stations.len());
        let predictions = load_predictions_from_dir(Path::new("data/predictions"))
            .expect("failed to load predcitions");
        println!("Loaded {} prediction collections", predictions.len());

        StationCatalogue {
            stations,
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
    #[allow(dead_code)]
    fn add(&mut self, name: &str, coordinates: &Coordinates, predictions: &[TidePrediction]) {
        let id = Uuid::new_v4();
        let station = Station {
            name: name.to_owned(),
            coordinates: *coordinates,
            id,
        };
        self.stations.push(station);
        self.predictions.push(PredictionsWithId {
            station_id: id,
            predictions: predictions.to_vec(),
        });
    }

    pub fn predictions_for_station(&self, station: &Station) -> Option<Vec<TidePrediction>> {
        Some(
            self.predictions
                .iter()
                .filter(|x| x.station_id == station.id)
                .flat_map(|x| x.predictions.clone())
                .collect(),
        )
    }
}

fn load_stations_from_dir(path: &Path) -> Result<Vec<Station>, Box<dyn Error>> {
    Ok(fs::read_dir(path)?
        .flat_map(|file| load_stations_from_json(&file?.path()))
        .flat_map(|item| item)
        .collect())
}

fn load_stations_from_json(path: &Path) -> Result<Vec<Station>, Box<dyn Error>> {
    let string = read_file(path)?;
    Ok(parse_stations(&string))
}

fn load_predictions_from_dir(path: &Path) -> Result<Vec<PredictionsWithId>, Box<dyn Error>> {
    Ok(fs::read_dir(path)?
        .flat_map(|file| load_predictions_from_json(&file?.path()))
        .flat_map(|item| item)
        .collect())
}

fn load_predictions_from_json(path: &Path) -> Result<Vec<PredictionsWithId>, Box<dyn Error>> {
    let string = read_file(&path)?;
    Ok(parse_predictions(&string))
}

fn read_file(path: &Path) -> Result<String, Box<dyn Error>> {
    let mut string = String::new();
    let mut file = File::open(&path)?;
    file.read_to_string(&mut string)?;
    Ok(string)
}

fn parse_predictions(src: &str) -> Vec<PredictionsWithId> {
    parse_vec_of_values(src)
        .unwrap_or_else(|e| {
            println!("Unable to parse string to vec of Values: {:?}", e);
            vec![]
        })
        .into_iter()
        .filter_map(|v| {
            let preds = serde_json::from_value(v);
            if preds.is_err() {
                println!(
                    "Could not parse Value as PredictionsWithId instance: {:?}",
                    preds
                );
            }
            preds.ok()
        })
        .collect()
}

fn parse_stations(src: &str) -> Vec<Station> {
    parse_vec_of_values(src)
        .unwrap_or_else(|e| {
            println!("Unable to parse string to vec of Values: {:?}", e);
            vec![]
        })
        .into_iter()
        .filter_map(|v| {
            let station = serde_json::from_value(v);
            if station.is_err() {
                println!("Could not parse Value as Station instance: {:?}", station);
            }
            station.ok()
        })
        .collect()
}

fn parse_vec_of_values(src: &str) -> Result<Vec<serde_json::Value>, serde_json::error::Error> {
    serde_json::from_str(src)
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

    mod parsing {
        use super::*;
        #[test]
        fn it_should_handle_broken_files() {
            let path = Path::new("test_data/stations/broken_files");
            let stations = load_stations_from_dir(&path);

            assert_eq!(stations.is_ok(), true);
            let stations = stations.unwrap();
            assert_eq!(stations.len(), 1);
            assert_eq!(stations[0].name, "Point Atkinson");
        }

        #[test]
        fn it_should_load_the_canadian_stations_file_without_error() {
            let path = Path::new("test_data/stations/good");
            let stations = load_stations_from_dir(&path);

            assert_eq!(stations.is_ok(), true);
            let stations = stations.unwrap();
            assert_eq!(stations.len(), 873);
        }

        #[test]
        fn it_should_load_known_good_predictions_without_error() {
            let path = Path::new("test_data/predictions/good");
            let preds = load_predictions_from_dir(path);

            assert_eq!(preds.is_ok(), true);
            let preds = preds.unwrap();
            assert_eq!(preds.len(), 1);
            assert_eq!(preds[0].predictions.len(), 495);
        }
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
