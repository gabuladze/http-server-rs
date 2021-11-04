use server::Server;

fn main() {
    let server = Server::new("http://127.0.0.1:8000".to_string());
    server.run();
}

mod server {
    pub struct Server {
        address: String,
    }

    impl Server {
        pub fn new(address: String) -> Self {
            Self { address }
        }

        pub fn run(self) {
            println!("Listening on {}", self.address);
        }
    }
}

mod http {
    pub mod request {
        use super::method::Method;
        struct Request {
            method: Method,
            query_string: Option<String>,
            path: String,
        }
    }

    pub mod method {
        pub enum Method {
            GET,
            DELETE,
            POST,
            PUT,
            HEAD,
            CONNECT,
            OPTIONS,
            TRACE,
            PATCH,
        }
    }
}
