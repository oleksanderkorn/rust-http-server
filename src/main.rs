#![allow(dead_code, unused_imports, clippy::enum_variant_names)]

use server::Server;

mod http;
mod server;

fn main() {
    let server = Server::new("127.0.0.1:8080".to_string());
    server.run();
}
