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
async fn it_creates_a_vendor( ){
    let api = setup();

    let test_vendor = json!({
        "company" : "e2e testin",
        "storefront" : "api",
        "email" : "e2etest@gmail.com",
        "phone" : "12345678",
        "address" : "The Land of Oz",
        "city" : "London",
        "country" :"GB",
        "state" : "CO",

    });

    let response = api
        .vendor()
        .create(test_vendor).await;


    match response {
        Ok(_) => assert!(true),
        Err(e) => {
            println!("{}", e);
            assert!(false)}
    }
}

#[tokio::test]
async fn it_gets_vendor_by_id(){

    let api = setup();

    let response = api
        .vendor()
        .get_by_id("2").await;

    match response {
        Ok(_) => assert!(true),
        Err(e) => {
            println!("{}", e);
            assert!(false)}
    }
}

#[tokio::test]
async fn it_updates_vendor_by_id(){

    let api = setup();

    let vendor = json!({
        "name" : "Testing Testing 123"
    });

    let response = api
        .vendor()
        .update_by_id("6", vendor).await;

    match response {
        Ok(_) => assert!(true),
        Err(e) => {
            println!("{}", e);
            assert!(false)}
    }
}

#[tokio::test]
async fn it_gets_all_vendors(){

    let api = setup();

    let response = api
        .vendor()
        .get_all().await;

    println!("{:?}", response);

    match response {
        Ok(_) => assert!(true),
        Err(_) => assert!(false)
    }
}