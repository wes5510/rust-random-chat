use std::{collections::HashMap, net::TcpStream};

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

    pub fn create_session(&mut self, stream: TcpStream) {
        self.seq = self.seq + 1;
        self.sessions.insert(self.seq, Session { stream });
    }
}

mod tests {
    #[test]
    fn create_session() {
        assert_eq!(1 + 1, 2);
    }
}
