#[macro_use]
extern crate lazy_static;
extern crate config;

use config::Config;
use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream},
};

lazy_static! {
    static ref SETTINGS: Config = {
        let mut settings = Config::default();
        settings.merge(config::File::with_name("Settings")).unwrap();
        settings
    };
}

fn echo_stream(mut stream: TcpStream) {
    let mut buf = [0; 512];
    stream.read(&mut buf).unwrap();
    stream.write(&buf).unwrap();
}

fn main() {
    let host = SETTINGS.get_str("host").unwrap();
    let port = SETTINGS.get_int("port").unwrap();
    let listener = TcpListener::bind(format!("{}:{}", host, port)).unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        echo_stream(stream);
    }
}
