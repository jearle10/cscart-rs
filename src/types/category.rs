
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Category {
    pub category: String,
    pub company_id: String,
    pub status: String,
    pub age_limit: Option<String>,
    pub age_verification: Option<String>,
    pub age_warning_message: Option<String>,
    pub category_id: Option<String>,
    pub default_layout: Option<String>,
    pub description: Option<String>,
    pub id_path: Option<String>,
    pub lang_code: Option<String>,
    pub localisation: Option<String>,
    pub main_pair: Option<Vec<String>>,
    pub meta_description: Option<String>,
    pub meta_keywords: Option<String>,
    pub page_title: Option<String>,
    pub parent_age_limit: Option<String>,
    pub parent_age_verification: Option<String>,
    pub parent_id: Option<String>,
    pub position: Option<String>,
    pub product_columns: Option<String>,
    pub product_count: Option<String>,
    pub product_details_layout: Option<String>,
    pub selected_layouts: Option<Vec<String>>,
    pub seo_name: Option<String>,
    pub timestamp: Option<String>,
    // #[serde(deserialize_with = "deserialize_string_or_int_to_vec_i32")]
    // pub usergroup_ids: Vec<i32>, // TODO
}
