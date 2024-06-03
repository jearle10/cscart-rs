
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Vendor {
    company_id: String,
    lang_code: Option<String>,
    email: Option<String>,
    company: Option<String>,
    timestamp: Option<String>,
    status: Option<String>,
    seo_name: Option<String>,
    seo_path: Option<String>,
    average_rating: Option<String>,
    // company_thread_ids: Vec<String>, // TODO
}
