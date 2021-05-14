use web_server::{App};
use httparse::{Request};
use std::fs;

fn handle_root() -> (String, String) {
    (String::from("HTTP/1.1 200 OK"), fs::read_to_string("templates/hello.html").unwrap())
}

fn handle_404() -> (String, String) {
    (String::from("HTTP/1.1 404 NOT FOUND"), fs::read_to_string("templates/404.html").unwrap())
}

fn main() {
    let app = App::new("127.0.0.1", 7878);
    // app.run();
    // I want it to be
    let h = Box::new(|request: Request| {
        match (request.method, request.path) {
            (Some("GET"), Some("/")) => handle_root(),
            (_, _) => handle_404(),
        }
    });
    app.run(h);
}