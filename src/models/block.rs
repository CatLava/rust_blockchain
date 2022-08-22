use super::{blockchain::Blockchain, keyGen::BlockchainMessage};
use chrono::prelude::*;
use sha2::{Sha256, Digest};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionData {
    pub message: String
}
// struct for representing a block in teh blockchain
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Block {
    // index of current block
    pub index: u64,
    // time of current block creation
    pub timestamp: u64,
    //POW mechanism for the block
    pub proof_of_work: u64,
    // previous hash of prior or genesis block
    pub previous_hash: String,
    // current block hash
    pub hash: String,

    pub data: TransactionData,
}

impl Block{
    pub fn new(index: u64,  previous_hash: String, data_pass: TransactionData ) -> Self {
        // blck to be created
        let mut block = Block {
            index: index,
            timestamp: Utc::now().timestamp_millis() as u64,
            proof_of_work: u64::default(),
            previous_hash: previous_hash,
            hash: String::default(),
            data: data_pass,
        };

        return block
    }
    pub fn generate_block_hash(&self) -> String {
        let mut block_data = self.clone();

        block_data.hash = String::default();

        // convert block data to JSON
        let serialize_block_data = serde_json::to_string(&block_data).unwrap();

        // calculate sha 256 from data above

        let mut hasher = Sha256::new();

        hasher.update(serialize_block_data);

        let res = hasher.finalize();

        format!("{:x}", res)

        
    }

    pub fn mine (&mut self, blockchain: Blockchain) {
        loop {
            if !self.hash.starts_with(&"0".repeat(blockchain.difficulty)) {
                self.proof_of_work += 1;
                self.hash = self.generate_block_hash();
            } else {
                break
            }
        }
    }
}