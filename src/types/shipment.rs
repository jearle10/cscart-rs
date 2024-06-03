use crate::utils::serde_utils::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Debug, Deserialize)]
pub struct Shipment {
    carrier: String,
    order_id: String,
    products: String,
    shipping: String,
    shipping_id: String,
    #[serde(deserialize_with = "deserialize_string_or_int_to_i32")]
    user_id: i32,
    tracking_number: String,
    comments: String,
    group_key: String,
    order_timestamp: String,
    status: String,
    order_status: String,
}
