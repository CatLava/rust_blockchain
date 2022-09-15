use std::net::{TcpListener, TcpStream};
use std::io::{prelude::*, BufReader};
use std::sync::mpsc::SendError;
use http::request::Request;

use crate::models::key_gen::BlockchainMessage;
use crate::models::transaction_handler::Transaction;


pub fn run() {
    
    let listener = TcpListener::bind("127.0.0.1:1337").unwrap();
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        println!("connection established");
        handle_data(stream);
    }
    let mut state: bool = false;
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
}

pub fn post_transaction(uri: String, data: &BlockchainMessage) {
    let request = Request::post(uri)
        .body(data)
        .unwrap();
    request;
    
}

pub struct Server;

