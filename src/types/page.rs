use crate::utils::serde_utils::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Page {
    page_type: Option<String>,
    page: Option<String>,
    page_id: Option<String>,
    company_id: Option<String>,
    parent_id: Option<String>,
    id_path: Option<String>,
    status: Option<String>,
    timestamp: Option<String>,
    #[serde(deserialize_with = "deserialize_string_or_int_to_vec_i32")]
    usergroup_ids: Vec<i32>,
    localization: Option<String>,
    new_window: Option<String>,
    use_avail_period: Option<String>,
    avail_from_timestamp: Option<String>,
    avail_till_timestamp: Option<String>,
    lang_code: Option<String>,
    description: Option<String>,
    meta_keywords: Option<String>,
    meta_description: Option<String>,
    page_title: Option<String>,
    link: Option<String>,
    seo_name: Option<String>,
    seo_path: Option<String>,
    level: i32,
    tags: Option<String>,
    // Todo Option<string or int> to Vec<>
    // main_pair: Vec<i32>,
    spoiler: Option<String>,
    author: Option<String>,
}
