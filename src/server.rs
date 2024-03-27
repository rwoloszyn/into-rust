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
    }
}