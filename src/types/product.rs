use crate::utils::serde_utils::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Product {
    product: String,
    category_ids: Vec<i32>,
    main_category: i32,
    price: String,
    #[serde(deserialize_with = "deserialize_string_or_int_to_i32")]
    pub company_id: i32,
    pub status: String,
    #[serde(deserialize_with = "deserialize_string_or_int_to_i32")]
    amount: i32,
    avail_since: String,
    box_height: Option<i32>,
    #[serde(deserialize_with = "deserialize_string_or_int_to_i32")]
    length: i32,
    box_width: Option<i32>,
    details_layout: String,
    edp_shipping: String,
    exceptions_type: String,
    free_shipping: String,
    full_description: Option<String>,
    //  TODO image_pairs
    is_dep: Option<String>,
    lang_code: Option<String>,
    #[serde(deserialize_with = "deserialize_string_or_float_to_f32")]
    list_price: f32,
    list_qty_count: String,
    localisation: Option<String>,
    low_avail_limit: String,
    // TODO main_pair
    max_qty: String,
    max_items_in_box: Option<i32>,
    meta_description: Option<String>,
    meta_keywords: Option<String>,
    min_items_in_box: Option<i32>,
    #[serde(deserialize_with = "deserialize_string_or_int_to_i32")]
    min_qty: i32,
    options_type: Option<String>,
    out_of_stock_actions: String,
    page_title: Option<String>,
    point_price: Option<String>,
    popularity: Option<String>,
    product_code: String,
    // TODO product_features
    #[serde(deserialize_with = "deserialize_string_or_int_to_i32")]
    pub product_id: i32,
    pub promo_text: Option<String>,
    #[serde(deserialize_with = "deserialize_string_or_int_to_i32")]
    qty_step: i32,
    return_period: String,
    sales_amount: Option<String>,
    search_words: Option<String>,
    seo_name: String,
    shared_product: Option<String>,
    shipping_freight: String,
    shipping_params: String,
    short_description: Option<String>,
    #[serde(deserialize_with = "deserialize_string_or_int_to_vec_i32")]
    tax_ids: Vec<i32>,
    timestamp: String,
    tracking: Option<String>,
    unlimited_download: String,
    updated_timestamp: String,
    #[serde(deserialize_with = "deserialize_string_or_int_to_vec_i32")]
    usergroup_ids: Vec<i32>,
    #[serde(deserialize_with = "deserialize_string_or_float_to_f32")]
    pub weight: f32,
    pub zero_price_action: String,
}
