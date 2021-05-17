use std::io::prelude::*;
use std::sync::Arc;
use std::net::{TcpListener, TcpStream};
use std::str;
use httparse::Request;
use serde::Serialize;

mod threadpool;
use threadpool::ThreadPool;

pub type RequestHandler = Box<dyn Fn(Request) -> HttpResponse + Send + Sync + 'static>;

pub struct App {
    tcp_listener: TcpListener,
    thread_pool: ThreadPool,
}

pub enum HttpResponse {
    HtmlResponse(String),
    JsonResponse(String),
}

impl App {
    pub fn new(ip_addr: &str, port: u32) -> App {
        let addr = format!("{}:{}", ip_addr, port);
        let tcp_listener = TcpListener::bind(addr).unwrap();
        let thread_pool = ThreadPool::new(4);

        App {
            tcp_listener,
            thread_pool,
        }
    }

    pub fn run(&self, f: RequestHandler) {
        let connection_handler = Arc::new(f);
        for stream in self.tcp_listener.incoming() {
            let stream_handler = Arc::clone(&connection_handler);
            let stream = stream.unwrap();
    
            self.thread_pool.execute(move || {
                Self::handle_connection(stream, stream_handler);
            });
        }
    }

    fn handle_connection(mut stream: TcpStream, connection_handler: Arc<RequestHandler>) {   
        let mut buffer = [0; 16];
        stream.read(&mut buffer).unwrap();
        let mut header = [httparse::EMPTY_HEADER; 16];
    
        let mut request = Request::new(&mut header);
        let _res = request.parse(&buffer).unwrap();
        let status_line = "HTTP/1.1 200 OK";
        let content = match connection_handler(request) {
            HttpResponse::HtmlResponse(content) => content,
            HttpResponse::JsonResponse(content) => content,
        };
    
        let response = format!(
            "{}\r\nContent-Length: {}\r\n\r\n{}",
            status_line,
            content.len(),
            content
        );
                
        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    }
}

