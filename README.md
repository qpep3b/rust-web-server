# What do I want
```rust
use web_server::{App, Request, Response};
use std::fs;
use serde::Serialize;

#[derive(Serialize)]
struct Foo {
    bar: String
}

fn handle_root(req: Request) -> String {
    fs::read_to_string("templates/hello.html").unwrap())
}

fn handle_foo(req: Request) -> Foo {
    Foo {
        bar: String::from("Hello"),
    }
}

fn main() {
    let mut app = App::new("127.0.0.1", 7878);
    app.reqister_handler("GET", "/", handle_root);
    app.register_handler("GET", "/foo", handle_foo);
    app.run();
}
```

Response of `handle_root` should be `html` with `status_code 200`, response of `handle_foo` should be `json` of `{"bar": "Hello"}` with `status_code 200` (structure `Foo` will be automatically serialized)

# When to publish crate
Only when library could serve all of HTTP types (GET, POST, PUT, PATCH, DELETE)
