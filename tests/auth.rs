use auth_service::CreateAuthRequest;
use cscart_rs::prelude::*;

#[tokio::test]
async fn it_get_an_auth_link() {
    let api = test_utils::setup();

    let email = std::env::var("CSCART_USERNAME").expect("No username found");
    let request = CreateAuthRequest { email };

    let response = api.auth().create(request).await;
    dbg!(&response);
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
