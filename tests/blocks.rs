use cscart_rs::prelude::*;

#[tokio::test]
async fn it_gets_all_blocks() -> Result<(), Box<dyn std::error::Error>> {
    let api = test_utils::setup();

    let response = api.block().get_all(GetAllOptions::default()).await;
    assert!(response.is_ok());
    assert!(!response.unwrap().is_empty());
    Ok(())
}
