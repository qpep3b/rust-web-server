use web_server::{App, HttpResponse};
use httparse::{Request};
use std::fs;
use serde::Serialize;
use serde_json;

#[derive(Serialize)]
struct Foo<'a> {
    bar: &'a str
}

fn handle_root() -> HttpResponse {
    HttpResponse::HtmlResponse(fs::read_to_string("templates/hello.html").unwrap())
}

fn handle_json() -> HttpResponse {
    let f = Foo {bar: "lorem ipsum"};
    HttpResponse::JsonResponse(
        serde_json::to_string(&f).unwrap()
    )
}

fn main() {
    let app = App::new("127.0.0.1", 7878);

    let h = Box::new(|request: Request| {
        match (request.method, request.path) {
            (Some("GET"), Some("/")) => handle_root(),
            (_, _) => handle_json(),
        }
    });
    app.run(h);
}