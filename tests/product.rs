use cscart_rs::prelude::*;
use serde_json::json;

#[tokio::test]
async fn it_creates_a_product() {
    let api = test_utils::setup();

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
    let api = test_utils::setup();

    let response = api.product().get_by_id("12").await;
    let product: Product = serde_json::from_value(dbg!(response.unwrap())).unwrap();
    assert_eq!(product.product_id, 12)
}

#[tokio::test]
async fn it_updates_product_by_id() {
    let api = test_utils::setup();

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
    let api = test_utils::setup();

    let response = api.product().get_all(GetAllOptions::default()).await;
    let data = response.unwrap().get("products").unwrap().to_owned();
    let products: Vec<Product> = serde_json::from_value(data).unwrap();
    assert_ne!(products.is_empty(), true);
}
