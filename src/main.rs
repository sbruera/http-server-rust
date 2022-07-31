#![allow(dead_code, unused_imports, unused_parens)]

use server::Server;
use http::Request;
use http::Method;
use website_handler::WebsiteHandler;
use std::env;


mod server;
mod http;
mod website_handler;

fn main() {
    let default_path = format!("{}/public", env!("CARGO_MANIFEST_DIR")) ;
    let public_path = env::var("PUBLIC_PATH").unwrap_or(default_path);
    let server = Server::new(String::from("127.0.0.1:8080"));
    server.run(WebsiteHandler::new(public_path));
}

/*
GET /user?id=10 HTTP/1.1\r\n
HEADERS \r\n
BODY
*/
