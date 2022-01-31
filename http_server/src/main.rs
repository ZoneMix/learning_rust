use std::fmt::Result;

use server::Server;
use http::Request;
use http::Method;
use std::io;

mod server;
mod http;

fn main() -> io::Result<()> {
    let server = Server::new("127.0.0.1:8080".to_string());
    server.run();
    Ok(())
}
