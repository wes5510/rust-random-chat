use std::{collections::HashMap, hint::unreachable_unchecked, io::Write, net::TcpStream};

const USER_PREFIX: &'static str = "user";

struct Session {
    stream: TcpStream,
}

pub struct SessionManager {
    sessions: HashMap<u64, Session>,
    seq: u64,
}

impl SessionManager {
    pub fn new() -> Self {
        return SessionManager {
            sessions: HashMap::new(),
            seq: 0,
        };
    }

    pub fn create_session(&mut self, mut stream: TcpStream) {
        self.seq = self.seq + 1;
        self.sessions.insert(
            self.seq,
            Session {
                stream: stream.try_clone().unwrap(),
            },
        );
        stream.write(self.say_hello().as_bytes()).unwrap();
    }

    fn say_hello(&self) -> String {
        format!("Hello, {}{}!\n", USER_PREFIX, self.seq)
    }
}

mod tests {
    #[test]
    fn create_session() {
        assert_eq!(1 + 1, 2);
    }
}
