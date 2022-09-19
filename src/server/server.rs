use std::net::{TcpListener, TcpStream};
use std::io::{prelude::*, BufReader};
use std::sync::mpsc::SendError;
use http::request::Request;

use crate::models::key_gen::BlockchainMessage;
use crate::models::transaction_handler::Transaction;
use super::menu::State;

pub fn run() {
    println!("running chain");
    let bchain = State::new();

    let listener = TcpListener::bind("127.0.0.1:1337").unwrap();
    println!("server listening");
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        println!("connection established");
        handle_data(stream);
    }
    // input from menu
}

fn handle_data(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();
    println!("Request: {:#?}", http_request);
    if http_request[0].contains(&"POST".to_string()) {
        println!("you made a post!!!");
    }

    let response = "HTTP/1.1 200 OK";

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap()
    
}

#[tokio::main]
pub async fn post_transaction(uri: String) { //, data: &BlockchainMessage
    println!("sending get");
    let client = reqwest::Client::new();
    let res = client.post("http://localhost:1337/blockchain").body("send this and this").send().await;

    println!("request sent {:?}", res);
    
    
}

pub struct Server;

