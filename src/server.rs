use std::{io::Read, net::TcpListener};

pub struct HttpServer {
    addr: String
}

impl HttpServer {
    pub fn new(addr: String) -> Self {
        HttpServer {
            addr
        }
    }

    pub fn run(self) {
        println!("Server is running on {}", self.addr);
        let listner = TcpListener::bind(&self.addr).unwrap();

        loop {
            match listner.accept() {
                Ok((mut _socket, _)) => {
                    let mut buf = [0; 1024];
                    match _socket.read(&mut buf) {
                        Ok(_) => {
                            println!("Received a request: {}", String::from_utf8_lossy(&buf));
                        
                        },
                        Err(e) => {
                            println!("Failed to read from connection: {}", e);
                        }
                    }
                }
                Err(e) => {
                    println!("Failed to establish a connection: {}", e);
                }
            }
        }
    }
}