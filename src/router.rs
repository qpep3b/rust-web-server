use std::collections::HashMap;
use httparse::Request;

type HandlerFunc = Box<dyn Fn(Request) -> String + Send + Sync + 'static>;

enum HttpType {
    GET,
    POST,
    PUT,
    PATCH,
    DELETE,
}

pub struct Router {
    // handler_map should be:
    // {
    //      <String>: {
    //          <HttpType>: <HandlerFunc>,
    //          ...
    //      },
    //      ...
    // }
    handler_map: HashMap<String, HandlerFunc>,
}

impl Router {
    pub fn new() -> Router {
        Router {
            handler_map: HashMap::new(),
        }
    }

    pub fn add_handler(mut &self, method: &str, location: &str, handler: HandlerFunc) {
        let map_key = Self::_build_map_key(method, location);
        self.handler_map.insert(method, location);
    }

    pub fn handle(&self, method: &str, location: &str, request: Request) -> String {
        let map_key = Self::_build_map_key(method, location);
        
        match self.handler_map.get(map_key) {
            Some(handler) => handler(),
            None => "NOT FOUND",
        }
    }

    fn _build_map_key(method: &str, location: &str) -> String {
        String::from(
            format!("{} {}", method, location)
        )
    }
}