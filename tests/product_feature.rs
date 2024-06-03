use cscart_rs::prelude::*;

#[tokio::test]
async fn it_gets_all_product_features() {
    let api = test_utils::setup();

    let response = api
        .product_feature()
        .get_all(GetAllOptions::default())
        .await;

    match response {
        Ok(mut data) => {
            let value = data.get_mut("features").cloned().unwrap();
            serde_json::from_value::<Vec<ProductFeature>>(value).unwrap();
            assert!(true)
        }
        Err(e) => {
            dbg!(e);
            assert!(false)
        }
    }
}
