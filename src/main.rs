use simple_server::{Method, Server, StatusCode};

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
    "<html><h1>Hi!</h1><p>Hello Rust!</p></html>".to_string()
}

fn not_found_page() -> String {
    "<html><h1>404</h1><p>Not found!<p></html>".to_string()
}
