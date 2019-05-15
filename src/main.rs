use chrono::prelude::*;
use simple_server::{Method, Server, StatusCode};

mod model;

fn main() {
    let host = "127.0.0.1";
    let port = "7878";

    println!("WTIIRN booting up!");
    let server = Server::new(|request, mut response| {
        println!("Request received. {} {}", request.method(), request.uri());

        match (request.method(), request.uri().path()) {
            (&Method::GET, "/") => Ok(response.body(home_page().as_bytes().to_vec())?),
            (_, _) => {
                response.status(StatusCode::NOT_FOUND);
                Ok(response.body(not_found_page().as_bytes().to_vec())?)
            }
        }
    });

    println!("Server listening on port: {}", port);
    server.listen(host, port);
}

fn home_page() -> String {
    let tide = model::TidePrediction {
        tide: 0.5,
        time: FixedOffset::west(8 * 3600)
            .ymd(2019, 05, 14)
            .and_hms(0, 0, 0),
    };
    format!(
        "<html><h1>What Tide Is It Right Now?!</h1><p>{:?}!</p></html>",
        tide
    )
}

fn not_found_page() -> String {
    "<html><h1>404</h1><p>Not found!<p></html>".to_string()
}
