use cscart_rs::prelude::*;

#[tokio::test]
async fn it_gets_all_settings() {
    let api = test_utils::setup();

    let response = api.settings().get_all(GetAllOptions::default()).await;

    match response {
        Ok(mut data) => {
            let value = data.get_mut("settings").cloned().unwrap();
            serde_json::from_value::<Vec<Setting>>(value).unwrap();
            assert!(true)
        }
        Err(e) => {
            dbg!(e);
            assert!(false)
        }
    }
}
