use cscart_rs::prelude::*;

#[tokio::test]
async fn it_gets_all_call_requests() {
    let api = test_utils::setup();

    let response = api.call_request().get_all(GetAllOptions::default()).await;

    match response {
        Ok(mut data) => {
            let value = data.get_mut("call_requests").cloned().unwrap();
            let _call_requests: Vec<CallRequest> = serde_json::from_value(value).unwrap();
        }
        Err(e) => {
            println!("{}", e);
            assert!(false)
        }
    }
}

#[tokio::test]
async fn it_gets_a_call_request_by_id() {
    let api = test_utils::setup();

    let response = api.call_request().get_by_id("1").await;

    match response {
        Ok(data) => {
            let call_request: CallRequest = serde_json::from_value(data).unwrap();
            assert_eq!(call_request.request_id, Some("1".into()))
        }
        Err(e) => {
            dbg!(e);
            assert!(false)
        }
    }
}

#[tokio::test]
async fn it_creates_a_call_request() {
    let api = test_utils::setup();

    let data = serde_json::json!({
        "status": "non_answer",
        "phone": "+447487747296",
        "notes": "This is my note"
    });

    let response = api.call_request().create(data).await;

    match response {
        Ok(mut data) => {
            let request_id = data.get_mut("request_id").cloned().unwrap().is_number();
            assert!(request_id);
        }
        Err(e) => {
            dbg!(e);
            assert!(false)
        }
    }
}

#[tokio::test]
async fn it_deletes_a_call_request() {
    let api = test_utils::setup();

    let data = serde_json::json!({
        "status": "non_answer",
        "phone": "+447487747296",
        "notes": "This is my note"
    });

    let create_response = api.call_request().create(data).await;

    match create_response {
        Ok(mut data) => {
            let request_id = data.get_mut("request_id").cloned().unwrap().to_string();
            let delete_response = api.call_request().delete_by_id(&request_id).await;

            if let Ok(_) = delete_response {
                assert!(true)
            } else {
                assert!(false)
            }
        }
        Err(e) => {
            dbg!(e);
            assert!(false)
        }
    }
}
