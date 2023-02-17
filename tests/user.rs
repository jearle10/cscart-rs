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
async fn it_creates_a_user( ){
    let api = setup();

    let test_user = json!({
        "email" : "e2etest@gmail.com",
        "user_type" : "C",
        "company_id" : 1,
        "status" : "A"
    });

    let response = api
        .user()
        .create(test_user).await;

    match response {
        Ok(_) => assert!(true),
        Err(e) => {
            println!("{}", e);
            assert!(false)}
    }
}

#[tokio::test]
async fn it_gets_user_by_id(){

    let api = setup();

    let response = api
        .user()
        .get_by_id("210").await;

    match response {
        Ok(_) => assert!(true),
        Err(e) => {
            println!("{}", e);
            assert!(false)}
    }
}

#[tokio::test]
async fn it_updates_user_by_id(){

    let api = setup();

    let user = json!({
        "email" : "Comfort & Cruisers"
    });

    let response = api
        .user()
        .update_by_id("210", user).await;

    match response {
        Ok(_) => assert!(true),
        Err(e) => {
            println!("{}", e);
            assert!(false)}
    }
}

#[tokio::test]
async fn it_gets_all_users(){

    let api = setup();

    let response = api
        .user()
        .get_all().await;

    println!("{:?}", response);

    match response {
        Ok(_) => assert!(true),
        Err(_) => assert!(false)
    }
}