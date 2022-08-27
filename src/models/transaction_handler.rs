// This will handle transactions and balances
use std::collections::HashMap;
use serde::Serialize;

use super::blockchain::BlockchainLedger;

pub struct Balance {
    pub public_key: String,
    pub coins: u16,
}

#[derive(Debug,  Clone, Serialize, Hash)]
pub struct Transaction {
    pub receiver_public_key: String,
    pub amount_of_coins: u16,
    pub message: Option<String> // non-essential option for message in a transaction
}

impl Transaction {
    pub fn to_string(&self) -> String {
        let mut master_string = "".to_string();
        let num = self.amount_of_coins.to_string();
        master_string.push_str(&self.receiver_public_key.to_string());
        master_string.push_str(&":".to_string());
        master_string.push_str(&num);
        //master_string.push_str(&self.message.unwrap().to_string());
        return master_string;
    }
}


