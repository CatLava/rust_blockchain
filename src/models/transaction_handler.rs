// This will handle transactions and balances
use std::collections::HashMap;
use super::blockchain::BlockchainLedger;

pub struct Balance {
    pub public_key: String,
    pub coins: u16,
}


pub struct Transaction {
    pub receiver_public_key: String,
    pub amount_of_coins: u16,
    pub message: Option<String> // non-essential option for message in a transaction
}


