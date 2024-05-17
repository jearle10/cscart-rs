use cscart_rs::types::Product;
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
async fn it_creates_a_product() {
    let api = setup();

    let test_data = json!({
        "product" : "e2e testing",
        "category_ids" : [1 ,2],
        "main_category" : 210,
        "price" : 0,
        "company_id" : 1,
        "status" : "A"
    });

    let response = api.product().create(test_data).await;
    match response {
        Ok(_) => assert!(true),
        Err(e) => {
            println!("{}", e);
            assert!(false)
        }
    }
}

#[tokio::test]
async fn it_gets_product_by_id() {
    let api = setup();

    let response = api.product().get_by_id("12").await;
    let product: Product = serde_json::from_value(response.unwrap()).unwrap();
    assert_eq!(product.product_id, 12)
}

#[tokio::test]
async fn it_updates_product_by_id() {
    let api = setup();

    let product = json!({
        "name" : "e2e testing"
    });

    let response = api.product().update_by_id("210", product).await;

    match response {
        Ok(_) => assert!(true),
        Err(e) => {
            println!("{}", e);
            assert!(false)
        }
    }
}

#[tokio::test]
async fn it_gets_all_products() {
    let api = setup();

    let response = api.product().get_all().await;
    let data = response.unwrap().get("products").unwrap().to_owned();
    let products: Vec<Product> = serde_json::from_value(data).unwrap();
    assert_ne!(products.is_empty(), true);
}
