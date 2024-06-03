use cscart_rs::prelude::*;
#[tokio::test]
async fn it_gets_all_taxes() {
    let api = test_utils::setup();

    let response = api.tax().get_all(GetAllOptions::default()).await;

    match response {
        Ok(mut data) => {
            let value = data.get_mut("taxes").cloned().unwrap();
            serde_json::from_value::<Vec<Tax>>(value).unwrap();
            assert!(true)
        }
        Err(e) => {
            println!("{}", e);
            assert!(false)
        }
    }
}
