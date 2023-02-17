use dotenv::dotenv;
use serde_json::json;
use cscart_rs::Client;

fn setup() -> Client {
    dotenv().ok(); // For local testing
    let api_key = std::env::var("CSCART_API_KEY")
        .expect("No api key found");

    let username = std::env::var("CSCART_USERNAME")
        .expect("No username found");

    let host = std::env::var("CSCART_HOST")
        .expect("No host found");

    Client::new()
        .host(&host)
        .username(&username)
        .api_key(&api_key)
}

#[tokio::test]
async fn it_creates_a_product( ){
}

#[tokio::test]
async fn it_gets_product_by_id(){

    let api = setup();

    let response = api
        .product()
        .get_by_id("210").await;

    match response {
        Ok(_) => assert!(true),
        Err(e) => {
            println!("{}", e);
            assert!(false)}
    }
}


#[tokio::test]
async fn it_updates_product_by_id(){

    let api = setup();

    let product = json!({
        "name" : "Comfort & Cruisers"
    });

    let response = api
        .product()
        .update_by_id("210", product).await;

    match response {
        Ok(_) => assert!(true),
        Err(e) => {
            println!("{}", e);
            assert!(false)}
    }
}

#[tokio::test]
async fn it_gets_all_products(){

    let api = setup();

    let response = api
        .product()
        .get_all().await;

    match response {
        Ok(_) => assert!(true),
        Err(_) => assert!(false)
    }
}