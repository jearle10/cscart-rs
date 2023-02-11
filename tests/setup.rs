#[test]
fn api_key_exists(){
    let api_key = std::env::var("CSCART_API_KEY")
        .expect("No api key found");
    assert_eq!(api_key.len() > 0 , true)
}