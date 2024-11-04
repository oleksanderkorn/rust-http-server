use crate::http::{
    response::{self, Response},
    Request,
};
use std::{fmt::write, io::Read, io::Write, net::TcpListener};

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
                    let mut buffer = [0; 1024];
                    match stream.read(&mut buffer) {
                        Ok(_) => {
                            println!("Received a request: {}", String::from_utf8_lossy(&buffer));
                            let response = match Request::try_from(&buffer[..]) {
                                Ok(request) => {
                                    dbg!(&request);
                                    Response::new(
                                        crate::http::StatusCode::Ok,
                                        Some("<h1>It Works!</h1>".to_string()),
                                    )
                                }
                                Err(error) => {
                                    println!("Failed to parse a request: {}", error);
                                    Response::new(crate::http::StatusCode::BadRequest, None)
                                }
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
