mod models;
use std::{thread, time};

use crate::models::transaction_handler::Transaction;


fn main() {
    println!("Hello, world!");
    let difficulty = 1;

    let mut bchain = models::blockchain::Blockchain::new(difficulty);

    let client_keys = models::key_gen::Wallet::generate_wallet_keys();
    let message = client_keys.sign_transaction("Next block".to_string());
    let mut message_q = models::message_handler::MessageQueue::new();
    message_q.add_message_to_q(&message);

    bchain.add_block(message_q);
    //println!("{:?}", bchain);

    bchain.ledger.emit_funds(Transaction {
        receiver_public_key: client_keys.public_key.to_string(),
        amount_of_coins: 8,
        message: Some("hello".to_string()),
    }
    );

    bchain.ledger.check_balance(client_keys.public_key.to_string());
    println!("{:?}", bchain.ledger);



    let ten_millis = time::Duration::from_millis(100);
    let now = time::Instant::now();


    // next assess the message q


}
