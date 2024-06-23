use cscart_rs::prelude::*;

#[tokio::test]
async fn it_gets_an_order_by_id() -> Result<(), Box<dyn std::error::Error>> {
    let api = test_utils::setup();
    let response = api.order().get_by_id("355").await;
    assert!(response.is_ok());
    assert_eq!(response.unwrap().order_id, 355);
    Ok(())
}

#[tokio::test]
async fn it_gets_all_orders() -> Result<(), Box<dyn std::error::Error>> {
    let api = test_utils::setup();
    let response = api.order().get_all(GetAllOptions::default()).await;
    dbg!(&response);
    // assert!(response.is_ok());
    let orders = response?.orders;
    dbg!(orders);
    Ok(())
}
#[tokio::test]
async fn creates_an_order() -> Result<(), Box<dyn std::error::Error>> {
    let api = test_utils::setup();

    let new_order = serde_json::json!({
        "user_id": 3,
        "payment_id": 13,
        "shipping_id": 3,
        "products" : {
            "1": {
            "product_id": "148",
            "amount": "1",
            "product_options": {}
            }
        }
    });

    let order = api.order().create(new_order).await;
    assert!(order.is_ok());
    Ok(())
}

#[tokio::test]
async fn updates_an_order() -> Result<(), Box<dyn std::error::Error>> {
    let api = test_utils::setup();

    let new_order = serde_json::json!({
        "user_id": 5,
        "payment_id": 13,
        "shipping_id": 3,
        "products" : {
            "1": {
            "product_id": "148",
            "amount": "1",
            "product_options": {}
            }
        }
    });

    let response = api.order().create(new_order).await;

    assert!(response.is_ok());

    let order_id = response.unwrap().order_id;
    dbg!(order_id);

    let update_order = serde_json::json!({
        "user_id": 5,
        "payment_id": 13,
        "shipping_id": 3,
        "products" : {
            "3": {
            "product_id": "148",
            "amount": "1",
            "product_options": {}
            }
        }
    });

    let update_response = api
        .order()
        .update_by_id(order_id.to_string(), update_order)
        .await?;

    assert!(update_response.is_object());
    Ok(())
}

#[tokio::test]
async fn deletes_an_order() -> Result<(), Box<dyn std::error::Error>> {
    let api = test_utils::setup();

    let new_order = serde_json::json!({
        "user_id": 5,
        "payment_id": 13,
        "shipping_id": 3,
        "products" : {
            "1": {
            "product_id": "148",
            "amount": "1",
            "product_options": {}
            }
        }
    });

    let delete_response = api.order().create(new_order).await;
    assert!(delete_response.is_ok());

    Ok(())
}
