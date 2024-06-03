use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Block {
    block_id: String,
    #[serde(rename = "type")]
    block_type: String,
    properties: serde_json::Value,
    company_id: String,
    lang_code: String,
    name: String,
    content: Option<serde_json::Value>,
}
