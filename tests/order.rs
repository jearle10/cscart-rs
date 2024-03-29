use cscart_rs::Client;
use dotenv::dotenv;
use serde_json::json;
use serde_json::Value;

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
async fn it_gets_all_orders() {
    let api = setup();
    let response = api.order().get_all().await;

    match response {
        Ok(d) => {
            assert!(true)
        }
        Err(e) => {
            println!("{}", e);
            assert!(false)
        }
    }
}

#[tokio::test]
async fn creates_an_order() {
    let api = setup();

    let new_order = json!({
        "user_id": 3,
        "payment_id": 13,
        "shipping_id": 3,
        "products" : {
            "1": {
            "product_id": "148",
            "amount": "1",
            "product_options": {}
            }
        }
    });

    let response = api.order().create(new_order).await;

    println!("{:?}", &response);

    match response {
        Ok(d) => {
            assert!(true)
        }
        Err(e) => {
            println!("{}", e);
            assert!(false)
        }
    }
}

#[tokio::test]
async fn updates_an_order() {
    let api = setup();

    let new_order = json!({
        "user_id": 5,
        "payment_id": 13,
        "shipping_id": 3,
        "products" : {
            "1": {
            "product_id": "148",
            "amount": "1",
            "product_options": {}
            }
        }
    });

    let create_response = api.order().create(new_order).await.unwrap();

    let data = create_response.as_object().cloned().unwrap();
    let order_id = data.get("order_id").cloned().unwrap().to_string();
    println!("{:?}", &data);
    println!("{:?}", order_id);

    let update_order = json!({
        "user_id": 5,
        "payment_id": 13,
        "shipping_id": 3,
        "products" : {
            "3": {
            "product_id": "148",
            "amount": "1",
            "product_options": {}
            }
        }
    });

    let update_response = api
        .order()
        .update_by_id(order_id.as_str(), update_order)
        .await;

    match update_response {
        Ok(d) => {
            println!("{:#?}", d);
            assert!(true)
        }
        Err(e) => {
            println!("{}", e);
            assert!(false)
        }
    }
}

#[tokio::test]
async fn deletes_an_order() {
    let api = setup();

    let new_order = json!({
        "user_id": 5,
        "payment_id": 13,
        "shipping_id": 3,
        "products" : {
            "1": {
            "product_id": "148",
            "amount": "1",
            "product_options": {}
            }
        }
    });

    let create_response = api.order().create(new_order).await.unwrap();

    println!("{:?}", &create_response);

    let data = create_response.as_object().cloned().unwrap();
    let order_id = data.get("order_id").cloned().unwrap().to_string();

    let delete_response = api.order().delete_by_id(order_id.as_str()).await;

    println!("{:?}", delete_response);

    match delete_response {
        Ok(d) => {
            println!("{:#?}", d);
            assert!(true)
        }
        Err(e) => {
            println!("{}", e);
            assert!(false)
        }
    }
}
