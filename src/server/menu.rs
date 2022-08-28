use std::net::TcpListener;
use std::io;
use crate::models::block::Block;
use crate::models::blockchain::*;


pub fn main() {
    loop {
        println!("This is a simple blockchain: Options below");
        println!("1. Create blockchain, genesis node");
        println!("2. Create client keys");
        println!("3. List client keys");
        println!("4. Create transaction");
        let mut entry = String::new();
        io::stdin().read_line(&mut entry).expect("need a number");
        println!("number son, {:?}", entry);
        let mat = match entry.trim() {
            "1" => Menu::Genesis,
            "2" => Menu::Create_Client_Keys,
            "3" => Menu::List_Client_Keys,
            "4" => Menu::Create_Transaction,
            _ => Menu::Invalid,
        };
        let action = match mat {
            Menu::Genesis => Blockchain::new(1),
            _ => Blockchain::new(1),
        };
    }

}

#[derive(Debug, Clone)]
pub enum Menu {
    Genesis,
    Create_Client_Keys,
    List_Client_Keys,
    Create_Transaction,
    Invalid
}
