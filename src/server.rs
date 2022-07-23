use std::io::Read;
use std::net::TcpListener;

pub struct Server {
    addr: String,
}

impl Server {
    pub fn new(addr: String) -> Self {
        Server { addr }
    }

    pub fn run(&self) {
        println!("Listening on {}", &self.addr);

        let listener = TcpListener::bind(&self.addr).unwrap();

        loop {
            match listener.accept() {
                Ok(tup) => {
                    let (mut stream, _) = tup;

                    let mut buffer = [0; 1024];
                    match stream.read(&mut buffer){
                        Ok(_) => {
                            println!("Received a request: {}", String::from_utf8_lossy(&buffer));
                        },
                        Err(e) => println!("Failed to read from connection: {}", e)
                    }
                }
                Err(err) => {
                    println!("Failed to establish a connection: {}", err);
                    continue;
                }
            }
        }
    }
}