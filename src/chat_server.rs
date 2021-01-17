use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream},
};

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

        for stream in listener.incoming() {
            let stream = stream.unwrap();

            Server::handle_client(stream);
        }
    }

    pub fn handle_client(mut stream: TcpStream) {
        let mut buf = [0; 512];
        stream.read(&mut buf).unwrap();
        stream.write(&buf).unwrap();
    }
}
