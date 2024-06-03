use cscart_rs::prelude::*;

#[tokio::test]
async fn it_gets_all_pages() {
    let api = test_utils::setup();

    let response = api.page().get_all(GetAllOptions::default()).await;

    match response {
        Ok(mut data) => {
            let value = data.get_mut("pages").cloned().unwrap();
            serde_json::from_value::<Vec<Page>>(value).unwrap();
            assert!(true)
        }
        Err(e) => {
            println!("{}", e);
            assert!(false)
        }
    }
}
