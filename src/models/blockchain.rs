use chrono::prelude::*;
use serde::Serialize;
// internal module call
use super::message_handler::*;
use super::key_gen::{Wallet, BlockchainMessage };
use super::{block::{Block, TransactionData,  }};
use secp256k1::{Secp256k1, Message, Signing, All, SecretKey, PublicKey};
use sha2::{Sha256, Digest};
use super::message_handler::MessageQueue;
use super::transaction_handler::{Balance, Transaction};
use std::borrow::BorrowMut;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;



type Blocks = Vec<Block>;

#[derive(Debug, Clone)]
pub struct Blockchain {
    // first block added to the chain
    pub genesis_block: Block,
    // chain that stores individual blocks
    pub chain: Blocks,
    // defining POW difficulty setting
    pub difficulty: usize,

    pub ledger: BlockchainLedger,
}



impl Blockchain{
    pub fn new(difficulty: usize) -> Self {
        // create keys for initial block
        let genesis_keys = Wallet::generate_wallet_keys();
        let message = genesis_keys.sign_transaction("gensis block".to_string());
        let mut message_q = MessageQueue::new();
        message_q.add_message_to_q(&message);

        // Create initial coins and balance for ledger
        let initial_coins = Balance {
            public_key: genesis_keys.public_key.to_string(),
            coins: 10,
        };
        let ledger = BlockchainLedger::new(initial_coins);
        // Genesis block creation
        let mut genesis_block = Block {
            index: 0,
            timestamp: Utc::now().timestamp_millis() as u64,
            proof_of_work: u64::default(),
            previous_hash: String::default(),
            hash: String::default(),
            data: StringedMessageQueue::new(message_q),
        };
        genesis_block.hash = genesis_block.generate_block_hash();
        println!("{:?}", genesis_block);
        // create chain from genisis block
        let mut chain = Vec::new();
        chain.push(genesis_block.clone());
        let blockchain = Blockchain {
            genesis_block,
            chain,
            difficulty,
            ledger
        };

        return blockchain
    }

    // Add a compute hash with a nonce
    // need to fix previous hash output
    // there is no state on this blockchain
    pub fn add_block(&mut self, data_pass: MessageQueue) {
        let mut new_block = Block::new(
            self.chain.len() as u64,
            self.chain[&self.chain.len() - 1].hash.clone(),
            StringedMessageQueue::new( data_pass),
        );

        new_block.mine(self.clone());
        self.chain.push(new_block.clone());
        println!("New block added to chain -> {:?}", new_block);
    }


    pub fn process_blockchain_message(message: &BlockchainMessage) -> Result<(), secp256k1::Error> {
        
        let secp: Secp256k1<All> = Secp256k1::new();
        let res = secp.verify_ecdsa(&message.hashed_message, &message.signed_hash, &message.pub_key);
        return res
    }  
}


#[derive(Debug, Clone)]
pub struct BlockchainLedger {
    pub ledger: HashMap<String, u16>
}


impl BlockchainLedger {
    pub fn new(balance: Balance) -> Self {
        return BlockchainLedger{ 
            ledger: HashMap::from([(balance.public_key, balance.coins)])
        }
    }

    pub fn check_balance(&mut self, pub_key: String) {
        if self.ledger.contains_key(&pub_key) {
            println!("Your balance {:?}", self.ledger.get(&pub_key));
        }
        else {
            println!("no match");
            println!("{:?}", self);
            println!("{:?}", &pub_key);
        }
    }

    pub fn emit_funds(&mut self, transaction: Transaction) {
        if self.ledger.contains_key(&transaction.receiver_public_key) {
            *self.ledger.get_mut(&transaction.receiver_public_key).unwrap() += transaction.amount_of_coins;
            
        } else {
            self.ledger.insert(transaction.receiver_public_key, transaction.amount_of_coins );
        }

    }
}


