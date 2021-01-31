#[path = "session.rs"]
mod session;

use session::Session;
use std::{
    collections::{HashMap, VecDeque},
    net::TcpStream,
    str,
};

pub struct SessionManager {
    sessions: HashMap<u64, Session>,
    wating_queue: VecDeque<u64>,
    seq: u64,
}

impl SessionManager {
    pub fn new() -> Self {
        return SessionManager {
            sessions: HashMap::new(),
            wating_queue: VecDeque::new(),
            seq: 0,
        };
    }

    pub fn create_session(&mut self, stream: TcpStream) -> u64 {
        self.seq = self.seq + 1;
        let new_session = Session::new(stream);
        self.sessions.insert(self.seq, new_session);

        self.seq
    }

    pub fn find_wating_session_id(&mut self) -> Option<u64> {
        self.wating_queue.pop_back()
    }

    pub fn wating(&mut self, id: u64) {
        let session = self.sessions.get_mut(&id).unwrap();
        session.wating();
        self.wating_queue.push_front(id);
    }

    pub fn connect(&mut self, session_id: u64, other_session_id: u64) {
        let session = self.sessions.get_mut(&session_id).unwrap();
        session.connect(other_session_id);

        let other_session = self.sessions.get_mut(&other_session_id).unwrap();
        other_session.connect(session_id);

        self.connect_stream(session_id, other_session_id);
    }

    fn connect_stream(&mut self, session_id: u64, other_session_id: u64) {
        let session = self.sessions.get_mut(&session_id).unwrap();
        let mut copied_session = session.copy();
        let session_buf = copied_session.read_stream();

        let other_session = self.sessions.get_mut(&other_session_id).unwrap();
        let other_session_buf = other_session.read_stream();

        copied_session.write_stream(str::from_utf8(&other_session_buf).unwrap());
        other_session.write_stream(str::from_utf8(&session_buf).unwrap());
    }
}

mod tests {
    #[test]
    fn create_session() {
        assert_eq!(1 + 1, 2);
    }
}
