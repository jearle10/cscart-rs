use cscart_rs::prelude::*;

#[tokio::test]
async fn it_gets_cart_by_id() {
    let api = test_utils::setup();

    let response = api.cart().get_by_id("210").await;

    match response {
        Ok(_) => assert!(true),

        Err(e) => {
            dbg!(e);
            assert!(false)
        }
    }
}

#[tokio::test]
async fn it_gets_all_carts() {
    let api = test_utils::setup();

    let response = api.cart().get_all(GetAllOptions::default()).await;
    match response {
        Ok(_) => assert!(true),
        Err(_) => assert!(false),
    }
}
