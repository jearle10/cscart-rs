use cscart_rs::prelude::*;
use uuid::Uuid;

#[tokio::test]
async fn it_creates_and_deletes_a_user() {
    let api = test_utils::setup();

    let guuid = Uuid::new_v4();

    let test_user = serde_json::json!({
        "email" : format!("{}@test.com" , guuid),
        "user_type" : "C",
        "company_id" : "1",
        "status" : "A"
    });

    let create_response = api.user().create(test_user).await;

    let data = create_response.unwrap();
    let _id = data["user_id"].as_i64().unwrap();

    // let delete_response = api.user()
    //     .delete_by_id(id.to_string().as_str())
    //     .await;
}

#[tokio::test]
async fn it_gets_user_by_id() {
    let api = test_utils::setup();

    let response = api.user().get_by_id("1").await;

    match response {
        Ok(value) => {
            let user: User = serde_json::from_value(value).unwrap();
            assert_eq!(user.user_id, Some("1".to_string()));
        }
        Err(_e) => assert!(false),
    };
}

#[tokio::test]
async fn it_updates_user_by_id() {
    let api = test_utils::setup();

    let user = serde_json::json!({
        "email" : "jianearle93@googlemail.com",
        "user_type" : "A",
        "company_id" : 1,
        "status" :  "A",
        "password" : "Musashi1009!"
    });

    let response = api.user().update_by_id("1", user).await;

    match response {
        Ok(_) => assert!(true),
        Err(e) => {
            println!("{}", e);
            assert!(false)
        }
    }
}

#[tokio::test]
async fn it_gets_all_users() {
    let api = test_utils::setup();

    let options = GetAllOptions {
        params: Some(vec![(("user_type".into(), "A".into()))]),
    };

    let response = api.user().get_all(options).await;
    match response {
        Ok(mut value) => {
            let users_value = value.get_mut("users").cloned().unwrap();
            let users: Vec<User> = serde_json::from_value(users_value).unwrap();
            assert!(users.len() > 0)
        }
        Err(_) => assert!(false),
    }
}
