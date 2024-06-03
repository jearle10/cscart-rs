use cscart_rs::prelude::*;

#[tokio::test]
async fn it_gets_all_languages() {
    let api = test_utils::setup();

    let response = api.language().get_all(GetAllOptions::default()).await;

    match response {
        Ok(mut data) => {
            let value = data.get_mut("languages").cloned().unwrap();
            serde_json::from_value::<Language>(value).unwrap();
            assert!(true)
        }
        Err(e) => {
            println!("{}", e);
            assert!(false)
        }
    }
}
