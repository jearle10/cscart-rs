use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize, Debug)]
pub struct Status {
    description: String,
    email_header: Option<String>,
    email_subj: Option<String>,
    is_default: Option<String>,
    lang_code: Option<String>,
    params: Option<Value>,
    status: Option<String>,
    status_id: Option<String>,
    #[serde(rename = "type")]
    status_type: Option<String>,
}
