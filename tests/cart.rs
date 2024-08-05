use cart_service::GetAllCartResponse;
use cscart_rs::prelude::*;

#[tokio::test]
async fn it_gets_cart_by_id() -> Result<(), Box<dyn std::error::Error>> {
    let api = test_utils::setup();

    let response = api.cart().get_by_id("11").await?;
    let cart = serde_json::from_value::<Cart>(response);
    assert!(cart.is_ok());
    Ok(())
}

#[tokio::test]
async fn it_gets_all_carts() -> Result<(), Box<dyn std::error::Error>> {
    let api = test_utils::setup();

    let response = api.cart().get_all(GetAllOptions::default()).await?;
    let carts = serde_json::from_value::<GetAllCartResponse>(response);
    assert!(carts.is_ok());
    Ok(())
}
