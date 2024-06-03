use cscart_rs::prelude::*;

#[tokio::test]
async fn it_gets_all_blocks() {
    let api = test_utils::setup();

    let response = api.block().get_all(GetAllOptions::default()).await;

    match response {
        Ok(data) => {
            let blocks: Vec<Block> = serde_json::from_value(data).unwrap();
            assert!(blocks.len() > 0);
        }
        Err(e) => {
            dbg!(e);
            assert!(false)
        }
    }
}
