use cscart_rs::prelude::*;
use serde_json::json;

#[tokio::test]
async fn it_creates_a_vendor() {
    let api = test_utils::setup();

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

    let response = api.vendor().create(test_vendor).await;

    match response {
        Ok(_) => assert!(true),
        Err(e) => {
            println!("{}", e);
            assert!(false)
        }
    }
}

#[tokio::test]
async fn it_gets_vendor_by_id() {
    let api = test_utils::setup();

    let response = api.vendor().get_by_id("2").await;

    match response {
        Ok(_) => assert!(true),
        Err(e) => {
            println!("{}", e);
            assert!(false)
        }
    }
}

#[tokio::test]
async fn it_updates_vendor_by_id() {
    let api = test_utils::setup();

    let vendor = json!({
        "name" : "Testing Testing 123"
    });

    let response = api.vendor().update_by_id("6", vendor).await;

    match response {
        Ok(_) => assert!(true),
        Err(e) => {
            println!("{}", e);
            assert!(false)
        }
    }
}

#[tokio::test]
async fn it_gets_all_vendors() {
    let api = test_utils::setup();

    let response = api.vendor().get_all(GetAllOptions::default()).await;

    match response {
        Ok(mut value) => {
            dbg!(&value);
            let vendors_value = value.get_mut("vendors").cloned().unwrap();
            let users: Vec<Vendor> = serde_json::from_value(vendors_value).unwrap();
            assert!(users.len() > 0)
        }
        Err(_) => assert!(false),
    }
}
