use chrono::prelude::*;
use chrono_humanize::HumanTime;
use serde::Deserialize;
use uom::si::f64::*;
use uom::si::length::{centimeter, kilometer, meter};

use crate::compute;
use crate::model::{Coordinates, TidePredictionPair, TIME_FORMAT};
use crate::stations::{Station, StationCatalogue};

static POINT_ATKINSON: Coordinates = Coordinates {
    lat: 49.3299,
    lon: -123.2650,
};

#[derive(Deserialize, Clone, Copy, Debug)]
pub struct HomePageParams {
    lat: Option<f64>,
    lon: Option<f64>,
    #[serde(alias = "offset")]
    offset_in_minutes: Option<i32>,
}

impl HomePageParams {
    fn get_coords(&self) -> Option<Coordinates> {
        match (self.lat, self.lon) {
            (Some(lat), Some(lon)) => Some(Coordinates { lat, lon }),
            _ => None,
        }
    }
}

pub struct HomePageViewModel {
    current_time: DateTime<FixedOffset>,
    current_location: Option<Coordinates>,
    prediction_pair: Option<TidePredictionPair>,
    station: Station,
}

impl HomePageViewModel {
    /// Collect the information necessary for rendering the home page based on a request's
    /// location and the station catalogue that was loaded at startup.
    pub fn new(stn_catalogue: &StationCatalogue, params: &Option<HomePageParams>) -> Self {
        let offset_in_minutes = params.and_then(|x| x.offset_in_minutes).unwrap_or(8 * 60);
        let offset = FixedOffset::west(offset_in_minutes * 60);
        let current_time = Local::now().with_timezone(&offset);

        let coords = params.and_then(|x| x.get_coords());
        let station = stn_catalogue.find_near(&coords.unwrap_or_else(|| POINT_ATKINSON));
        let predictions = stn_catalogue.predictions_for_station(&station);
        let prediction_pair = predictions
            .and_then(|preds| compute::find::nearest_pair(&preds, current_time))
            .map(|mut x| x.set_offset(offset));

        HomePageViewModel {
            current_time,
            current_location: coords,
            prediction_pair,
            station: station.clone(),
        }
    }

    fn headline(&self) -> String {
        match self.prediction_pair {
            Some(p) => p.headline(),
            _ => "No Tide Information".into(),
        }
    }

    fn detail(&self) -> String {
        match self.prediction_pair {
            Some(p) => p.as_table(),
            _ => "".into(),
        }
    }

    fn distance_from_station(&self) -> Length {
        match self.current_location {
            None => Length::new::<meter>(0.0),
            Some(c) => compute::gcd::great_circle_distance(&c, &self.station.coordinates),
        }
    }

    fn km_from_station(&self) -> f64 {
        self.distance_from_station().get::<kilometer>()
    }

    fn station_info(&self) -> String {
        let mut info = format!("The tide station used is <b>{}</b>", self.station.name);
        if self.current_location.is_some() {
            info += &format!(
                " which is <b>{:.2}</b> KM from your current location",
                self.km_from_station()
            );
        }
        info
    }

    /// Constructs a natural language sentence explaining the current tide status, include direction,
    /// amount, and timing.
    fn current_level(&self) -> String {
        if let Some(pair) = self.prediction_pair {
            let current_level = compute::find::approximate_current_level(&pair, &self.current_time);
            let change = pair.next.tide - current_level;
            let human_time = HumanTime::from(pair.next.time);
            if pair.tide_is_coming_in() {
                format!(
                    "The tide will go up {:.0} centimeters until High Tide {}",
                    change.get::<centimeter>().abs(),
                    human_time
                )
            } else {
                format!(
                    "The tide will go down {:.0} centimeters until Low Tide {}",
                    change.get::<centimeter>().abs(),
                    human_time
                )
            }
        } else {
            "Can't calculate current tide level".to_string()
        }
    }

    fn station_lat(&self) -> f64 {
        self.station.coordinates.lat
    }

    fn station_lon(&self) -> f64 {
        self.station.coordinates.lon
    }
}

pub fn home_page(vm: HomePageViewModel) -> String {
    format!(
        r#"<html>
            <head>
                <title>What Tide Is It Right Now?!</title>
                <link REL=stylesheet href='style.css' />

                <link rel='stylesheet' href='https://unpkg.com/leaflet@1.5.1/dist/leaflet.css'
                  integrity='sha512-xwE/Az9zrjBIphAcBb3F6JVqxf46+CDLwfLMHloNu6KEQCAWi6HcDUbeOfBIptF7tcCzusKFjFw2yuvEpDL9wQ=='
                crossorigin=''/>
                <script src='https://unpkg.com/leaflet@1.5.1/dist/leaflet.js'
              integrity='sha512-GffPMF3RvMeYyc1LWMHtK8EbPv0iNZ8/oTtHPx9/cc2ILxQ+u905qIwdpULaqDkyBKgOaB57QTMg7ztg8Jm2Og=='
                crossorigin=''></script>

            </head>
            <body>
                <div class='container'>
                    <div class='content'>
                        <div class='time'>
                            {}
                        </div>
                        <div class='title'>
                            <h1>What Tide Is It Right Now?!</h1>
                        </div>
                        <div class='headline'>
                            <h2>{}</h2>
                        </div>
                        <div class='current'>
                        <p>{}</p>
                        </div>
                        <div class='detail'>
                            {}
                            <p>{}</p>
                        </div>
                        <div id='map'></div>
                    </div>
                </div>
                <script src='getlocation.js'></script>
                <script>
                  showMap({}, {})
                </script>
            </body>
        </html>"#,
        vm.current_time.format(TIME_FORMAT),
        vm.headline(),
        vm.current_level(),
        vm.detail(),
        vm.station_info(),
        vm.station_lat(),
        vm.station_lon(),
    )
}

pub fn not_found_page() -> String {
    "<html><h1>404</h1><p>Not found!<p></html>".to_string()
}
