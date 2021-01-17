use std::io::Write;
use std::net::{TcpListener, TcpStream};

fn connection_handler(mut stream: TcpStream) {
    let str = b"hello\n";
    stream.write(str).unwrap();
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        connection_handler(stream);
    }
}
