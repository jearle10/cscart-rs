use serde::de::Deserializer;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Deserialize, Serialize)]
pub struct Product {
    pub product: String,
    pub category_ids: Vec<i32>,
    // #[serde(deserialize_with = "deserialize_string_or_int")]
    pub main_category: i32,
    pub price: String,
    #[serde(deserialize_with = "deserialize_string_or_int")]
    pub company_id: i32,
    pub status: String,
    #[serde(deserialize_with = "deserialize_string_or_int")]
    pub amount: i32,
    pub avail_since: String,
    // #[serde(deserialize_with = "deserialize_string_or_int")]
    pub box_height: Option<i32>, // Only present for /products/<id>
    #[serde(deserialize_with = "deserialize_string_or_int")]
    pub length: i32,
    // #[serde(deserialize_with = "deserialize_string_or_int")]
    pub box_width: Option<i32>, // Only present for /products/<id>
    pub details_layout: String,
    pub edp_shipping: String,
    pub exceptions_type: String,
    pub free_shipping: String,
    pub full_description: Option<String>,
    // image_pairs: //
    pub is_dep: Option<String>,
    pub lang_code: Option<String>,
    #[serde(deserialize_with = "deserialize_string_or_float")] // Update to float
    pub list_price: f32,
    pub list_qty_count: String,
    pub localisation: Option<String>,
    pub low_avail_limit: String,
    // main_pair
    pub max_qty: String,
    // #[serde(deserialize_with = "deserialize_string_or_int")]
    // max_items_in_box: Option<i32>, // Only present for /products/<id>
    pub meta_description: Option<String>,
    pub meta_keywords: Option<String>,
    // #[serde(deserialize_with = "deserialize_string_or_int")]
    // min_items_in_box: Option<i32>, // Only present for /products/<id>
    #[serde(deserialize_with = "deserialize_string_or_int")]
    pub min_qty: i32,
    pub options_type: Option<String>,
    pub out_of_stock_actions: String,
    pub page_title: Option<String>,
    pub point_price: Option<String>,
    pub popularity: Option<String>,
    pub product_code: String,
    // product_features
    #[serde(deserialize_with = "deserialize_string_or_int")]
    pub product_id: i32,
    pub promo_text: Option<String>,
    #[serde(deserialize_with = "deserialize_string_or_int")]
    pub qty_step: i32,
    pub return_period: String,
    pub sales_amount: Option<String>,
    pub search_words: Option<String>,
    pub seo_name: String,
    pub shared_product: Option<String>,
    pub shipping_freight: String,
    pub shipping_params: String,
    pub short_description: Option<String>,
    // tax_ids: Vec<String>, // Array or string
    pub timestamp: String,
    pub tracking: Option<String>,
    pub unlimited_download: String,
    pub updated_timestamp: String,
    // usergroup_ids: Vec<String>,
    #[serde(deserialize_with = "deserialize_string_or_float")]
    pub weight: f32,
    pub zero_price_action: String,
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
