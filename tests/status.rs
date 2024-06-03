use cscart_rs::prelude::*;
#[tokio::test]
async fn it_gets_all_statuses() {
    let api = test_utils::setup();

    let response = api.status().get_all(GetAllOptions::default()).await;

    match response {
        Ok(mut data) => {
            let value = data.get_mut("statuses").cloned().unwrap();
            serde_json::from_value::<Vec<Status>>(value).unwrap();
            assert!(true)
        }
        Err(e) => {
            println!("{}", e);
            assert!(false)
        }
    }
}
