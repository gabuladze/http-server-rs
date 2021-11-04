fn main() {
    let server = Server::new("http://127.0.0.1:8000".to_string());
    server.run();
}

struct Server {
    address: String,
}

impl Server {
    fn new(address: String) -> Self {
        Self { address }
    }

    fn run(self) {
        println!("Listening on {}", self.address);
    }
}