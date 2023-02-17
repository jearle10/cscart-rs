use dotenv::dotenv;
use serde_json::json;
use cscart_rs::Client;

fn setup() -> Client {
    dotenv().ok(); // For local testing
    let api_key = std::env::var("CSCART_API_KEY")
        .expect("No api key found");

    let username = std::env::var("CSCART_USERNAME")
        .expect("No username found");

    let host = std::env::var("CSCART_HOST")
        .expect("No host found");

    Client::new()
        .host(&host)
        .username(&username)
        .api_key(&api_key)
}


#[tokio::test]
async fn it_creates_a_category( ){
    let api = setup();

    let test_data = json!({
        "category" : "e2e testing",
        "company_id" : 1
    });

    let categories = api
        .category()
        .create(test_data).await;

    match categories {
        Ok(_) => assert!(true),
        Err(e) => {
            println!("{}", e);
            assert!(false)}
    }
}

#[tokio::test]
async fn it_gets_category_by_id(){

    let api = setup();

    let categories = api
        .category()
        .get_by_id("210").await;

    match categories {
        Ok(_) => assert!(true),
        Err(e) => {
            println!("{}", e);
            assert!(false)}
    }
}


#[tokio::test]
async fn it_updates_category_by_id(){

    let api = setup();

    let category = json!({
        "category" : "Comfort & Cruisers",
        "company_id" : 0,
        "status" : "A"
    });

    let categories = api
        .category()
        .update_by_id("210", category).await;

    match categories {
        Ok(_) => assert!(true),
        Err(e) => {
            println!("{}", e);
            assert!(false)}
    }
}


#[tokio::test]
async fn it_gets_all_categories(){

    let api = setup();

    let categories = api
        .category()
        .get_all().await;

    match categories {
        Ok(_) => assert!(true),
        Err(_) => assert!(false)
    }
}


#[tokio::test]
async fn it_gets_products_in_category(){

    let api = setup();

    let categories = api
        .category()
        .get_all_entity("210" , "products").await;


    println!("{:#?}" , categories);

    match categories {
        Ok(_) => assert!(true),
        Err(_) => assert!(false)
    }
}