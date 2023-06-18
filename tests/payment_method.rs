use cscart_rs::Client;
use dotenv::dotenv;
use serde_json::json;

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
async fn it_creates_a_payment_method() {
    let api = setup();

    let test_payment_method = json!({
        "payment" : "e2e test payment method"
    });

    let response = api.payment_method().create(test_payment_method).await;

    match response {
        Ok(_) => assert!(true),
        Err(e) => {
            println!("{}", e);
            assert!(false)
        }
    }
}

#[tokio::test]
async fn it_gets_payment_method_by_id() {
    let api = setup();

    let response = api.payment_method().get_by_id("210").await;

    match response {
        Ok(_) => assert!(true),
        Err(e) => {
            println!("{}", e);
            assert!(false)
        }
    }
}

#[tokio::test]
async fn it_updates_payment_method_by_id() {
    let api = setup();

    let payment_method = json!({
        "payment" : "e2e test updated"
    });

    let response = api
        .payment_method()
        .update_by_id("210", payment_method)
        .await;

    match response {
        Ok(_) => assert!(true),
        Err(e) => {
            println!("{}", e);
            assert!(false)
        }
    }
}

#[tokio::test]
async fn it_gets_all_payment_methods() {
    let api = setup();

    let response = api.payment_method().get_all().await;

    println!("{:?}", response);

    match response {
        Ok(_) => assert!(true),
        Err(_) => assert!(false),
    }
}
