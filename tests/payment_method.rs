use cscart_rs::prelude::*;
#[tokio::test]
async fn it_creates_a_payment_method() {
    let api = test_utils::setup();

    let test_payment_method = serde_json::json!({
        "payment" : "e2e test payment method"
    });

    let response = api.payment_method().create(test_payment_method).await;

    match response {
        Ok(_) => assert!(true),
        Err(e) => {
            println!("{}", e);
            assert!(false)
        }
    }
}

#[tokio::test]
async fn it_gets_payment_method_by_id() {
    let api = test_utils::setup();

    let response = api.payment_method().get_by_id("210").await;

    match response {
        Ok(_) => assert!(true),
        Err(e) => {
            println!("{}", e);
            assert!(false)
        }
    }
}

#[tokio::test]
async fn it_updates_payment_method_by_id() {
    let api = test_utils::setup();

    let payment_method = serde_json::json!({
        "payment" : "e2e test updated"
    });

    let response = api
        .payment_method()
        .update_by_id("210", payment_method)
        .await;

    match response {
        Ok(_) => assert!(true),
        Err(e) => {
            println!("{}", e);
            assert!(false)
        }
    }
}

#[tokio::test]
async fn it_gets_all_payment_methods() {
    let api = test_utils::setup();

    let response = api.payment_method().get_all(GetAllOptions::default()).await;

    match response {
        Ok(mut data) => {
            let value = data.get_mut("payments").cloned().unwrap();
            serde_json::from_value::<Vec<Payment>>(value).unwrap();
            assert!(true)
        }
        Err(e) => {
            println!("{}", e);
            assert!(false)
        }
    }
}
