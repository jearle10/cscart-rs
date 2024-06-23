use cscart_rs::prelude::*;

#[tokio::test]
async fn it_get_an_auth_link() {
    let api = test_utils::setup();

    let data = serde_json::json!({
        "email": dbg!(std::env::var("CSCART_USERNAME").expect("No username found"))
    });

    let response = api.auth().create(data).await;

    match response {
        Ok(data) => {
            dbg!(&data);
            assert!(data.link.len() > 0)
        }
        Err(e) => {
            dbg!(e);
            assert!(false);
        }
    }
}
