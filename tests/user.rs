use cscart_rs::prelude::*;
use uuid::Uuid;

#[tokio::test]
async fn it_creates_and_deletes_a_user() -> Result<(), Box<dyn std::error::Error>> {
    let api = test_utils::setup();

    let guuid = Uuid::new_v4();

    let test_user = serde_json::json!({
        "email" : format!("{}@test.com" , guuid),
        "user_type" : "C",
        "company_id" : "1",
        "status" : "A"
    });

    let create_response = api.user().create(test_user).await;

    assert!(create_response.is_ok());

    let user_id = create_response?.user_id.to_string();

    let delete_response = api.user().delete_by_id(&user_id).await;
    assert!(delete_response.is_ok());

    Ok(())
}

#[tokio::test]
async fn it_gets_user_by_id() -> Result<(), Box<dyn std::error::Error>> {
    let api = test_utils::setup();

    let response = api.user().get_by_id("1").await;
    assert!(response.is_ok_and(|data| data.user_id == Some("1".into())));
    Ok(())
}

#[tokio::test]
async fn it_updates_user_by_id() -> Result<(), Box<dyn std::error::Error>> {
    let api = test_utils::setup();

    let guuid = Uuid::new_v4();

    let user = serde_json::json!({
        "email" : format!("{}@test.com" , guuid),
        "user_type" : "A",
        "company_id" : 1,
        "status" :  "A",
        "password" : "qwrewerwerrfgerfgergerg"
    });

    let response = api.user().update_by_id("1", user).await;
    assert!(response.is_ok());
    Ok(())
}

#[tokio::test]
async fn it_gets_all_users() -> Result<(), Box<dyn std::error::Error>> {
    let api = test_utils::setup();

    let options = GetAllOptions {
        params: Some(vec![(("user_type".into(), "A".into()))]),
    };

    let response = api.user().get_all(options).await;
    assert!(response.is_ok_and(|data| data.users.len() > 0));
    Ok(())
}
