use crate::http::{
    response::{self, Response},
    ParseError, Request,
};
use std::{fmt::write, io::Read, io::Write, net::TcpListener};

pub trait Handler {
    fn handle_request(&mut self, request: &Request) -> Response;
    fn handle_bad_request(&mut self, error: &ParseError) -> Response {
        println!("Failed to parse a request: {}", error);
        Response::new(crate::http::StatusCode::BadRequest, None)
    }
}

pub struct Server {
    addr: String,
}

impl Server {
    pub fn new(addr: String) -> Self {
        Server { addr }
    }

    pub fn run(self, mut handler: impl Handler) {
        println!("Listening on {}", self.addr);
        let listener = TcpListener::bind(&self.addr).unwrap();
        loop {
            match listener.accept() {
                Ok((mut stream, _)) => {
                    let mut buffer = [0; 1024];
                    match stream.read(&mut buffer) {
                        Ok(_) => {
                            println!("Received a request: {}", String::from_utf8_lossy(&buffer));
                            let response = match Request::try_from(&buffer[..]) {
                                Ok(request) => handler.handle_request(&request),
                                Err(error) => handler.handle_bad_request(&error),
                            };
                            if let Err(error) = response.send(&mut stream) {
                                println!("Failed to send a response: {}", error);
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
