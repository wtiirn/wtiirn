use std::env;

use chrono::prelude::*;
use simple_server::{Method, Server, StatusCode};

mod compute;
mod model;

static PREDICTIONS_SRC: &'static str = include_str!("predictions.json");

fn main() {
    let host = env::var("WTIIRN_HOST").unwrap_or("127.0.0.1".to_string());
    let port = env::var("PORT").unwrap_or("7878".to_string());

    let predictions = parse_predictions(PREDICTIONS_SRC);

    println!("WTIIRN booting up!");
    let server = Server::new(move |request, mut response| {
        println!("Request received. {} {}", request.method(), request.uri());

        match (request.method(), request.uri().path()) {
            (&Method::GET, "/") => Ok(response.body(home_page(&predictions).as_bytes().to_vec())?),
            (_, _) => {
                response.status(StatusCode::NOT_FOUND);
                Ok(response.body(not_found_page().as_bytes().to_vec())?)
            }
        }
    });

    println!("Server listening on port: {}", port);
    server.listen(&host, &port);
}

fn home_page(predictions: &[model::TidePrediction]) -> String {
    let time = now_in_pst();
    let tide = compute::find_nearest_prediction(&predictions, time);
    let pair = compute::find_nearest_pair(&predictions, time);
    format!(
        "<html><h1>What Tide Is It Right Now?!</h1>
        <p>It is currently {}</p>
        <p>The nearest available tide prediction from Point Atkinson is:</p>
        <p>{:?}!</p>
        <p>{}</p>
        </html>",
        time.format(model::TIME_FORMAT),
        tide,
        pair
    )
}

fn now_in_pst() -> DateTime<FixedOffset> {
    let pst = FixedOffset::west(8 * 3600);
    Local::now().with_timezone(&pst)
}

fn not_found_page() -> String {
    "<html><h1>404</h1><p>Not found!<p></html>".to_string()
}

fn parse_predictions(src: &str) -> Vec<model::TidePrediction> {
    use serde_json;
    serde_json::from_str(src).expect("Failure to parse included predictions.json")
}

#[test]
fn test_parsing_predictions_file() {
    parse_predictions(PREDICTIONS_SRC);
}
