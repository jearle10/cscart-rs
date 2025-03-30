use auth_service::CreateAuthRequest;
use cscart_rs::prelude::*;

// This test fails if a reset link has been request recently
#[tokio::test]
#[ignore]
async fn it_get_an_auth_link() {
    let api = test_utils::setup();

    let email = std::env::var("CSCART_USERNAME").expect("No username found");
    let request = CreateAuthRequest { email };

    let response = api.auth().create(request).await;
    dbg!(&response);
    match response {
        Ok(data) => {
            dbg!(&data);
            assert!(!data.link.is_empty())
        }
        Err(e) => {
            dbg!(e);
            assert!(false);
        }
    }
}
