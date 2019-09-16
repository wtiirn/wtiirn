use std::convert::TryInto;
use std::env;
use wtiirn::{pages, stations};

use http::header::{self, HeaderName};
use simple_server::{Handler, Method, Request, ResponseBuilder, Server, StatusCode};

fn main() {
    let host = env::var("WTIIRN_HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port = env::var("PORT").unwrap_or_else(|_| "7878".to_string());

    let catalogue = stations::StationCatalogue::load();

    println!("WTIIRN booting up!");
    let server = Server::new(routes(catalogue));

    println!("Server listening on port: {}", port);
    server.listen(&host, &port);
}

fn routes(catalogue: stations::StationCatalogue) -> Handler {
    let forwarded_proto_header = HeaderName::from_static("x-forwarded-proto");

    Box::new(
        move |request: Request<Vec<u8>>, mut response: ResponseBuilder| {
            println!("Request received. {} {}", request.method(), request.uri());
            let coords = request.uri().query().try_into().ok();

            match (
                request.method(),
                request.uri().path(),
                request
                    .headers()
                    .get(&forwarded_proto_header)
                    .map(|x| x.len()),
            ) {
                (_, _, Some(4)) => {
                    response
                        .status(StatusCode::MOVED_PERMANENTLY)
                        .header(header::LOCATION, "https://whattideisitrightnow.com");
                    Ok(response.body(vec![])?)
                }
                (&Method::GET, "/", _) => Ok(response.body(
                    pages::home_page(pages::HomePageViewModel::new(&catalogue, &coords))
                        .as_bytes()
                        .to_vec(),
                )?),
                (_, _, _) => {
                    response.status(StatusCode::NOT_FOUND);
                    Ok(response.body(pages::not_found_page().as_bytes().to_vec())?)
                }
            }
        },
    )
}

#[cfg(test)]
mod test {
    use super::*;
    use simple_server::Response;
    #[test]
    fn it_should_redirect_in_http() {
        let routes = routes(stations::StationCatalogue::test());
        let request = Request::builder()
            .header("x-forwarded-proto", "http")
            .body(vec![])
            .unwrap();

        let response = routes(request, Response::builder()).unwrap();

        assert_eq!(response.status(), StatusCode::MOVED_PERMANENTLY);
    }

    #[test]
    fn it_should_not_redirect_in_https() {
        let routes = routes(stations::StationCatalogue::test());
        let request = Request::builder()
            .header("x-forwarded-proto", "https")
            .body(vec![])
            .unwrap();

        let response = routes(request, Response::builder()).unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }
}
