use cscart_rs::prelude::*;
use serde_json::json;
use std::time::{SystemTime, UNIX_EPOCH};

#[tokio::test]
async fn it_creates_a_vendor() -> Result<(), Box<dyn std::error::Error>> {
    let api = test_utils::setup();

    let company = format!("{:?}", SystemTime::now().duration_since(UNIX_EPOCH)?);
    let email = format!(
        "{:?}@gmail.com",
        SystemTime::now().duration_since(UNIX_EPOCH)?
    );

    let test_vendor = json!({
        "company" : company,
        "email" : email,
        "phone" : "12345678",
        "address" : "The Land of Oz",
        "city" : "London",
        "country" :"GB",
        "state" : "CO",

    });

    let response = dbg!(api.vendor().create(test_vendor).await);
    assert!(response.is_ok());
    Ok(())
}

#[tokio::test]
async fn it_gets_vendor_by_id() -> Result<(), Box<dyn std::error::Error>> {
    let api = test_utils::setup();

    let response = api.vendor().get_by_id("2").await;
    assert!(response.is_ok_and(|data| data.company_id == *"2"));
    Ok(())
}

#[tokio::test]
async fn it_updates_vendor_by_id() {
    let api = test_utils::setup();

    let vendor = json!({
        "name" : "Testing Testing 123"
    });

    let response = dbg!(api.vendor().update_by_id("6", vendor).await);
    assert!(response.is_ok())
}

#[tokio::test]
async fn it_gets_all_vendors() -> Result<(), Box<dyn std::error::Error>> {
    let api = test_utils::setup();

    let response = api.vendor().get_all(GetAllOptions::default()).await;
    assert!(response.is_ok_and(|data| !data.vendors.is_empty()));
    Ok(())
}
