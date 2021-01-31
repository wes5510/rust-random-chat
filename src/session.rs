use std::{
    io::{prelude::*, BufReader, BufWriter},
    net::TcpStream,
    str,
};

const USER_PREFIX: &'static str = "user";

pub struct Session {
    ori_stream: TcpStream,
    wstream: BufWriter<TcpStream>,
    rstream: BufReader<TcpStream>,
}

impl Session {
    pub fn new(stream: TcpStream) -> Self {
        Session {
            ori_stream: stream.try_clone().unwrap(),
            rstream: BufReader::new(stream.try_clone().unwrap()),
            wstream: BufWriter::new(stream.try_clone().unwrap()),
        }
    }

    pub fn wating(&mut self) {
        let wating_msg = String::from("Wating...\n");
        self.write_stream(&wating_msg);
    }

    pub fn connect(&mut self, other_session_id: u64) {
        let text: String = format!(
            "Enjoy chatting with {}-{}!!!\n",
            USER_PREFIX, other_session_id
        );
        self.write_stream(&text);
    }

    pub fn read_stream(&mut self) -> Vec<u8> {
        let mut buf = String::new();

        self.rstream.read_line(&mut buf).unwrap();

        buf.into_bytes()
    }

    pub fn write_stream(&mut self, msg: &str) {
        self.wstream.write(msg.as_bytes()).unwrap();
        self.wstream.flush().unwrap();
    }

    pub fn copy(&mut self) -> Session {
        Session::new(self.ori_stream.try_clone().unwrap())
    }
}
