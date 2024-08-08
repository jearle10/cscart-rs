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
    assert!(response.is_ok());
}

#[tokio::test]
async fn it_gets_product_by_id() -> Result<(), Box<dyn std::error::Error>> {
    let api = test_utils::setup();
    let product = api.product().get_by_id("12").await?;
    assert_eq!(product.product_id, 12);
    Ok(())
}

#[tokio::test]
async fn it_updates_product_by_id() -> Result<(), Box<dyn std::error::Error>> {
    let api = test_utils::setup();

    let product = json!({
        "name" : "e2e testing"
    });

    let response = dbg!(api.product().update_by_id("12", product).await);
    assert!(response.is_ok());
    Ok(())
}

#[tokio::test]
async fn it_gets_all_products() -> Result<(), Box<dyn std::error::Error>> {
    let api = test_utils::setup();

    let response = api.product().get_all(GetAllOptions::default()).await?;
    assert!(!response.products.is_empty());
    Ok(())
}
