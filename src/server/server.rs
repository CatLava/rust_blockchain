use std::net::TcpListener;


fn main() {
    
    let listener = TcpListener::bind("127.0.0.1:1337").unwrap();
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        println("connection established")
    }
    let mut state: bool = false;
    // input from menu
}

pub struct Server;

