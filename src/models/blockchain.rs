use chrono::prelude::*;
// internal module call
use super::block::{Block, TransactionData,  };


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
}