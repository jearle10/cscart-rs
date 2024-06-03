use cscart_rs::prelude::*;

#[tokio::test]
async fn it_gets_all_user_groups() {
    let api = test_utils::setup();

    let response = api.user_group().get_all(GetAllOptions::default()).await;

    match response {
        Ok(data) => {
            serde_json::from_value::<UserGroup>(data).unwrap();
            assert!(true)
        }
        Err(e) => {
            dbg!(e);
            assert!(false)
        }
    }
}
