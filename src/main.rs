mod models;
use std::{thread, time};


fn main() {
    println!("Hello, world!");
    let difficulty = 1;

    let mut bchain = models::blockchain::Blockchain::new(difficulty);

    models::blockchain::Blockchain::add_block(&mut bchain, "message1".to_string());


    let ten_millis = time::Duration::from_millis(100);
    let now = time::Instant::now();

    //thread::sleep(ten_millis);

    models::blockchain::Blockchain::add_block(&mut bchain, "message2".to_string());
    models::blockchain::Blockchain::add_block(&mut bchain, "message3".to_string());


    let client = models::keyGen::Wallet::generate_wallet_keys();
    println!("This is the client {:?}", client);
    // Now sign a message with the keys
    let test = &client.sign_message("hello World".to_string());
    println!("{:?}",&client.sign_message("hello World".to_string()));
    let test2 = &client.sign_message("MY World".to_string());
    let mut mq = models::blockchain::MessageQueue::new();
    mq.add_message_to_q(test);
    mq.add_message_to_q(test2);
    mq.print_q();
    let test3 = &client.sign_message("Gus".to_string());
    mq.add_message_to_q(test3);
    mq.print_q();


}
