use dotenv::dotenv;

#[test]
fn api_credentials_exist() {
    dotenv().ok(); // For local testing
    let api_key = std::env::var("CSCART_API_KEY").expect("No api key found");

    let username = std::env::var("CSCART_USERNAME").expect("No username found");

    let host = std::env::var("CSCART_HOST").expect("No host found");

    assert!(!api_key.is_empty());
    assert!(!username.is_empty());
    assert!(!host.is_empty());
}
