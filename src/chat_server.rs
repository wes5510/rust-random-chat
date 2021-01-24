#[path = "client_handler.rs"]
mod client_handler;

use std::net::TcpListener;

pub struct Server {
    host: String,
    port: i64,
}

impl Server {
    pub fn new(host: String, port: i64) -> Self {
        Self { host, port }
    }

    pub fn run(&self) {
        let listener = TcpListener::bind(format!("{}:{}", self.host, self.port)).unwrap();
        let mut sessoin_manager = client_handler::SessionManager::new();

        for stream in listener.incoming() {
            let mut stream = stream.unwrap();
            sessoin_manager.create_session(stream);
            /*
            let mut buf = [0; 512];

            stream.read(&mut buf).unwrap();
            stream.write(client_handler::echo(&buf)).unwrap();
            */
        }
    }
}
