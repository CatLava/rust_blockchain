use crate::models::blockchain::Blockchain;
use super::menu::State;
use super::application;

#[get("/")]
pub fn index() -> &'static str {
    application::main(1);
    "Hello, world!"
    
}


#[post("/blockchain_message")]
pub fn blockchain_message_receiver() {
    "made transaction";
}
#[rocket::main]
pub async fn main() -> Result<(), rocket::Error> {
    application::main(1);
    let _rocket = rocket::build()
        .mount("/", routes![index])
        .launch()
        .await?;
    Ok(())
}