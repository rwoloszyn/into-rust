fn main(){
    let server = HttpServer::new("127.0.0.0:8080");
    server.run()
}

struct HttpServer {
    addr: String,

}

impl HttpServer {
    fn new(addr: String) -> Self {
        HttpServer {
            addr
        }
    }

    fn run(self) {
        println!("Server is running on {}", self.addr);
    }
}