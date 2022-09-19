mod models;
mod server;
use std::{thread, time};
#[macro_use] extern crate rocket;



use crate::models::transaction_handler::Transaction;


fn main() {
    println!("Hello, world!");
    server::menu::main();


    // next assess the message q


}
