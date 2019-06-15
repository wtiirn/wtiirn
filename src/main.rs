use std::convert::TryInto;
use std::env;

use simple_server::{Method, Server, StatusCode};

mod compute;
mod model;

mod pages;
mod stations;
static PREDICTIONS_SRC: &'static str = include_str!("predictions.json");

fn main() {
    let host = env::var("WTIIRN_HOST").unwrap_or("127.0.0.1".to_string());
    let port = env::var("PORT").unwrap_or("7878".to_string());

    let predictions = parse_predictions(PREDICTIONS_SRC);

    println!("WTIIRN booting up!");
    let server = Server::new(move |request, mut response| {
        println!("Request received. {} {}", request.method(), request.uri());
        let coords = request.uri().query().try_into().ok();

        match (request.method(), request.uri().path()) {
            (&Method::GET, "/") => {
                Ok(response.body(pages::home_page(&predictions, &coords).as_bytes().to_vec())?)
            }
            (_, _) => {
                response.status(StatusCode::NOT_FOUND);
                Ok(response.body(pages::not_found_page().as_bytes().to_vec())?)
            }
        }
    });

    println!("Server listening on port: {}", port);
    server.listen(&host, &port);
}

fn parse_predictions(src: &str) -> Vec<model::TidePrediction> {
    use serde_json;
    serde_json::from_str(src).expect("Failure to parse included predictions.json")
}

#[test]
fn test_parsing_predictions_file() {
    parse_predictions(PREDICTIONS_SRC);
}
