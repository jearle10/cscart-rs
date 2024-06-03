use cscart_rs::prelude::*;

#[tokio::test]
async fn it_gets_all_product_options() {
    let api = test_utils::setup();

    let response = api.product_option().get_all(GetAllOptions::default()).await;

    match response {
        Ok(mut data) => {
            let value = data.get_mut("product_options").cloned().unwrap();
            serde_json::from_value::<Vec<product_option::ProductOption>>(value).unwrap();
            assert!(true)
        }
        Err(e) => {
            dbg!(e);
            assert!(false)
        }
    }
}
