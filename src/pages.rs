use chrono::prelude::*;
use uom::si::f64::*;
use uom::si::length::{kilometer, meter};

use crate::compute;
use crate::model::{Coordinates, TidePredictionPair};
use crate::stations::{Station, StationCatalogue};

static POINT_ATKINSON: Coordinates = Coordinates {
    lat: 49.3299,
    lon: -123.2650,
};

pub struct HomePageViewModel {
    current_time: DateTime<FixedOffset>,
    current_location: Option<Coordinates>,
    prediction_pair: Option<TidePredictionPair>,
    station: Station,
}

impl HomePageViewModel {
    /// Collect the information necessary for rendering the home page based on a request's
    /// location and the station catalogue that was loaded at startup.
    pub fn new(stn_catalogue: &StationCatalogue, coords: &Option<Coordinates>) -> Self {
        let current_time = now_in_pst();
        let station = stn_catalogue.find_near(&coords.unwrap_or_else(|| POINT_ATKINSON));
        let predictions = stn_catalogue.predictions_for_station(&station);
        let prediction_pair =
            predictions.and_then(|preds| compute::find::nearest_pair(preds, current_time));
        HomePageViewModel {
            current_time,
            current_location: *coords,
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
            Some(p) => p.detail(),
            _ => "".into(),
        }
    }

    fn distance_from_station(&self) -> Length {
        match self.current_location {
            None => Length::new::<meter>(0.0),
            Some(c) => compute::gcd::great_circle_distance(&c, &POINT_ATKINSON),
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
}

pub fn home_page(vm: HomePageViewModel) -> String {
    format!(
        "<html>
            <head>
                <title>What Tide Is It Right Now?!</title>
                <link REL=stylesheet href='style.css' />
            </head>
            <body>
                <div class='container'>
                    <div class='content'>
                        <div class='title'>
                            <h1>What Tide Is It Right Now?!</h1>
                        </div>
                        <div class='headline'>
                            <h2>{}</h2>
                        </div>
                        <div class='detail'>
                            <p>{}</p>
                            <p>{}</p>
                        </div>
                    </div>
                </div>
                <script src='getlocation.js'></script>
            </body>
        </html>",
        vm.headline(),
        vm.detail(),
        vm.station_info()
    )
}

fn now_in_pst() -> DateTime<FixedOffset> {
    let pst = FixedOffset::west(8 * 3600);
    Local::now().with_timezone(&pst)
}

pub fn not_found_page() -> String {
    "<html><h1>404</h1><p>Not found!<p></html>".to_string()
}
