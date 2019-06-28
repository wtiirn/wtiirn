use chrono::prelude::*;
use uom::si::f64::*;
use uom::si::length::{kilometer, meter};

use crate::compute;
use crate::model::{Coordinates, TidePrediction};

static POINT_ATKINSON: Coordinates = Coordinates {
    lat: 49.3299,
    lon: -123.2650,
};

struct HomePageViewModel {
    current_time: DateTime<FixedOffset>,
    current_location: &Option<Coordinates>,
    prediction_pair: &Option<TidePredictionPair>,
    station: &Station
}

pub fn home_page(predictions: &[TidePrediction], current_location: &Option<Coordinates>) -> String {
    let time = now_in_pst();
    let pair = compute::find::nearest_pair(&predictions, time);

    let (headline, detail) = match pair {
        Some(p) => (p.headline(), p.detail()),
        _ => ("No Tide Information".into(), "".into()),
    };

    let distance = match current_location {
        None => Length::new::<meter>(0.0),
        Some(c) => compute::gcd::great_circle_distance(c, &POINT_ATKINSON),
    };

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
                            <p>{:?}</p>
                            <p>{}</p>
                        </div>
                    </div>
                </div>
                <script src='getlocation.js'></script>
            </body>
        </html>",
        headline,
        detail,
        current_location,
        distance.get::<kilometer>()
    )
}

fn now_in_pst() -> DateTime<FixedOffset> {
    let pst = FixedOffset::west(8 * 3600);
    Local::now().with_timezone(&pst)
}

pub fn not_found_page() -> String {
    "<html><h1>404</h1><p>Not found!<p></html>".to_string()
}
