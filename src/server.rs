use std::net::TcpListener;
use std::io::Read;    
    pub struct Server{
        addr: String
    }

impl Server{

    pub fn new(addr: String) -> Self{
        Self{
            addr
        }
    }

    pub fn run(self){ 
        println!("Listening on {}", &self.addr);

        let listener = TcpListener::bind(&self.addr).unwrap();
        loop{
            match listener.accept(){
                Ok((mut stream, _)) => {
                    let mut buf = [0; 1024];
                    stream.read(&mut buf);
                    println!("connected!")
                }
                Err(e) => println!("Failed to establish a connection: {}", e),
            }
        }
    }
}