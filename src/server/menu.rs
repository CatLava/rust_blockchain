use std::net::TcpListener;
use std::io;
use rand::Error;

use crate::models::block::Block;
use crate::models::message_handler::MessageQueue;
use crate::models::transaction_handler::{Transaction, TransactionType};
use crate::models::{blockchain::*, self};
use crate::models::key_gen::{Wallet, BlockchainMessage};
use std::process;


pub fn main() {
    let mut Bstate = State::new();
    
    loop {
        Menu::show_menu();
        println!("Messages in queue: {:?}", &Bstate.messsages.queue.len());
        println!("Block height: {:?}", &Bstate.blockchain.chain.len());
        let mut entry = String::new();
        io::stdin().read_line(&mut entry).expect("need a number");
        println!("Selection, {:?}", entry.trim());
        let mat = Menu::convert_to_menu(entry);
        // this should be a match statement
        // menu should map to functions with no output or same output
        match mat {
            Menu::Genesis => Bstate.gensis(),
            Menu::CreateClientKeys => Bstate.create_keys(),
            Menu::ListClientKeys => Bstate.list_keys(),
            Menu::ListLedger => Bstate.list_ledger(),
            Menu::RequestFunds => Bstate.request_funds(),
            Menu::SendFunds => Bstate.transfer_funds(),
            Menu::MineBlock => Bstate.mine_block(),
            Menu::Invalid => println!("Invalid selection")
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
    MineBlock,
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
        println!("7. Mine Block");
    }

    pub fn convert_to_menu(entry: String)-> Menu {
        return match entry.trim() {
            "1" => Menu::Genesis,
            "2" => Menu::CreateClientKeys,
            "3" => Menu::ListClientKeys,
            "4" => Menu::ListLedger,
            "5" => Menu::RequestFunds,
            "6" => Menu::SendFunds,
            "7" => Menu::MineBlock,
            _ => Menu::Invalid,
        };
    }
}

#[derive(Debug, Clone)]
pub struct State {
    state: bool,
    blockchain: Blockchain,
    wallets: Vec<Wallet>,
    messsages: MessageQueue,
}

impl State {
    pub fn new() -> Self {
        State {
            state: false,
            blockchain: Blockchain::new(1),
            wallets: vec![],
            messsages: MessageQueue::new(),
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
            println!("{}: {:?}", count, key.public_key.to_string());
            count += 1;
        }
    }

    pub fn request_funds(&mut self) {
        let transaction = match self.construct_transaction(TransactionType::emit) {
            Ok(v) => {
                                    self.messsages.add_message_to_q(&v);
                                    self.messsages.print_q();
                                    println!("Funds emitted")},
            Err(e) => println!("error {e:?}"),
        };
        transaction;
    }

    pub fn transfer_funds(&mut self) {
        println!("who is sending?");
        let sender_key = self.select_key().unwrap().public_key.to_string();
        println!("which account to send to: ");
        let transaction1 = self.construct_transaction(TransactionType::transfer(sender_key)).unwrap();
        println!("Added message to queue");
        self.messsages.add_message_to_q(&transaction1);
    }

    fn select_key(&mut self) -> Option<&Wallet> {
        let mut count = 0;
        println!("Select Key:");
        for key in self.wallets.iter() {
            println!("{}: {:?}", count,  key.public_key.to_string());
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
        
        return Some(key.unwrap())

    }

    fn construct_transaction(&mut self, transaction_type: TransactionType) -> Result<BlockchainMessage, &'static str> {
        let wallet = self.select_key().unwrap();
        let pub_key = &wallet.public_key.to_string();
        if pub_key == "" {
            println!("Invalid key request");
            return Err("invalid key")
        }
        let transaction = Transaction {
            receiver_public_key: pub_key.to_owned(),
            sender_public_key: transaction_type,
            amount_of_coins: 10,
            message: Some("hello".to_string()),
        };
        let blockchain_message = wallet.sign_transaction(&transaction);
        Ok(blockchain_message)
    }

    pub fn mine_block(&mut self) {
        // need significant error handling here
        // Need much more Result handling
        self.blockchain.add_block(&self.messsages);
        self.messsages.clear_queue();
    }

    pub fn list_ledger(&mut self) {
        println!("{:?}", self.blockchain.ledger)
    }



}

