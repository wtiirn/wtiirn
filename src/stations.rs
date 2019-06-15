use crate::model;

/// The generic information about a tide station, divorced
/// from meta-data like "how are the tides predicted" and
/// "who's responsible for this station".
#[derive(Debug, PartialEq)]
pub struct Station {
    pub name: String,
    pub coords: model::Coordinates,
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
    pub fn find_near(&self, location: &model::Coordinates) -> &Station {
        unimplemented!()
    }

    /// Add a station's data to this catalogue, assigning it an appropriate unique id.
    fn add(&mut self, name: &str, coords: &model::Coordinates) {
        unimplemented!()
    }
}
