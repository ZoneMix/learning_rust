use crate::http::Request;
use std::convert::TryFrom;
use std::convert::TryInto;
use std::net::{TcpListener, TcpStream};
use std::io::Read;

pub struct Server {
    addr: String,
}

impl Server {
    pub fn new(addr: String) -> Server {
        Server { addr }
    }

    pub fn run(&self) {
        println!("Listening on {}", self.addr);

        let listener = TcpListener::bind(&self.addr).unwrap();

        loop {
            match listener.accept() {
                Ok((mut stream, addr)) => { 
                    println!("Recieved connection from {:?}!", addr);
                    let mut buf = [0; 1024];
                    match stream.read(&mut buf) {
                        Ok(_) => {
                            println!("Recieved a request: {}", String::from_utf8_lossy(&buf));
                            match Request::try_from(&buf[..]) {
                                Ok(request) => {},
                                Err(e) => println!("Failed to parse request {}", e),
                            }
                            //let res: &Result<Request, _> = &buf[..].try_into();
                        },
                        Err(e) => println!("Failed to read from connection: {}", e),
                    }
                },
                Err(e) => println!("Failed to establish connection: {:?}!", e),
            }
        }
    }
}