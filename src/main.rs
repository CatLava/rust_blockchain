mod models;
mod server;
use std::{thread, time};


use crate::models::transaction_handler::Transaction;


fn main() {
    println!("Hello, world!");
    let difficulty = 1;

    let mut bchain = models::blockchain::Blockchain::new(difficulty);

    let client_keys = models::key_gen::Wallet::generate_wallet_keys();
    let t1 = Transaction {
        receiver_public_key: client_keys.public_key.to_string(),
        amount_of_coins: 8,
        message: Some("hello".to_string()),
    };
    let client_2 = models::key_gen::Wallet::generate_wallet_keys();
    let t2 = Transaction {
        receiver_public_key: client_2.public_key.to_string(),
        amount_of_coins: 1,
        message: Some("hello".to_string()),
    };
    bchain.ledger.emit_funds( &t1);

    let message = client_keys.sign_transaction(t2);
    let mut message_q = models::message_handler::MessageQueue::new();
    message_q.add_message_to_q(&message);

    bchain.add_block(message_q);
    //println!("{:?}", bchain);

    server::menu::main();


    // next assess the message q


}
