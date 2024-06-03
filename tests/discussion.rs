use cscart_rs::prelude::*;

#[tokio::test]
async fn it_gets_all_discussions() {
    let api = test_utils::setup();

    let response = api.discussion().get_all(GetAllOptions::default()).await;

    match response {
        Ok(mut data) => {
            let value = data.get_mut("discussions").cloned().unwrap();
            serde_json::from_value::<Vec<Discussion>>(value).unwrap();
            assert!(true)
        }
        Err(e) => {
            println!("{}", e);
            assert!(false)
        }
    }
}

// CRUD
