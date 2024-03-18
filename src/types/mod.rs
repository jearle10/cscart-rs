use serde::de::Deserializer;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Deserialize, Serialize)]
pub struct Product {
    product: String,
    category_ids: Vec<i32>,
    // #[serde(deserialize_with = "deserialize_string_or_int")]
    main_category: i32,
    price: String,
    #[serde(deserialize_with = "deserialize_string_or_int")]
    company_id: i32,
    status: String,
    #[serde(deserialize_with = "deserialize_string_or_int")]
    amount: i32,
    avail_since: String,
    // #[serde(deserialize_with = "deserialize_string_or_int")]
    box_height: Option<i32>, // Only present for /products/<id>
    #[serde(deserialize_with = "deserialize_string_or_int")]
    length: i32,
    // #[serde(deserialize_with = "deserialize_string_or_int")]
    box_width: Option<i32>, // Only present for /products/<id>
    details_layout: String,
    edp_shipping: String,
    exceptions_type: String,
    free_shipping: String,
    full_description: Option<String>,
    // image_pairs: //
    is_dep: Option<String>,
    lang_code: Option<String>,
    #[serde(deserialize_with = "deserialize_string_or_float")] // Update to float
    list_price: f32,
    list_qty_count: String,
    localisation: Option<String>,
    low_avail_limit: String,
    // main_pair
    max_qty: String,
    // #[serde(deserialize_with = "deserialize_string_or_int")]
    // max_items_in_box: Option<i32>, // Only present for /products/<id>
    meta_description: Option<String>,
    meta_keywords: Option<String>,
    // #[serde(deserialize_with = "deserialize_string_or_int")]
    // min_items_in_box: Option<i32>, // Only present for /products/<id>
    #[serde(deserialize_with = "deserialize_string_or_int")]
    min_qty: i32,
    options_type: Option<String>,
    out_of_stock_actions: String,
    page_title: Option<String>,
    point_price: Option<String>,
    popularity: Option<String>,
    product_code: String,
    // product_features
    #[serde(deserialize_with = "deserialize_string_or_int")]
    product_id: i32,
    promo_text: Option<String>,
    #[serde(deserialize_with = "deserialize_string_or_int")]
    qty_step: i32,
    return_period: String,
    sales_amount: Option<String>,
    search_words: Option<String>,
    seo_name: String,
    shared_product: Option<String>,
    shipping_freight: String,
    shipping_params: String,
    short_description: Option<String>,
    // tax_ids: Vec<String>, // Array or string
    timestamp: String,
    tracking: Option<String>,
    unlimited_download: String,
    updated_timestamp: String,
    // usergroup_ids: Vec<String>,
    #[serde(deserialize_with = "deserialize_string_or_float")]
    weight: f32,
    zero_price_action: String,
}

fn deserialize_string_or_int<'de, D>(deserializer: D) -> Result<i32, D::Error>
where
    D: Deserializer<'de>,
{
    let value = Value::deserialize(deserializer)?;
    let parse_result = match value {
        Value::String(string) => string.parse::<i64>().ok(),
        Value::Number(number) => number.as_i64(),
        _ => None,
    };

    match parse_result {
        Some(number) => Ok(number as i32),
        None => Err(serde::de::Error::custom("Could not parse")),
    }
}

fn deserialize_string_or_float<'de, D>(deserializer: D) -> Result<f32, D::Error>
where
    D: Deserializer<'de>,
{
    let value = Value::deserialize(deserializer)?;
    let parse_result = match value {
        Value::String(string) => string.parse::<f64>().ok(),
        Value::Number(number) => number.as_f64(),
        _ => None,
    };

    match parse_result {
        Some(number) => Ok(number as f32),
        None => Err(serde::de::Error::custom("Could not parse data")),
    }
}
