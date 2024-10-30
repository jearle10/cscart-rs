use cscart_rs::prelude::*;

#[tokio::test]
async fn it_gets_cart_by_id() -> Result<(), Box<dyn std::error::Error>> {
    let api = test_utils::setup();

    let response = api.cart().get_by_id("11").await;
    assert!(response.is_ok());
    Ok(())
}

#[tokio::test]
async fn it_gets_all_carts() -> Result<(), Box<dyn std::error::Error>> {
    let api = test_utils::setup();

    let response: Result<cart_service::GetAllCartResponse, anyhow::Error> =
        dbg!(api.cart().get_all(GetAllOptions::default()).await);
    assert!(response.is_ok());
    Ok(())
}
