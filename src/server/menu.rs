use std::net::TcpListener;
use std::io;
use rand::Error;

use crate::models::block::Block;
use crate::models::transaction_handler::Transaction;
use crate::models::{blockchain::*, self};
use crate::models::key_gen::Wallet;
use std::process;


pub fn main() {
    let mut Bstate = State::new();
    
    loop {
        Menu::show_menu();
        let mut entry = String::new();
        io::stdin().read_line(&mut entry).expect("need a number");
        println!("Selection, {:?}", entry.trim());
        let mat = match entry.trim() {
            "1" => Menu::Genesis,
            "2" => Menu::CreateClientKeys,
            "3" => Menu::ListClientKeys,
            "4" => Menu::ListLedger,
            "5" => Menu::RequestFunds,
            "6" => Menu::SendFunds,
            _ => Menu::Invalid,
        };
        // this should be a match statement
        // menu should map to functions with no output or same output
        if mat == Menu::Genesis {
            Bstate.gensis();
        } else if mat == Menu::CreateClientKeys  {
            Bstate.create_keys();
        } else if mat == Menu::ListClientKeys {
            Bstate.list_keys();
        } else if mat == Menu::ListLedger  {
            Bstate.list_ledger();
        } else if mat == Menu::RequestFunds {
            Bstate.request_funds()
        }  else if mat == Menu::SendFunds {
            Bstate.transfer_funds()
        }

    }

}

#[derive(Debug, Clone, PartialEq)]
pub enum Menu {
    Genesis,
    CreateClientKeys,
    ListClientKeys,
    RequestFunds,
    ListLedger,
    SendFunds,
    Invalid
}

impl Menu {
    pub fn show_menu() {
        println!("This is a simple blockchain: Options below");
        println!("1. Create blockchain, genesis node");
        println!("2. Create client keys");
        println!("3. List client keys");
        println!("4. List Ledger");
        println!("5. Request Funds");
        println!("6. Send Funds");
    }
}

#[derive(Debug, Clone)]
pub struct State {
    state: bool,
    blockchain: Blockchain,
    wallets: Vec<Wallet>,
}

impl State {
    pub fn new() -> Self {
        State {
            state: false,
            blockchain: Blockchain::new(1),
            wallets: vec![],
        }
    }

    pub fn gensis(&mut self) {
        if self.state == true {
            println!("Blockchain already running");
            return 
        }
        self.state = true;

    }

    pub fn create_keys(&mut self) {
        let mut keys  =  Wallet::generate_wallet_keys();
        self.wallets.push(keys);
        println!("keys added to usage, {:?}", self.wallets); 
    }

    pub fn list_keys(&mut self) {
        let mut count = 0;
        for key in self.wallets.iter() {
            println!("{}: {:?}", count,  key.public_key);
            count += 1;
        }
    }

    pub fn request_funds(&mut self) {
        let transaction = match self.construct_transaction() {
            Ok(v) => {self.blockchain.ledger.emit_funds(&v);
                                    println!("Funds emitted")},
            Err(e) => println!("error {e:?}"),
        };
        transaction;
    }

    pub fn transfer_funds(&mut self) {
        let transaction1 = self.construct_transaction().unwrap();
        let sender_key = self.select_key().unwrap();
        self.blockchain.ledger.transfer_funds(&transaction1, &sender_key)
    }

    fn select_key(&mut self) -> Option<String> {
        let mut count = 0;
        println!("Select Key:");
        for key in self.wallets.iter() {
            println!("{}: {:?}", count,  key.public_key);
            count += 1;
        }
        let mut entry = String::new();
        io::stdin().read_line(&mut entry).expect("Select a key");
        let entry: i32 =  entry.trim().parse::<i32>().unwrap_or_else(|err| {
            println!("invalid entry {}", err);
            return -1
        });

        if entry == -1 || entry as usize >= self.wallets.len() {
            return None;
        };
        
        let key = self.wallets.get(entry as usize);
        
        return Some(key.unwrap().public_key.to_string())

    }

    fn construct_transaction(&mut self) -> Result<Transaction, &'static str> {
        let pub_key = self.select_key();
        if pub_key == None {
            println!("Invalid key request");
            return Err("invalid key")
        }
        let transaction = Transaction {
            receiver_public_key: pub_key.unwrap(),
            amount_of_coins: 10,
            message: Some("hello".to_string()),
        };
        Ok(transaction)
    }

    pub fn list_ledger(&mut self) {
        println!("{:?}", self.blockchain.ledger)
    }



}

