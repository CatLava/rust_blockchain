use std::net::TcpListener;
use std::io;
use crate::models::block::Block;
use crate::models::blockchain::*;
use crate::models::key_gen::Wallet;


pub fn main() {
    let mut Bstate = State::new();
    
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
        if mat == Menu::Genesis {
            Bstate.gensis();
        }

    }

}

#[derive(Debug, Clone, PartialEq)]
pub enum Menu {
    Genesis,
    Create_Client_Keys,
    List_Client_Keys,
    Create_Transaction,
    Invalid
}

pub struct State {
    state: bool,
    blockchain: Option<Blockchain>,
    wallets: Option<Vec<Wallet>>,
}

impl State {
    pub fn new() -> Self {
        State {
            state: false,
            blockchain: None,
            wallets: None,
        }
    }

    pub fn gensis(&mut self) {
        if self.state == true {
            println!("Blockchain already running");
            return 
        }
        self.state = true;
        self.blockchain = Some(Blockchain::new(1));
    }
}

