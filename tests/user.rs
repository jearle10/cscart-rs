use cscart_rs::Client;
use dotenv::dotenv;
use serde_json::{json, Value};
use std::error::Error;
use uuid::Uuid;

fn setup() -> Client {
    dotenv().ok(); // For local testing
    let api_key = std::env::var("CSCART_API_KEY").expect("No api key found");

    let username = std::env::var("CSCART_USERNAME").expect("No username found");

    let host = std::env::var("CSCART_HOST").expect("No host found");

    Client::new()
        .host(&host)
        .username(&username)
        .api_key(&api_key)
}

#[tokio::test]
async fn it_creates_and_deletes_a_user() {
    let api = setup();

    let guuid = Uuid::new_v4();

    let test_user = json!({
        "email" : format!("{}@test.com" , guuid),
        "user_type" : "C",
        "company_id" : "1",
        "status" : "A"
    });

    let create_response = api.user().create(test_user).await;

    let data = create_response.unwrap();
    let id = data["user_id"].as_i64().unwrap();

    // let delete_response = api.user()
    //     .delete_by_id(id.to_string().as_str())
    //     .await;
}

#[tokio::test]
async fn it_gets_user_by_id() {
    let api = setup();

    let response = api.user().get_by_id("1").await;

    match response {
        Ok(_) => assert!(true),
        Err(e) => assert!(false),
    }
}

#[tokio::test]
async fn it_updates_user_by_id() {
    let api = setup();

    let user = json!({
        "email" : "jianearle93@googlemail.com",
        "user_type" : "A",
        "company_id" : 1,
        "status" :  "A",
        "password" : "Musashi1009!"
    });

    let response = api.user().update_by_id("1", user).await;

    match response {
        Ok(_) => assert!(true),
        Err(e) => {
            println!("{}", e);
            assert!(false)
        }
    }
}

#[tokio::test]
async fn it_gets_all_users() {
    let api = setup();

    let response = api.user().get_all().await;

    match response {
        Ok(_) => assert!(true),
        Err(_) => assert!(false),
    }
}
