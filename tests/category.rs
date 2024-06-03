use cscart_rs::prelude::*;
use serde_json::json;


#[tokio::test]
async fn it_creates_a_category() {
    let api = test_utils::setup();

    let test_data = json!({
        "category" : "e2e testing",
        "company_id" : 1
    });

    let category = api.category().create(test_data).await;

    match category {
        Ok(mut data) => {
            assert!(data.get_mut("category_id").cloned().unwrap().is_number())
        }
        Err(e) => {
            println!("{}", e);
            assert!(false)
        }
    }
}

#[tokio::test]
async fn it_gets_category_by_id() {
    let api = test_utils::setup();

    let response = api.category().get_by_id("210").await;

    match response {
        Ok(data) => {
            let category: Category = serde_json::from_value(data).unwrap();
            assert_eq!(category.category_id, Some("210".into()))
        }
        Err(e) => {
            dbg!(e);
            assert!(false)
        }
    }
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

    match categories {
        Ok(mut data) => {
            dbg!(&data);
            let value = data.get_mut("categories").cloned().unwrap();
            let categories = serde_json::from_value::<Vec<Category>>(value).unwrap();
            assert!(categories.len() > 0)
        }
        Err(e) => {
            println!("{}", e);
            assert!(false)
        }
    }
}

#[tokio::test]
async fn it_gets_products_in_category() {
    let api = test_utils::setup();

    let categories = api.category().get_all_entity("210", "products").await;

    println!("{:#?}", categories);

    match categories {
        Ok(_) => assert!(true),
        Err(_) => assert!(false),
    }
}
