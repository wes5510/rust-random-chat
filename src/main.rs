#[macro_use]
extern crate lazy_static;
extern crate config;

mod chat_server;

use crate::chat_server::Server;
use config::Config;

lazy_static! {
    static ref SETTINGS: Config = {
        let mut settings = Config::default();
        settings.merge(config::File::with_name("Settings")).unwrap();
        settings
    };
}

fn main() {
    let server = Server::new(
        SETTINGS.get_str("host").unwrap(),
        SETTINGS.get_int("port").unwrap(),
    );

    server.run();
}
