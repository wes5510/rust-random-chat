use std::{
    collections::{HashMap, VecDeque},
    io::Write,
    net::TcpStream,
};

const USER_PREFIX: &'static str = "user";

struct Session {
    stream: TcpStream,
}

impl Session {
    pub fn wating(&mut self) {
        self.stream.write("Wating...".as_bytes()).unwrap();
    }

    pub fn connect(&mut self, other_session_id: u64) {
        let text: String = format!(
            "Enjoy chatting with {}-{}!!!",
            USER_PREFIX, other_session_id
        );
        self.stream.write(text.as_bytes()).unwrap();
    }
}

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
        let new_session = Session {
            stream: stream.try_clone().unwrap(),
        };
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
    }
}

mod tests {
    #[test]
    fn create_session() {
        assert_eq!(1 + 1, 2);
    }
}
