use cscart_rs::prelude::*;
use serde_json::json;

#[tokio::test]
async fn it_creates_a_category() -> Result<(), Box<dyn std::error::Error>> {
    let api = test_utils::setup();

    let test_data = json!({
        "category" : "e2e testing",
        "company_id" : 1
    });

    let response = api.category().create(test_data).await;
    assert!(response.is_ok());
    assert_eq!(response?.category, "e2e testing".to_string());
    Ok(())
}

#[tokio::test]
async fn it_gets_category_by_id() -> Result<(), Box<dyn std::error::Error>> {
    let api = test_utils::setup();

    let response = api.category().get_by_id("210").await;
    assert!(response.is_ok());
    assert_eq!(response?.category_id, Some("210".into()));
    Ok(())
}

#[tokio::test]
async fn it_updates_category_by_id() {
    let api = test_utils::setup();

    let category = json!({
        "category" : "Comfort & Cruisers",
        "company_id" : 0,
        "status" : "A"
    });

    let categories = api.category().update_by_id("210", category).await;

    match categories {
        Ok(_) => assert!(true),
        Err(e) => {
            println!("{}", e);
            assert!(false)
        }
    }
}

#[tokio::test]
async fn it_gets_all_categories() {
    let api = test_utils::setup();

    let categories = api.category().get_all(GetAllOptions::default()).await;
    assert!(categories.is_ok());
    assert!(categories.unwrap().len() > 0);
}

#[tokio::test]
async fn it_gets_products_in_category() -> Result<(), Box<dyn std::error::Error>> {
    let api = test_utils::setup();
    let response = api.category().get_all_sub_entity("255", "products").await;
    dbg!(&response);
    assert!(response.is_ok());
    assert!(response?.products.len() > 0);
    Ok(())
}
