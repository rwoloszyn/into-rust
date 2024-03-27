
use http::Method;
use http::Request;
use server::HttpServer;

mod server;
mod http;


fn main(){
    let get = Method::GET;

    let server = HttpServer::new("127.0.0.0:8800".to_string());
    server.run()
}