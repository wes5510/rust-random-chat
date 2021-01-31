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
        let mut session_manager = client_handler::SessionManager::new();

        for stream in listener.incoming() {
            let stream = stream.unwrap();
            let wating_session_id = session_manager.find_wating_session_id();

            let new_session_id = session_manager.create_session(stream);

            match wating_session_id {
                None => {
                    session_manager.wating(new_session_id);
                }
                Some(session_id) => {
                    println!("hello: {}", session_id);
                }
            }
        }
    }
}
