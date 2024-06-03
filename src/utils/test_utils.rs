use crate::Client;
use dotenv::dotenv;

pub fn setup() -> Client {
    dotenv().ok(); // For local testing
    let api_key = std::env::var("CSCART_API_KEY").expect("No api key found");
    let username = std::env::var("CSCART_USERNAME").expect("No username found");
    let host = std::env::var("CSCART_HOST").expect("No host found");

    Client::new()
        .host(&host)
        .username(&username)
        .api_key(&api_key)
}
