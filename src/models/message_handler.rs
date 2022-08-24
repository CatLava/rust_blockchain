// this will be message storage in between blocks
// essentially all transactions will queue here before a blockchain
use chrono::prelude::*;
use serde::{Serialize, Deserialize};
// internal module call
use super::blockchain::*;
use super::{block::{Block, TransactionData,  }, key_gen::BlockchainMessage};
use sha2::{Sha256, Digest};

#[derive(Debug, Clone, Hash)]
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
            println!("This is your message");
            println!("{:?}", m);
        }
    }
}

#[derive(Debug, Clone, Hash, Serialize, Deserialize)]
pub struct StringedBlockchainMessage {
    pub message: String,
    pub hashed_message: String,
    pub signed_hash: String,
    pub pub_key: String
}

#[derive(Debug, Clone, Hash, Serialize, Deserialize)]
pub struct StringedMessageQueue {
    pub stringed_queue: Vec<StringedBlockchainMessage>,
}

impl StringedMessageQueue {
    pub fn new(message_queue: MessageQueue) -> Self {
        let mut q: Vec<StringedBlockchainMessage> = Vec::new();
        for m in message_queue.queue.iter() {
            let mut q1 = StringedBlockchainMessage{
                message: m.message.to_string(),
                hashed_message: m.hashed_message.to_string(),
                signed_hash: m.signed_hash.to_string(),
                pub_key: m.pub_key.to_string(),
            };
            q.push(q1);
        }
        return StringedMessageQueue { stringed_queue: q }
    }

    pub fn hash_message_q(&mut self) -> String {
        let serialize_block_data = serde_json::to_string(&self).unwrap();

            // calculate sha 256 from data above

        let mut hasher = Sha256::new();

        hasher.update(serialize_block_data);

        let res = hasher.finalize();
        

        println!("Your hash {:?}", res);
        format!("{:x}", res)
    }
}