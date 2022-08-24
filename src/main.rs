mod models;
use std::{thread, time};


fn main() {
    println!("Hello, world!");
    let difficulty = 1;

    let mut bchain = models::blockchain::Blockchain::new(difficulty);

    let genesis_keys = models::key_gen::Wallet::generate_wallet_keys();
    let message = genesis_keys.sign_message("Next block".to_string());
    let mut message_q = models::message_handler::MessageQueue::new();
    message_q.add_message_to_q(&message);

    bchain.add_block(message_q);


    let ten_millis = time::Duration::from_millis(100);
    let now = time::Instant::now();


    // next assess the message q


}
