use crate::model;

/// The generic information about a tide station, divorced
/// from meta-data like "how are the tides predicted" and
/// "who's responsible for this station".
#[derive(Debug, PartialEq)]
pub struct Station {
    pub name: String,
    pub coordinates: model::Coordinates,
    id: u64,
}

/// Queryable repository of stations.
pub struct StationCatalogue {
    stations: Vec<Station>,
}

impl StationCatalogue {
    /// Initialize a catalogue from a suitable data source.
    /// Panics if there isn't at least one tide station in
    /// the initialized catalogue.
    pub fn load() -> Self {
        // Ensures that there's at least one tide station.
        unimplemented!()
    }

    /// Find the station nearest to the given coordinates.
    pub fn find_near(&self, coordinates: &model::Coordinates) -> &Station {
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
    fn add(&mut self, name: &str, coordinates: &model::Coordinates) {
        let id = self.stations.len() as u64;
        let station = Station {
            name: name.to_owned(),
            coordinates: *coordinates,
            id,
        };
        self.stations.push(station);
    }
}


#[test]
fn test_adding_and_finding_stations() {
    let mut catalogue = StationCatalogue { stations: vec![] };
    catalogue.add(
        "Point Atkinson",
        &model::Coordinates {
            lat: 49.336,
            lon: -123.262,
        },
    );
    catalogue.add(
        "Port Lavaca",
        &model::Coordinates {
            lat: 28.6406,
            lon: -96.6098,
        },
    );
    let aus = model::Coordinates {
        lat: 30.194444,
        lon: -97.67,
    };
    let yvr = model::Coordinates {
        lat: 49.194722,
        lon: -123.183889,
    };
    assert_eq!(catalogue.find_near(&aus).name, "Port Lavaca",);
    assert_eq!(catalogue.find_near(&yvr).name, "Point Atkinson");
}