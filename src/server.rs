use crate::http::{ParseError, Request, Response, StatusCode};
use std::io::Read;
use std::net::TcpListener;
use std::sync::Arc;
use std::thread;

pub trait Handler: Send + Sync {
    fn handle_request(&self, request: &Request) -> Response;

    fn handle_bad_request(&self, e: &ParseError) -> Response {
        println!("Failed to parse request: {}", e);
        Response::new(StatusCode::BadRequest, None)
    }
}

pub struct Server {
    address: String,
}

impl<'a> Server {
    pub fn new(address: String) -> Self {
        Self { address }
    }

    pub fn run(self, handler: Arc<dyn Handler>) {
        println!("Listening on {}", self.address);

        let listener = TcpListener::bind(&self.address).unwrap();
        for stream in listener.incoming() {
            match stream {
                Ok(mut stream) => {
                    let handler = Arc::clone(&handler);
                    thread::spawn(move || {
                        let mut buffer = [0; 1024];
                        match stream.read(&mut buffer) {
                            Ok(_) => {
                                let response = match Request::try_from(&buffer[..]) {
                                    Ok(request) => handler.handle_request(&request),
                                    Err(e) => handler.handle_bad_request(&e),
                                };
                                if let Err(e) = response.send(&mut stream) {
                                    println!("Failed to send the response: {}", e);
                                }
                            }
                            Err(e) => println!("Failed to read from stream: {}", e),
                        }
                    });
                }
                Err(e) => println!("Failed to establish connection: {}", e),
            }
        }
    }
}
