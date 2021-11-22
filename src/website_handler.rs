use super::http::{Method, Request, Response, StatusCode};
use crate::server::Handler;
use std::fs;
use std::{thread, time::Duration};

pub struct WebsiteHandler {
    public_path: String,
}

impl WebsiteHandler {
    pub fn new(public_path: String) -> Self {
        Self { public_path }
    }

    fn read_file(&self, file_path: &str) -> Option<String> {
        let full_path = format!("{}/{}", self.public_path, file_path);

        match fs::canonicalize(full_path) {
            Ok(path) => {
                if path.starts_with(&self.public_path) {
                    fs::read_to_string(path).ok()
                } else {
                    println!("Directory traversal attack attempted: {}", file_path);
                    None
                }
            }
            Err(_) => None,
        }
    }
}

impl Handler for WebsiteHandler {
    fn handle_request(&self, request: &Request) -> Response {
        match request.method() {
            Method::GET => match request.path() {
                "/" => Response::new(StatusCode::Ok, self.read_file("index.html")),
                "/test" => {
                    thread::sleep(Duration::from_secs(5));
                    Response::new(StatusCode::Ok, Some("<h1>Test</h1>".to_string()))
                }
                path => match self.read_file(path) {
                    Some(string) => Response::new(StatusCode::Ok, Some(string)),
                    None => Response::new(StatusCode::NotFound, None),
                },
            },
            _ => Response::new(StatusCode::NotFound, None),
        }
    }
}
