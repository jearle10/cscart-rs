use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Discussion {
    object_type: String,
    object_id: String,
    thread_id: String,
    name: String,
    message: String,
    rating_value: String,
    timestamp: String,
    status: String,
    post_id: String,
    user_id: String,
    ip_address: String,
    #[serde(rename = "type")]
    disussion_type: String,
    company_id: String,
}
