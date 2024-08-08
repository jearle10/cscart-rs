use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Vendor {
    pub company_id: String,
    pub lang_code: Option<String>,
    pub email: Option<String>,
    pub company: Option<String>,
    pub timestamp: Option<String>,
    pub status: Option<String>,
    pub seo_name: Option<String>,
    pub seo_path: Option<String>,
    pub average_rating: Option<String>,
    // company_thread_ids: Vec<String>, // TODO
}
