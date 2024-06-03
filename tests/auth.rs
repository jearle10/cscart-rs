use cscart_rs::prelude::*;

#[tokio::test]
async fn it_get_an_auth_link() {
    let api = test_utils::setup();

    let data = serde_json::json!({
        "email": dbg!(std::env::var("CSCART_USERNAME").expect("No username found"))
    });

    let response = api.auth().create(data).await;

    match response {
        Ok(mut data) => {
            dbg!(&data);
            let auth_link = data.get_mut("link").cloned().unwrap().to_string();
            assert!(auth_link.len() > 0)
        }
        Err(e) => {
            println!("{}", e);
            assert!(false)
        }
    }
}
