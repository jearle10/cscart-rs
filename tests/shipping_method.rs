use cscart_rs::prelude::*;
#[tokio::test]
async fn it_gets_all_shipment_methods() {
    let api = test_utils::setup();

    let response = api
        .shipment_method()
        .get_all(GetAllOptions::default())
        .await;

    match response {
        Ok(mut data) => {
            let value = data.get_mut("shippings").cloned().unwrap();
            serde_json::from_value::<Vec<ShipmentMethod>>(value).unwrap();
            assert!(true)
        }
        Err(e) => {
            println!("{}", e);
            assert!(false)
        }
    }
}
