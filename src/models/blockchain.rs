use chrono::prelude::*;
// internal module call
use super::{block::{Block, TransactionData,  }, keyGen::BlockchainMessage};
use secp256k1::{Secp256k1, Message, Signing, All, SecretKey, PublicKey};


type Blocks = Vec<Block>;

#[derive(Debug, Clone)]
pub struct Blockchain {
    // first block added to the chain
    pub genesis_block: Block,
    // chain that stores individual blocks
    pub chain: Blocks,
    // defining POW difficulty setting
    pub difficulty: usize
}



impl Blockchain{
    pub fn new(difficulty: usize) -> Self {
        // Genesis block creation
        let mut genesis_block = Block {
            index: 0,
            timestamp: Utc::now().timestamp_millis() as u64,
            proof_of_work: u64::default(),
            previous_hash: String::default(),
            hash: String::default(),
            data: TransactionData{message: "Gensis Block message".to_owned()}
        };
        genesis_block.hash = genesis_block.generate_block_hash();
        println!("{:?}", genesis_block);
        // create chain from genisis block
        let mut chain = Vec::new();
        chain.push(genesis_block.clone());
        let blockchain = Blockchain {
            genesis_block,
            chain,
            difficulty
        };

        return blockchain
    }

    // Add a compute hash with a nonce
    // need to fix previous hash output
    // there is no state on this blockchain
    pub fn add_block(&mut self, data_pass: String) {
        let mut new_block = Block::new(
            self.chain.len() as u64,
            self.chain[&self.chain.len() - 1].hash.clone(),
            TransactionData{message: data_pass},
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
pub struct MessageQueue {
    pub queue: Vec<BlockchainMessage>
}

impl MessageQueue {
    pub fn new() -> Self {
        let mut q: Vec<BlockchainMessage> = Vec::new();
        let new = MessageQueue {queue: q};
        return new
    }

    pub fn add_message_to_q(&mut self, message: &BlockchainMessage) {
        let add_check = Blockchain::process_blockchain_message(message);
        match add_check {
            Ok(()) => self.queue.push(message.to_owned()),
            Err(e) => println!("will not process transaction: {e:?}")
        }
        
    }

    pub fn print_q(&mut self) {
        println!("q message");
        for m in self.queue.iter() {
            println!("THis is your message");
            println!("{:?}", m);
        }
    }
}
