use crate::http::Request;
use std::{io::Read, net::TcpListener};

pub struct Server {
    addr: String,
}

impl Server {
    pub fn new(addr: String) -> Self {
        Server { addr }
    }

    pub fn run(self) {
        println!("Listening on {}", self.addr);
        let listener = TcpListener::bind(&self.addr).unwrap();
        loop {
            match listener.accept() {
                Ok((mut stream, _)) => {
                    let mut buffer = Vec::new();
                    match stream.read_to_end(&mut buffer) {
                        Ok(_) => {
                            println!("Received a request: {}", String::from_utf8_lossy(&buffer));
                            match Request::try_from(&buffer) {
                                Ok(request) => {
                                    println!("Received a request with path {}", request.path);
                                }
                                Err(error) => println!("Faiiled to parse a request: {}", error),
                            }
                        }
                        Err(error) => println!("Failed to establiish a connectiion: {}", error),
                    }
                }
                Err(error) => {
                    println!("Failed to establiish a connectiion: {}", error);
                }
            }
        }
    }
}
