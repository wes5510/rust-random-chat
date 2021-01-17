#[macro_use]
extern crate lazy_static;
extern crate config;

use config::Config;
use std::io::Write;
use std::net::{TcpListener, TcpStream};

lazy_static! {
    static ref SETTINGS: Config = {
        let mut settings = Config::default();
        settings.merge(config::File::with_name("Settings")).unwrap();
        settings
    };
}

fn connection_handler(mut stream: TcpStream) {
    let str = b"hello\n";
    stream.write(str).unwrap();
}

fn main() {
    let host = SETTINGS.get_str("host").unwrap();
    let port = SETTINGS.get_int("port").unwrap();
    let listener = TcpListener::bind(format!("{}:{}", host, port)).unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        connection_handler(stream);
    }
}
